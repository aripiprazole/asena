use asena_derive::*;

use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use crate::*;

/// Value body node, is a value body that is an `=`.
#[derive(Default, Node, Located, Clone)]
pub struct ExprBranch(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, ExprWalker, PatWalker, StmtWalker)]
impl ExprBranch {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

/// Do body node, is a value body that is an do-notation.
#[derive(Default, Node, Located, Clone)]
pub struct BlockBranch(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, ExprWalker, PatWalker, StmtWalker)]
impl BlockBranch {
    #[ast_leaf]
    pub fn stmts(&self) -> Vec<Stmt> {
        self.filter()
    }
}

ast_enum! {
    #[derive(Walker)]
    #[ast_walker_traits(ExprWalker, PatWalker, StmtWalker)]
    pub enum Branch {
        ExprBranch  <- BranchExpr,
        BlockBranch <- BranchBlock,
    }
}
