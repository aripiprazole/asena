use asena_derive::{node_leaf, Leaf};

use asena_leaf::ast::Cursor;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind;

use crate::*;

/// Value body node, is a value body that is an `=`.
#[derive(Leaf, Clone)]
pub struct Value(GreenTree);

impl Value {
    #[node_leaf]
    pub fn value(&self) -> Cursor<Expr> {
        todo!()
    }
}

/// Do body node, is a value body that is an do-notation.
#[derive(Leaf, Clone)]
pub struct Do(GreenTree);

impl Do {
    #[node_leaf]
    pub fn stmts(&self) -> Vec<Cursor<Stmt>> {
        todo!()
    }
}

ast_enum! {
    #[derive(Debug)]
    pub enum Body {
        Value <- TreeKind::BodyValue,
        Do    <- TreeKind::BodyDo,
    }
}
