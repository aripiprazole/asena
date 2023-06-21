use std::fmt::Debug;

use asena_derive::{ast_node, leaf};

use asena_leaf::ast::{GreenTree, Leaf};

/// Represents a true-false value, just like an wrapper to [bool], this represents if an integer
/// value is signed, or unsigned.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signed {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone)]
pub struct Decl;

pub trait A {
    fn a(&self) {}
}

impl A for Decl {
    fn a(&self) {}
}

/// Represents the root of the asena source code file, it contains a set of declarations.
#[ast_node]
pub trait AsenaFile {
    #[leaf]
    fn declarations(&self) -> Vec<Decl> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::AsenaFile;

    #[test]
    fn works() {
        let file = AsenaFile::new(vec![]);

        println!("{:?}", file);
    }
}
