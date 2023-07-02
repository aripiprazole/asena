use asena_derive::*;

use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use crate::*;

/// A expression branch is a branch that is an expression.
///
/// # Examples
///
/// ```asena
/// if x == 0 then ()
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct ExprBranch(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl ExprBranch {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

/// A block branch is a branch that is an collection of statements.
///
/// # Examples
///
/// ```asena
/// if x == 0 {
/// }
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct BlockBranch(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl BlockBranch {
    #[ast_leaf]
    pub fn stmts(&self) -> Vec<Stmt> {
        self.filter()
    }
}

ast_enum! {
    #[ast_walker(AsenaVisitor)]
    pub enum Branch {
        ExprBranch  <- BranchExpr,
        BlockBranch <- BranchBlock,
    }
}
