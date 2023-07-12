#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, token::Pub, Field, FieldMutability, Fields, Ident, Token, Visibility};

#[proc_macro_attribute]
#[allow(clippy::redundant_clone)]
pub fn hir_node(args: TokenStream, input: TokenStream) -> TokenStream {
    let struct_name: proc_macro2::TokenStream = args.into();
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident.clone();
    let kind_name = Ident::new(&format!("{}Kind", struct_name), Span::call_site());

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

        impl crate::query::leaf::HirNode for #name {
            type Id = <#struct_name as crate::query::leaf::HirNode>::Id;
            type Visitor<'a, T> = <#struct_name as crate::query::leaf::HirNode>::Visitor<'a, T>;

            fn hash_id(&self) -> Self::Id {
                <<#struct_name as crate::query::leaf::HirNode>::Id>::of(fxhash::hash(self))
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
                    use crate::query::leaf::HirNode;

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

        impl crate::query::dbg::HirDebug for #name {
            type Database = dyn crate::database::HirBag;

            fn fmt(&self, db: std::sync::Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #fmt_patterns
                }
            }
        }

        impl crate::query::leaf::HirNode for #name {
            type Id = <#struct_name as crate::query::leaf::HirNode>::Id;
            type Visitor<'a, T> = <#struct_name as crate::query::leaf::HirNode>::Visitor<'a, T>;

            fn hash_id(&self) -> Self::Id {
                <#struct_name as crate::query::leaf::HirNode>::Id::of(match self {
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
        ty: parse_quote!(crate::query::leaf::HirLoc),
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
    let data_fn = camel_case_ident(&format!("{name}Data"));

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

        impl crate::query::dbg::HirDebug for #id_name {
            type Database = dyn crate::database::HirBag;

            fn fmt(&self, db: std::sync::Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                db.clone().#data_fn(*self).fmt(db.clone(), f)
            }
        }

        impl crate::query::leaf::HirId for #id_name {
            type Node = #name;

            fn new(node: Self::Node) -> Self {
                node.id
            }
        }

        impl crate::query::dbg::HirDebug for #name {
            type Database = dyn crate::database::HirBag;

            fn fmt(&self, db: std::sync::Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.kind.fmt(db.clone(), f)
            }
        }

        impl crate::query::leaf::HirLocated for #name {
            fn location(&self) -> crate::query::leaf::HirLoc {
                self.span.clone()
            }
        }

        impl crate::query::leaf::HirNode for #name {
            type Id = #id_name;
            type Visitor<'a, T> = dyn #visitor<T>;

            fn hash_id(&self) -> Self::Id {
                crate::query::leaf::HirNode::hash_id(&self.kind)
            }

            fn accept<O: Default>(&mut self, visitor: &mut Self::Visitor<'_, O>) -> O {
                self.kind.accept(visitor)
            }
        }

        impl crate::query::leaf::HirInterned for #name {
            type Id = #id_name;
            type Database = dyn crate::database::HirBag;

            fn interned(db: std::sync::Arc<Self::Database>, id: Self::Id) -> std::sync::Arc<Self> {
                db.clone().#data_fn(id)
            }
        }

        impl #name {
            pub fn new(db: std::sync::Arc<dyn crate::database::HirBag> #parameters) -> <Self as crate::query::leaf::HirNode>::Id {
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

    match input.data {
        syn::Data::Struct(ref data) => {
            let name = input.ident.clone();

            let dbg = match data.fields {
                Fields::Named(ref fields) => {
                    let fields = fields.named.iter().cloned().map(|next| {
                        let name = next.ident.clone();
                        quote!(s.field(stringify!(#name), &crate::query::hir_dbg!(db.clone(), self.#name));)
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

                        quote!(s.field(&crate::query::hir_dbg!(db.clone(), self.#i));)
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

                impl crate::query::leaf::HirInterned for #name {
                    type Id = #name;
                    type Database = dyn crate::database::HirBag;

                    fn interned(db: std::sync::Arc<Self::Database>, id: Self::Id) -> std::sync::Arc<Self> {
                        std::sync::Arc::new(id)
                    }
                }

                impl crate::query::dbg::HirDebug for #name {
                    type Database = dyn crate::database::HirBag;

                    fn fmt(&self, db: std::sync::Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        #dbg
                    }
                }
            })
        }
        syn::Data::Enum(ref enum_data) => {
            let fmt = enum_data.variants.iter().map(|variant| {
                let name = variant.ident.clone();
                match variant.fields {
                    Fields::Named(ref fields) => {
                        let decl_fields = fields.named.iter().cloned().map(|next| {
                            let name = next.ident.clone();
                            quote!(#name)
                        });

                        let fields = fields.named.iter().cloned().map(|next| {
                            let name = next.ident.clone();
                            quote!(s.field(stringify!(#name), &crate::query::dbg::hir_dbg!(db.clone(), self.#name));)
                        });

                        quote! {
                            Self::#name(#(#decl_fields),*) => {
                                let mut s = f.debug_struct(stringify!(#name));
                                #(#fields);*
                                s.finish()
                            }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let decl_fields =
                            fields.unnamed.iter().cloned().enumerate().map(|(i, _)| {
                                let i = syn::Ident::new(&format!("_{}", i), Span::call_site());

                                quote!(#i)
                            });

                        let fields = fields.unnamed.iter().cloned().enumerate().map(|(i, _)| {
                            let i = syn::Ident::new(&format!("_{}", i), Span::call_site());

                            quote!(s.field(&crate::query::hir_dbg!(db.clone(), #i));)
                        });

                        quote! {
                            Self::#name(#(#decl_fields),*) => {
                                let mut s = f.debug_tuple(stringify!(#name));
                                #(#fields);*
                                s.finish()
                            }
                        }
                    }
                    Fields::Unit => quote! {
                        Self::#name => f.debug_struct(stringify!(#name)).finish()
                    },
                }
            });

            let name = input.ident.clone();

            TokenStream::from(quote! {
                #input

                impl crate::query::leaf::HirInterned for #name {
                    type Id = #name;
                    type Database = dyn crate::database::HirBag;

                    fn interned(db: std::sync::Arc<Self::Database>, id: Self::Id) -> std::sync::Arc<Self> {
                        std::sync::Arc::new(id)
                    }
                }

                impl crate::query::dbg::HirDebug for #name {
                    type Database = dyn crate::database::HirBag;

                    fn fmt(&self, db: std::sync::Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#fmt),*
                        }
                    }
                }
            })
        }
        _ => {
            input
                .ident
                .span()
                .unwrap()
                .error("Why are you using Union for an IR?");

            TokenStream::from(quote! {
                #input
            })
        }
    }
}

#[proc_macro_derive(HirDebug, attributes(hir_interned))]
pub fn derive_hir_debug(_: TokenStream) -> TokenStream {
    TokenStream::new()
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
