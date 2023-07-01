use asena_derive::*;

use asena_leaf::ast::GreenTree;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use asena_span::Spanned;

use crate::*;

#[derive(Default, Node, Located, Clone)]
pub struct Ask(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, PatWalker, ExprWalker, StmtWalker)]
impl Ask {
    #[ast_leaf]
    pub fn pattern(&self) -> Pat {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

#[derive(Default, Node, Located, Clone)]
pub struct IfStmt(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, PatWalker, ExprWalker, StmtWalker)]
impl IfStmt {
    #[ast_leaf]
    pub fn cond(&self) -> Expr {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn then_branch(&self) -> Branch {
        self.filter().nth(0)
    }

    #[ast_leaf]
    pub fn else_branch(&self) -> Option<Branch> {
        self.filter().try_as_nth(1)
    }
}

#[derive(Default, Node, Located, Clone)]
pub struct LetStmt(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, PatWalker, ExprWalker, StmtWalker)]
impl LetStmt {
    #[ast_leaf]
    pub fn pattern(&self) -> Pat {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

#[derive(Default, Node, Located, Clone)]
pub struct Return(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, ExprWalker, PatWalker, StmtWalker)]
impl Return {
    #[ast_leaf]
    pub fn value(&self) -> Option<Expr> {
        self.filter().first()
    }
}

#[derive(Default, Node, Located, Clone)]
pub struct ExprStmt(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, ExprWalker, PatWalker, StmtWalker)]
impl ExprStmt {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter::<Expr>().first()
    }
}

ast_enum! {
    #[derive(Walker)]
    #[ast_walker_traits(BranchWalker, PatWalker, ExprWalker)]
    pub enum Stmt {
        Ask      <- StmtAsk,    // <local_id> <- <expr>
        Return   <- StmtReturn, // return <expr?>
        IfStmt   <- StmtIf,     // if <expr> <branch> (else <branch>)?
        LetStmt  <- StmtLet,    // let <local_id> = <expr>
        ExprStmt <- StmtExpr,   // <expr?>
    }
}

pub type StmtRef = Spanned<Stmt>;
