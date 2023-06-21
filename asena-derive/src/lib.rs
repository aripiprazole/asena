#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, FnArg, Ident, ItemTrait, Meta, PatType, ReturnType, TraitItem, TraitItemFn,
};

#[proc_macro_attribute]
pub fn ast_node(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemTrait);

    let leaf_properties = input.items.iter().filter_map(|next| match next {
        TraitItem::Fn(item) => {
            let node_leaf_attr = item.attrs.iter().find_map(|a| match a.meta {
                Meta::Path(ref name) if name.is_ident("leaf") => Some(name.clone()),
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
                name.span()
                    .unwrap()
                    .error("The first argument of a `node_leaf` function should be the receiver");
            }

            let tree_param = match parameters.get(1) {
                Some(FnArg::Typed(fn_arg)) => Some(fn_arg.clone()),
                Some(FnArg::Receiver(..)) => None,
                None => None,
            };

            let Some(body) = item.default.clone() else {
                name.span()
                    .unwrap()
                    .error("All of `node_leaf` functions should have implementations");

                return None;
            };

            Some(NodeLeaf {
                name,
                leaf_type,
                tree_param,
                body,
            })
        }
        _ => None,
    });

    let debug_tokens = derive_debug_ast_node(input.clone(), leaf_properties.clone());
    let getters_tokens = create_getters(input.clone(), leaf_properties.clone());
    let struct_tokens = create_struct_item(input.clone(), leaf_properties.clone());
    let new_fn_tokens = create_new_fn_item(leaf_properties.clone());

    let node_name = input.ident;

    TokenStream::from(quote! {
        #struct_tokens
        #debug_tokens
        impl #node_name {
            #new_fn_tokens
            #getters_tokens
        }
    })
}

#[proc_macro_attribute]
pub fn leaf(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TraitItemFn);

    TokenStream::from(quote!(#input))
}

struct NodeLeaf {
    name: Ident,
    tree_param: Option<PatType>,
    leaf_type: proc_macro2::TokenStream,
    body: syn::Block,
}

fn derive_debug_ast_node<I>(item: ItemTrait, leaf_properties: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = NodeLeaf> + Clone,
{
    let node_name = item.ident;
    let name = node_name.to_string();

    let fields_tokens = leaf_properties
        .map(|next| {
            let name = next.name.to_string();
            let value = next.name;

            quote! {
                debug_struct.field(#name, &self.#value());
            }
        })
        .reduce(|acc, next| quote!(#acc #next));

    quote! {
        impl std::fmt::Debug for #node_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_struct = f.debug_struct(#name);
                #fields_tokens
                debug_struct.finish()
            }
        }
    }
}

fn create_getters<I>(item: ItemTrait, leaf_properties: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = NodeLeaf> + Clone,
{
    let node_name = item.ident;

    leaf_properties
        .map(|next| {
            let name = next.name;
            let leaf_type = next.leaf_type;
            let body = next.body;
            let impl_name = Ident::new(&format!("_impl_{name}"), Span::call_site());
            let tree_param = next
                .tree_param
                .map(|next| quote!(#next))
                .unwrap_or_else(|| quote!(_: asena_leaf::ast::GreenTree));

            quote! {
                pub fn #name(&self) -> std::borrow::Cow<'_, #leaf_type> {
                    match self {
                        #node_name::Impl(green_tree) => {
                            std::borrow::Cow::Owned(self.#impl_name(green_tree.clone()))
                        }
                        #node_name::Tree { #name, .. } => {
                            std::borrow::Cow::Borrowed(#name)
                        }
                    }
                }

                fn #impl_name(&self, #tree_param) -> #leaf_type
                    #body
            }
        })
        .fold(quote!(), |acc, next| quote!(#acc #next))
}

fn create_new_fn_item<I>(leaf_properties: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = NodeLeaf> + Clone,
{
    let parameters = leaf_properties
        .clone()
        .map(|next| {
            let name = next.name;
            let leaf_type = next.leaf_type;

            quote! { #name: #leaf_type, }
        })
        .fold(quote!(), |acc, next| quote!(#acc #next));

    let arguments = leaf_properties
        .map(|next| {
            let name = next.name;

            quote! { #name }
        })
        .fold(quote!(), |acc, next| quote!(#acc #next));

    quote! {
        pub fn new(#parameters) -> Self {
            Self::Tree { #arguments }
        }
    }
}

fn create_struct_item<I>(item: ItemTrait, leaf_properties: I) -> proc_macro2::TokenStream
where
    I: Iterator<Item = NodeLeaf>,
{
    let node_attrs = item
        .attrs
        .into_iter()
        .fold(quote!(), |acc, next| quote!(#acc #next));

    let node_name = item.ident;
    let properties = leaf_properties
        .map(|next| {
            let name = next.name;
            let leaf_type = next.leaf_type;

            quote! { #name: #leaf_type, }
        })
        .reduce(|acc, next| quote!(#acc, #next));

    let (_, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        #node_attrs
        #[derive(Clone)]
        pub enum #node_name #ty_generics #where_clause {
            Impl(asena_leaf::ast::GreenTree),
            Tree {
                #properties
            }
        }
    }
}
