use asena_derive::{ast_debug, ast_leaf, Leaf};

use asena_leaf::ast::Cursor;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind;

use crate::*;

/// Value body node, is a value body that is an `=`.
#[derive(Leaf, Clone)]
pub struct Value(GreenTree);

#[ast_debug]
impl Value {
    #[ast_leaf]
    pub fn value(&self) -> Cursor<Expr> {
        todo!()
    }
}

/// Do body node, is a value body that is an do-notation.
#[derive(Leaf, Clone)]
pub struct Do(GreenTree);

#[ast_debug]
impl Do {
    #[ast_leaf]
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
