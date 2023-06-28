#![feature(box_patterns)]
#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

extern crate proc_macro;

mod ast_command;
mod ast_debug;
mod ast_derive_leaf;
mod ast_derive_node;
mod ast_derive_step;
mod ast_derive_walker;
mod ast_leaf;
mod ast_of;
mod ast_step;
mod ast_walkable;
pub(crate) mod util;

/// Proc-macro `Leaf` derives the Leaf attributes and trait for the given enum.
#[proc_macro_derive(Leaf, attributes(ast_terminal, ast_from, ast_build_fn))]
pub fn derive_leaf(input: TokenStream) -> TokenStream {
    ast_derive_leaf::expand_derive_leaf(input)
}

/// Proc-macro `Node` derives the Node attributes and trait for the given struct. The struct must
/// have a field with `GreenTree` type.
#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    ast_derive_node::expand_derive_node(input)
}

#[proc_macro_derive(Walker, attributes(ast_walker_traits, ast_impl_trait))]
pub fn derive_ast_walker(input: TokenStream) -> TokenStream {
    ast_derive_walker::expand_ast_derive_walker(input)
}

#[proc_macro_derive(Reporter, attributes(ast_reporter))]
pub fn ast_derive_step(input: TokenStream) -> TokenStream {
    ast_derive_step::expand_ast_derive_step(input)
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
pub fn ast_walkable(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_walkable::expand_ast_walkable(args, input)
}

#[proc_macro_attribute]
pub fn ast_command(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_command::expand_ast_command(args, input)
}
