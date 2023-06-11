use asena_derive::Leaf;
use asena_leaf::ast_enum;
use asena_leaf::green::GreenTree;
use asena_leaf::node::TreeKind;
use asena_leaf::spec::Node;

use asena_span::Spanned;

use crate::*;

#[derive(Leaf, Clone)]
pub struct Ask(GreenTree);

impl Ask {
    pub fn pattern(&self) -> Node<Spanned<Pat>> {
        todo!()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

#[derive(Leaf, Clone)]
pub struct Set(GreenTree);

impl Set {
    pub fn pattern(&self) -> Node<Spanned<Pat>> {
        self.filter::<Pat>().first().cloned().into()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        self.filter::<Expr>().first().cloned().into()
    }
}

#[derive(Leaf, Clone)]
pub struct Return(GreenTree);

impl Return {
    /// This is using directly [ExprRef] in the AST, because when expanded, this will generate
    /// and [Option] wrapped value.
    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

#[derive(Leaf, Clone)]
pub struct Eval(GreenTree);

impl Eval {
    pub fn value(&self) -> Node<Spanned<Expr>> {
        self.filter::<Expr>().first().cloned().into()
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
