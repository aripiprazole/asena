#![feature(default_free_fn)]

use std::fmt::{Debug, Formatter};

use asena_derive::*;

use asena_leaf::ast::GreenTree;

/// Represents a true-false value, just like an wrapper to [bool], this represents if an integer
/// value is signed, or unsigned.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signed {
    Signed,
    Unsigned,
}

/// Represents the root of the asena source code file, it contains a set of declarations.
#[derive(Default, Node, Clone)]
pub struct AsenaFile(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl AsenaFile {
    #[ast_leaf]
    pub fn declarations(&self) -> Vec<Decl> {
        self.filter()
    }
}

pub trait FileWalker {
    fn walk_file(&mut self, _value: &AsenaFile) {}
}

pub use body::*;
pub use decl::*;
pub use expr::*;
pub use identifier::*;
pub use literal::*;
pub use parameter::*;
pub use pat::*;
pub use stmt::*;
pub use traits::binary::*;
pub use traits::function::*;
pub use visitor::*;

pub mod body;
pub mod decl;
pub mod expr;
pub mod identifier;
pub mod literal;
pub mod parameter;
pub mod pat;
pub mod reporter;
pub mod stmt;
pub mod visitor;

pub mod traits {
    pub mod binary;
    pub mod function;
}
