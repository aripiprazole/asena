//! This crates provides an High Level API for the Asena Abstract Syntax Tree, with a focus on
//! object oriented programming, to make it easier to use.
//!
//! It's an abstraction layer over the AST, and it's not meant to be used for parsing, but for
//! semantic analysis and code generation.

use asena_ast::visitor;
use asena_hir_leaf::{hir_declare, HirId, HirNode};
use asena_span::{Loc, Spanned};

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirTypeId(usize);

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirLiteralId(usize);

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirPatternId(usize);

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirStmtId(usize);

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirExprId(usize);

impl HirId for HirExprId {
    type Node = HirExpr;

    fn new(node: Self::Node) -> Self {
        node.id
    }
}

pub struct ScopeId(usize);

pub struct NameId(usize);

pub struct HirExprGroup {
    pub value: HirExprId,
}

pub struct HirExprLiteral {
    pub value: HirLiteralId,
}

pub struct HirDsl {
    pub parameters: Vec<NameId>,
    pub stmts: Vec<HirStmtId>,
}

pub struct HirExprCall {
    pub callee: HirExprId,
    pub arguments: Vec<HirExprId>,
    pub as_dsl: Option<HirDsl>,
}

pub struct HirExprReference {
    pub scope: ScopeId,
    pub name: Spanned<NameId>,
}

pub enum HirMatchArm {
    Expr(HirExprId),
    Block(HirStmtId),
}

pub struct HirMatchCase {
    pub pattern: HirPatternId,
    pub value: HirMatchArm,
}

pub struct HirExprMatch {
    pub scrutinee: HirExprId,
    pub cases: Vec<HirMatchCase>,
}

pub struct HirExprHelp {
    pub value: HirExprId,
}

pub struct HirExprAnn {
    pub value: HirExprId,
    pub against: HirTypeId,
}

pub struct HirExprLam {
    pub parameters: Vec<NameId>,
    pub value: HirExprId,
}

pub enum HirIfBranch {
    Expr(HirExprId),
    Block(Vec<HirStmtId>),
}

pub struct HirExprIf {
    pub condition: HirExprId,
    pub then_branch: HirIfBranch,
    pub otherwise_branch: Option<HirIfBranch>,
}

pub struct HirExprArray {
    pub items: Vec<HirExprId>,
}

pub enum HirUnimplemented {}

pub enum HirExprKind {
    Error,
    Unit,
    This,
    Unimplemented(HirUnimplemented),
    Group(HirExprGroup),
    Literal(HirExprLiteral),
    Reference(HirExprReference),
    Call(HirExprCall),
    Match(HirExprMatch),
    Help(HirExprHelp),
    Ann(HirExprAnn),
    Lam(HirExprLam),
    If(HirExprIf),
    Array(HirExprArray),
}

pub struct HirExpr {
    pub span: Loc,
    pub id: HirExprId,
    pub kind: HirExprKind,
}

impl HirNode for HirExpr {
    type Id = HirExprId;
    type Visitor<'a, T> = dyn HirVisitor<T>;

    fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
        todo!()
    }
}

pub trait HirVisitor<T: Default> {
    fn visit_expr_group(&mut self, expr: &mut HirExprGroup2) -> T {
        let _ = expr;
        T::default()
    }

    fn visit_expr_unit(&mut self, expr: &mut HirExprUnit2) -> T {
        let _ = expr;
        T::default()
    }
}

hir_declare! {
    pub trait HirExpr2 {
        type Visitor = HirVisitor;
    }

    pub struct HirExprUnit2 : HirExpr2 {
        let value: ();

        fn accept<O: Default>(&mut self, visitor: &mut Self::Visitor<'_, O>) -> O {
            visitor.visit_expr_unit(self)
        }
    }

    pub struct HirExprGroup2 : HirExpr2 {
        let value: HirExprId;

        fn accept<O: Default>(&mut self, visitor: &mut Self::Visitor<'_, O>) -> O {
            visitor.visit_expr_group(self)
        }
    }
}
