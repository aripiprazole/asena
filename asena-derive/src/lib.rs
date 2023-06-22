#![feature(box_patterns)]
#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

extern crate proc_macro;

mod ast_debug;
mod ast_derive_leaf;
mod ast_leaf;
mod ast_of;
pub(crate) mod util;

#[proc_macro_attribute]
pub fn ast_debug(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_debug::expand_ast_debug(args, input)
}

#[proc_macro_derive(Leaf)]
pub fn derive_leaf(input: TokenStream) -> TokenStream {
    ast_derive_leaf::expand_derive_leaf(input)
}

#[proc_macro_attribute]
pub fn ast_leaf(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_leaf::expand_ast_leaf(args, input)
}

#[proc_macro_attribute]
pub fn ast_of(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_of::expand_ast_of(args, input)
}
