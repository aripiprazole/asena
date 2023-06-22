use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

use crate::util::iter_leaf;

pub fn expand_ast_of(_args: TokenStream, input: TokenStream) -> TokenStream {
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
