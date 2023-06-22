use proc_macro::TokenStream;
use quote::quote;
use syn::*;

use crate::util::iter_leaf;

pub fn expand_ast_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
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
