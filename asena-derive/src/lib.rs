#![feature(box_patterns)]
#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

extern crate proc_macro;

mod ast_build_fn;
mod ast_command;
mod ast_debug;
mod ast_derive_leaf;
mod ast_derive_walker;
mod ast_from;
mod ast_leaf;
mod ast_of;
mod ast_step;
mod ast_walkable;
pub(crate) mod util;

#[proc_macro_derive(Leaf, attributes(ast_from, ast_build_fn))]
pub fn derive_leaf(input: TokenStream) -> TokenStream {
    ast_derive_leaf::expand_derive_leaf(input)
}

#[proc_macro_derive(Walker, attributes(ast_walker_traits))]
pub fn derive_ast_walker(input: TokenStream) -> TokenStream {
    ast_derive_walker::expand_ast_derive_walker(input)
}

#[proc_macro_attribute]
pub fn ast_step(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_step::expand_ast_step(args, input)
}

#[proc_macro_attribute]
pub fn ast_debug(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_debug::expand_ast_debug(args, input)
}

#[proc_macro_attribute]
pub fn ast_leaf(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_leaf::expand_ast_leaf(args, input)
}

#[proc_macro_attribute]
pub fn ast_of(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_of::expand_ast_of(args, input)
}

#[proc_macro_attribute]
pub fn ast_from(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_from::expand_ast_from(args, input)
}

#[proc_macro_attribute]
pub fn ast_walkable(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_walkable::expand_ast_walkable(args, input)
}

#[proc_macro_attribute]
pub fn ast_build_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_build_fn::expand_ast_build_fn(args, input)
}

#[proc_macro_attribute]
pub fn ast_command(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_command::expand_ast_command(args, input)
}
