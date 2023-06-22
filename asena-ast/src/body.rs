use asena_derive::{ast_debug, ast_leaf, Leaf};

use asena_leaf::ast::Leaf;
use asena_leaf::ast_enum;
use asena_leaf::node::{Tree, TreeKind};
use asena_span::Spanned;

use crate::*;

/// Value body node, is a value body that is an `=`.
#[derive(Default, Leaf, Clone)]
pub struct Value(GreenTree);

#[ast_of]
#[ast_debug]
impl Value {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        todo!()
    }
}

/// Do body node, is a value body that is an do-notation.
#[derive(Default, Leaf, Clone)]
pub struct Do(GreenTree);

#[ast_of]
#[ast_debug]
impl Do {
    #[ast_leaf]
    pub fn stmts(&self) -> Vec<Stmt> {
        todo!()
    }
}

ast_enum! {
    pub enum Body {
        Value <- TreeKind::BodyValue,
        Do    <- TreeKind::BodyDo,
    }
}

impl Leaf for Body {
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}
