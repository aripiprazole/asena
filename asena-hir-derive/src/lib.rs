#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, token::Pub, Field, FieldMutability, Fields, Ident, Token, Visibility};

#[proc_macro_attribute]
#[allow(clippy::redundant_clone)]
pub fn hir_node(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident.clone();
    let kind = Ident::new(&format!("{}Kind", args), Span::call_site());
    let node_name = format!("{args}").replace("Hir", "");
    let simplified_name = Ident::new(
        &name.to_string().replace(&node_name, "").replace("Hir", ""),
        Span::call_site(),
    );

    TokenStream::from(quote! {
        impl From<#name> for #kind {
            fn from(node: #name) -> Self {
                #kind::#simplified_name(node)
            }
        }

        #input
    })
}

#[allow(clippy::redundant_clone)]
#[proc_macro_attribute]
pub fn hir_kind(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    let name = input.ident.clone();
    let data = Ident::new(&format!("{}Data", args), Span::call_site());

    TokenStream::from(quote! {
        impl From<#name> for #data {
            fn from(kind: #name) -> Self {
                Self {
                    kind,
                    ..Self::default()
                }
            }
        }

        #input
    })
}

#[allow(clippy::redundant_clone)]
#[proc_macro_attribute]
pub fn hir_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemStruct);

    let name = input.ident.clone();

    let data_name = Ident::new(&format!("{name}Data"), name.span());

    let Fields::Named(ref mut fields) = input.fields else {
        panic!("HIR struct must be a named struct");
    };

    fields.named.push(Field {
        attrs: vec![],
        vis: Visibility::Public(Pub {
            span: Span::call_site(),
        }),
        mutability: FieldMutability::None,
        ident: Some(Ident::new("span", Span::call_site())),
        colon_token: Some(Token![:](Span::call_site())),
        ty: parse_quote!(crate::HirLoc),
    });

    input.ident = data_name.clone();

    TokenStream::from(quote! {
        #[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
        #input

        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
        pub struct #name(salsa::InternId);

        impl salsa::InternKey for #name {
            fn from_intern_id(id: salsa::InternId) -> Self {
                Self(id)
            }

            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }
    })
}
