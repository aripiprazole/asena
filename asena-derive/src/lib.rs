#![feature(box_patterns)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data::Struct, *};

#[proc_macro_derive(Leaf)]
pub fn derive_leaf(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let Struct(data) = input.data else {
        name
            .span()
            .unwrap()
            .error("The leaf should be a struct");

        return TokenStream::new()
    };

    if data.fields.len() != 1 {
        name.span()
            .unwrap()
            .error("The leaf should have no fields, because it should hold the original node");
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #name {
            pub fn new<T: Into<asena_leaf::ast::GreenTree>>(tree: T) -> Self {
                Self(tree.into())
            }

            pub fn unwrap(self) -> asena_leaf::ast::GreenTree {
                self.0
            }
        }

        impl asena_leaf::ast::Ast for #name {}

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl std::ops::Deref for #name {
            type Target = asena_leaf::ast::GreenTree;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
#[allow(clippy::redundant_clone)]
pub fn ast_leaf(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let name = input.sig.ident.clone();
    let cursor_name = Ident::new(&format!("find_{name}"), Span::call_site());
    let output = match input.sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ty) => quote!(#ty),
    };

    input.sig.output = parse(quote!(-> asena_leaf::ast::Cursor<#output>).into()).unwrap();

    let mut impl_fn_tokens = input.clone();
    impl_fn_tokens.sig.ident = Ident::new(&format!("_impl_{name}"), Span::call_site());

    let mut get_fn_tokens = input.clone();
    get_fn_tokens.sig.ident = Ident::new(&format!("{name}"), Span::call_site());
    get_fn_tokens.sig.output = parse(quote!(-> #output).into()).unwrap();
    get_fn_tokens.block = Box::new(parse(quote! {{self.#cursor_name().as_leaf()}}.into()).unwrap());

    let mut set_fn_tokens = input.clone();
    set_fn_tokens.sig.output = parse(quote!(-> ()).into()).unwrap();
    set_fn_tokens
        .sig
        .inputs
        .push(parse(quote!(value: #output).into()).unwrap());
    set_fn_tokens.sig.ident = Ident::new(&format!("set_{name}"), Span::call_site());
    set_fn_tokens.block = Box::new(parse(quote! {{todo!()}}.into()).unwrap());

    let mut find_fn_tokens = input.clone();
    find_fn_tokens.sig.ident = cursor_name;
    find_fn_tokens.block = Box::new(parse(quote! {{todo!()}}.into()).unwrap());

    TokenStream::from(quote! {
        #get_fn_tokens
        #set_fn_tokens
        #find_fn_tokens
    })
}

#[proc_macro_attribute]
pub fn ast_of(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);

    let parameters = iter_leaf(&input).into_iter().fold(quote!(), |acc, next| {
        let name = Ident::new(&format!("_{}", next.name), Span::call_site());
        let ty = next.leaf_type;
        quote! { #acc #name: #ty, }
    });

    let _arguments = iter_leaf(&input).into_iter().fold(quote!(), |acc, next| {
        let name = next.name;
        let value = Ident::new(&format!("set_{}", name), Span::call_site());
        quote! { #acc _local_new.#value(#name.into()); }
    });

    input.items.push(
        syn::parse(
            quote! {
                pub fn of(#parameters) -> Self {
                    let _local_new = Self::default();
                    _local_new
                }
            }
            .into(),
        )
        .unwrap(),
    );

    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn ast_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);

    #[allow(clippy::redundant_clone)]
    let self_ty = input.self_ty.clone();

    let leaf_properties = iter_leaf(&input).into_iter();

    let debug_code = leaf_properties.fold(quote!(), |acc, next| {
        let name = next.name.to_string();
        let value = next.name;
        quote! { #acc debug_struct.field(#name, &self.#value()); }
    });

    TokenStream::from(quote! {
        #input

        impl std::fmt::Debug for #self_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_struct = f.debug_struct(stringify!(#self_ty));
                #debug_code
                debug_struct.finish()
            }
        }
    })
}

fn iter_leaf(input: &ItemImpl) -> Vec<NodeLeaf> {
    input
        .items
        .iter()
        .filter_map(|next| -> Option<NodeLeaf> {
            match next {
                ImplItem::Fn(item) => {
                    let node_leaf_attr = item.attrs.iter().find_map(|a| match a.meta {
                        Meta::Path(ref name) if name.is_ident("ast_leaf") => Some(name.clone()),
                        _ => None,
                    });

                    node_leaf_attr.as_ref()?;

                    let name = item.sig.ident.clone();
                    let leaf_type = match item.sig.output.clone() {
                        ReturnType::Type(_, value) => quote! { #value },
                        ReturnType::Default => quote! { () },
                    };
                    let parameters = item.sig.inputs.clone().into_iter().collect::<Vec<_>>();

                    if let None | Some(FnArg::Typed(..)) = parameters.first().cloned() {
                        name.span().unwrap().error(
                            "The first argument of a `ast_leaf` function should be the receiver",
                        );
                    }

                    Some(NodeLeaf { name, leaf_type })
                }
                _ => None,
            }
        })
        .collect()
}

#[allow(dead_code)]
#[derive(Clone)]
struct NodeLeaf {
    name: Ident,
    leaf_type: proc_macro2::TokenStream,
}
