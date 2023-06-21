use std::fmt::{Debug, Formatter};

use asena_derive::{ast_debug, ast_leaf, ast_of, Leaf};

use asena_leaf::ast::{Cursor, GreenTree};

/// Represents a true-false value, just like an wrapper to [bool], this represents if an integer
/// value is signed, or unsigned.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signed {
    Signed,
    Unsigned,
}

/// Represents the root of the asena source code file, it contains a set of declarations.
#[derive(Default, Leaf, Clone)]
pub struct AsenaFile(GreenTree);

#[ast_of]
#[ast_debug]
impl AsenaFile {
    // fn declarations(&self) -> Vec<Decl>
    //
    // fn set_declarations(&self, value: Vec<Decl>)
    //
    // fn find_declarations(&self) -> Cursor<Vec<Decl>>
    #[ast_leaf]
    pub fn declarations(&self) -> Cursor<Vec<Decl>> {
        self.filter()
    }
}

pub use binding::*;
pub use body::*;
pub use decl::*;
pub use expr::*;
pub use identifier::*;
pub use literal::*;
pub use parameter::*;
pub use pat::*;
pub use stmt::*;
pub use traits::binary::*;

pub mod binding;
pub mod body;
pub mod decl;
pub mod expr;
pub mod identifier;
pub mod literal;
pub mod parameter;
pub mod pat;
pub mod stmt;

pub mod traits {
    pub mod binary;
}

pub mod stub {
    pub mod display;
    pub mod enum_stub;
    pub mod leaf;
    pub mod terminal;
}
