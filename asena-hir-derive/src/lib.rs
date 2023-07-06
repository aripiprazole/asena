use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn hir_node(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn hir_kind(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn hir_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn hir_id(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
