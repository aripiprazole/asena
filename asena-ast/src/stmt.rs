use asena_derive::*;

use asena_leaf::ast::GreenTree;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use asena_span::Spanned;

use crate::*;

#[derive(Default, Node, Clone)]
pub struct Ask(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, ExprWalker, StmtWalker)]
impl Ask {
    #[ast_leaf]
    pub fn pattern(&self) -> Pat {
        todo!()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        todo!()
    }
}

#[derive(Default, Node, Clone)]
pub struct Set(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, ExprWalker, StmtWalker)]
impl Set {
    #[ast_leaf]
    pub fn pattern(&self) -> Pat {
        self.filter::<Pat>().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter::<Expr>().first()
    }
}

#[derive(Default, Node, Clone)]
pub struct Return(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, PatWalker, StmtWalker)]
impl Return {
    /// This is using directly [ExprRef] in the AST, because when expanded, this will generate
    /// and [Option] wrapped value.
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        todo!()
    }
}

#[derive(Default, Node, Clone)]
pub struct Eval(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, PatWalker, StmtWalker)]
impl Eval {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter::<Expr>().first()
    }
}

ast_enum! {
    #[derive(Walker)]
    #[ast_walker_traits(PatWalker, ExprWalker)]
    pub enum Stmt {
        Ask    <- StmtAsk,    // <local_id> <- <expr>
        Set    <- StmtLet,    // let <local_id> = <expr>
        Return <- StmtReturn, // return <expr?>
        Eval   <- StmtExpr,   // <expr?>
    }
}

pub type StmtRef = Spanned<Stmt>;
