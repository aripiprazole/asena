use asena_derive::{ast_debug, ast_leaf, Leaf};
use asena_leaf::ast::GreenTree;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind;

use asena_span::Spanned;

use crate::*;

#[derive(Default, Leaf, Clone)]
pub struct Ask(GreenTree);

#[ast_of]
#[ast_debug]
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

#[derive(Default, Leaf, Clone)]
pub struct Set(GreenTree);

#[ast_of]
#[ast_debug]
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

#[derive(Default, Leaf, Clone)]
pub struct Return(GreenTree);

#[ast_of]
#[ast_debug]
impl Return {
    /// This is using directly [ExprRef] in the AST, because when expanded, this will generate
    /// and [Option] wrapped value.
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        todo!()
    }
}

#[derive(Default, Leaf, Clone)]
pub struct Eval(GreenTree);

#[ast_of]
#[ast_debug]
impl Eval {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter::<Expr>().first()
    }
}

ast_enum! {
    pub enum Stmt {
        Ask    <- TreeKind::StmtAsk,    // <local_id> <- <expr>
        Set    <- TreeKind::StmtLet,    // let <local_id> = <expr>
        Return <- TreeKind::StmtReturn, // return <expr?>
        Eval   <- TreeKind::StmtExpr,   // <expr?>
    }
}

pub type StmtRef = Spanned<Stmt>;
