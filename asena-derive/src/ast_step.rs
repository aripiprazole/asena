use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

use crate::ast_walkable::Args;

#[allow(clippy::redundant_clone)]
pub fn expand_ast_step(args: TokenStream, input: TokenStream) -> TokenStream {
    let constraints = parse_macro_input!(args as Args);
    let input = parse_macro_input!(input as ItemStruct);
    let name = input.ident.clone();

    let constraints = constraints.vars.into_iter().fold(quote!(), |acc, walker| {
        quote! { #acc
            impl #walker for #name {}
        }
    });

    TokenStream::from(quote! {
        #input
        #constraints
    })
}
