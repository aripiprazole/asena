use asena_derive::*;

use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use crate::*;

/// Value body node, is a value body that is an `=`.
#[derive(Default, Node, Located, Clone)]
pub struct Value(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Value {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

/// Do body node, is a value body that is an do-notation.
#[derive(Default, Node, Located, Clone)]
pub struct Do(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Do {
    #[ast_leaf]
    pub fn stmts(&self) -> Vec<Stmt> {
        self.filter()
    }
}

ast_enum! {
    pub enum Body {
        Value <- BodyValue,
        Do    <- BodyDo,
    }
}
