use asena_derive::Leaf;

use asena_leaf::ast_enum;
use asena_leaf::green::GreenTree;
use asena_leaf::node::TreeKind;
use asena_leaf::spec::Node;

use asena_span::Spanned;

use crate::*;

/// Value body node, is a value body that is an `=`.
#[derive(Leaf, Clone)]
pub struct Value(GreenTree);

impl Value {
    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

/// Do body node, is a value body that is an do-notation.
#[derive(Leaf, Clone)]
pub struct Do(GreenTree);

impl Do {
    pub fn stmts(&self) -> Node<Vec<Spanned<Stmt>>> {
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
