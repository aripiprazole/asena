use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn expand_ast_command(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    TokenStream::from(quote! {
        #input
    })
}
