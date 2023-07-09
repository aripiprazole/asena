#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, token::Pub, Field, FieldMutability, Fields, Ident, Token, Visibility};

#[proc_macro_attribute]
#[allow(clippy::redundant_clone)]
pub fn hir_node(args: TokenStream, input: TokenStream) -> TokenStream {
    let struct_name: proc_macro2::TokenStream = args.into();
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let name = input.ident.clone();
    let kind_name = Ident::new(&format!("{}Kind", struct_name), Span::call_site());

    let dbg = match input.fields {
        Fields::Named(ref fields) => {
            let fields = fields.named.iter().cloned().map(|next| {
                let name = next.ident.clone();
                quote!(s.field(stringify!(#name), &self.#name);)
            });

            quote! {
                let mut s = f.debug_struct(stringify!(#name));
                #(#fields);*
                s.finish()
            }
        }
        Fields::Unnamed(ref fields) => {
            let fields = fields.unnamed.iter().cloned().enumerate().map(|(i, _)| {
                let i = syn::Index::from(i);

                quote!(s.field(&self.#i);)
            });

            quote! {
                let mut s = f.debug_tuple(stringify!(#name));
                #(#fields);*
                s.finish()
            }
        }
        Fields::Unit => quote!(f.debug_struct(stringify!(#name)).finish()),
    };

    TokenStream::from(quote! {
        #input

        impl From<#struct_name> for #name {
            fn from(node: #struct_name) -> Self {
                match node.kind {
                    #kind_name::#name(value) => value,
                    _ => panic!("Invalid conversion"),
                }
            }
        }

        impl From<#name> for #struct_name {
            fn from(node: #name) -> Self {
                #kind_name::#name(node).into()
            }
        }

        impl From<#name> for #kind_name {
            fn from(node: #name) -> Self {
                #kind_name::#name(node)
            }
        }

        impl asena_hir_leaf::HirDebug for #name {
            fn fmt(&self, db: &dyn asena_hir_leaf::HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #dbg
            }
        }

        impl asena_hir_leaf::HirNode for #name {
            type Id = <#struct_name as asena_hir_leaf::HirNode>::Id;
            type Visitor<'a, T> = <#struct_name as asena_hir_leaf::HirNode>::Visitor<'a, T>;

            fn hash_id(&self) -> Self::Id {
                <<#struct_name as asena_hir_leaf::HirNode>::Id>::of(fxhash::hash(self))
            }

            fn accept<O: Default>(&mut self, visitor: &mut Self::Visitor<'_, O>) -> O {
                todo!()
            }
        }
    })
}

#[allow(clippy::redundant_clone)]
#[proc_macro_attribute]
pub fn hir_kind(args: TokenStream, input: TokenStream) -> TokenStream {
    let struct_name: proc_macro2::TokenStream = args.into();
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    let name = input.ident.clone();

    let generate_patterns = input.variants.iter().cloned().fold(quote!(), |acc, next| {
        let ident = next.ident;
        let pattern = match next.fields {
            Fields::Named(_) => {
                ident.span().unwrap().error("Unsupported field variants");

                quote!()
            }
            Fields::Unnamed(_) => {
                quote!(#name::#ident(value) => {
                    fxhash::hash(&(std::any::TypeId::of::<Self>(), std::any::TypeId::of::<#struct_name>(), value.hash_id()))
                })
            }
            Fields::Unit => {
                quote!(#name::#ident => fxhash::hash(&(std::any::TypeId::of::<Self>(), std::any::TypeId::of::<#struct_name>(), ())))
            }
        };

        quote!(#acc #pattern,)
    });

    let fmt_patterns = input.variants.iter().cloned().fold(quote!(), |acc, next| {
        let ident = next.ident;
        let pattern = match next.fields {
            Fields::Named(_) => {
                ident.span().unwrap().error("Unsupported field variants");

                quote!()
            }
            Fields::Unnamed(_) => {
                quote!(#name::#ident(value) => value.fmt(db, f))
            }
            Fields::Unit => {
                quote!(#name::#ident => write!(f, "{}", stringify!(#ident)))
            }
        };

        quote!(#acc #pattern,)
    });

    let accept_patterns = input.variants.iter().cloned().fold(quote!(), |acc, next| {
        let ident = next.ident;
        let pattern = match next.fields {
            Fields::Named(_) => {
                ident.span().unwrap().error("Unsupported field variants");

                quote!()
            }
            Fields::Unnamed(_) => {
                quote!(#name::#ident(value) => todo!())
            }
            Fields::Unit => {
                quote!(#name::#ident => todo!())
            }
        };

        quote!(#acc #pattern,)
    });

    TokenStream::from(quote! {
        #input

        impl From<#struct_name> for #name {
            fn from(node: #struct_name) -> Self {
                node.kind
            }
        }

        impl From<#name> for #struct_name {
            fn from(node: #name) -> Self {
                #struct_name {
                    kind: node,
                    ..Default::default()
                }
            }
        }

        impl asena_hir_leaf::HirDebug for #name {
            fn fmt(&self, db: &dyn asena_hir_leaf::HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #fmt_patterns
                }
            }
        }

        impl asena_hir_leaf::HirNode for #name {
            type Id = <#struct_name as asena_hir_leaf::HirNode>::Id;
            type Visitor<'a, T> = <#struct_name as asena_hir_leaf::HirNode>::Visitor<'a, T>;

            fn hash_id(&self) -> Self::Id {
                <#struct_name as asena_hir_leaf::HirNode>::Id::of(match self {
                    #generate_patterns
                })
            }

            fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
                match self {
                    #accept_patterns
                }
            }
        }
    })
}

#[allow(clippy::redundant_clone)]
#[proc_macro_attribute]
pub fn hir_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let visitor: proc_macro2::TokenStream = args.into();
    let mut input = syn::parse_macro_input!(input as syn::ItemStruct);

    let name = input.ident.clone();

    let id_name = Ident::new(&format!("{}Id", name), name.span());

    let Fields::Named(ref mut fields) = input.fields else {
        panic!("HirStruct must be a named struct");
    };

    fields.named.push(Field {
        attrs: vec![],
        vis: Visibility::Public(Pub {
            span: Span::call_site(),
        }),
        mutability: FieldMutability::None,
        ident: Some(Ident::new("span", Span::call_site())),
        colon_token: Some(Token![:](Span::call_site())),
        ty: parse_quote!(asena_hir_leaf::HirLoc),
    });

    let instance_parameters = fields.named.clone().iter().fold(quote!(), |acc, next| {
        let name = next.ident.clone();
        quote!(#acc #name,)
    });

    let parameters = fields.named.clone().iter().fold(quote!(), |acc, next| {
        let name = next.ident.clone();
        let ty = next.ty.clone();
        quote!(#acc, #name: #ty)
    });

    fields.named.push(Field {
        attrs: vec![],
        vis: Visibility::Public(Pub {
            span: Span::call_site(),
        }),
        mutability: FieldMutability::None,
        ident: Some(Ident::new("id", Span::call_site())),
        colon_token: Some(Token![:](Span::call_site())),
        ty: parse_quote!(#id_name),
    });

    let intern_fn = camel_case_ident(&format!("intern{name}"));

    TokenStream::from(quote! {
        #input

        #[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum #id_name {
            Value(usize),

            #[doc(hidden)]
            #[default]
            __InternalToCreate,
        }

        impl #id_name {
            pub fn of(value: usize) -> Self {
                Self::Value(value)
            }
        }

        impl asena_hir_leaf::HirId for #id_name {
            type Node = #name;

            fn new(node: Self::Node) -> Self {
                node.id
            }
        }

        impl asena_hir_leaf::HirDebug for #name {
            fn fmt(&self, db: &dyn asena_hir_leaf::HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.kind.fmt(db, f)
            }
        }

        impl asena_hir_leaf::HirLocated for #name {
            fn location(&self) -> asena_hir_leaf::HirLoc {
                self.span.clone()
            }
        }

        impl asena_hir_leaf::HirNode for #name {
            type Id = #id_name;
            type Visitor<'a, T> = dyn #visitor<T>;

            fn hash_id(&self) -> Self::Id {
                self.kind.hash_id()
            }

            fn accept<O: Default>(&mut self, visitor: &mut Self::Visitor<'_, O>) -> O {
                self.kind.accept(visitor)
            }
        }

        impl #name {
            pub fn new(db: &dyn crate::database::HirBag #parameters) -> <Self as asena_hir_leaf::HirNode>::Id {
                db.#intern_fn(Self {
                    #instance_parameters
                    id: #id_name::__InternalToCreate,
                })
            }
        }
    })
}

#[allow(clippy::redundant_clone)]
#[proc_macro_attribute]
pub fn hir_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident.clone();

    TokenStream::from(quote! {
        #input

        impl asena_hir_leaf::HirDebug for #name {
            fn fmt(&self, db: &dyn asena_hir_leaf::HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
            }
        }
    })
}

#[proc_macro_attribute]
pub fn hir_id(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

fn camel_case_ident(s: &str) -> Ident {
    let name = s
        .replace("Hir", "") // workaround
        .chars()
        .enumerate()
        .flat_map(|(i, char)| {
            if char.is_uppercase() && i > 0 {
                vec!['_', char]
            } else {
                vec![char]
            }
        })
        .collect::<String>()
        .to_lowercase();
    Ident::new(&name, Span::call_site())
}
