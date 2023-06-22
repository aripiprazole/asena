use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use crate::ast_walkable::Args;

#[allow(clippy::redundant_clone)]
pub fn expand_ast_step(args: TokenStream, input: TokenStream) -> TokenStream {
    let constraints = parse_macro_input!(args as Args);
    let input = parse_macro_input!(input as ItemStruct);
    let name = input.ident.clone();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let constraints = constraints.vars.into_iter().fold(quote!(), |acc, walker| {
        quote! { #acc
            impl #impl_generics #walker for #name #ty_generics #where_clause {}
        }
    });

    TokenStream::from(quote! {
        #input
        #constraints
    })
}
