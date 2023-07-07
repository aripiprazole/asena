use asena_hir_derive::*;

use crate::{
    pattern::HirPatternId, stmt::HirStmtId, value::HirValueId, HirLiteralId, HirTypeId, HirVisitor,
    NameId, ScopeId,
};

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprGroup {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprLiteral {
    pub value: HirLiteralId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirDsl {
    pub parameters: Vec<NameId>,
    pub stmts: Vec<HirStmtId>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprCall {
    pub callee: HirValueId,
    pub arguments: Vec<HirValueId>,
    pub as_dsl: Option<HirDsl>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprReference {
    pub scope: ScopeId,
    pub name: NameId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub enum HirMatchArm {
    Expr(HirValueId),
    Block(HirStmtId),
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirMatchCase {
    pub pattern: HirPatternId,
    pub value: HirMatchArm,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprMatch {
    pub scrutinee: HirValueId,
    pub cases: Vec<HirMatchCase>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprHelp {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprAnn {
    pub value: HirValueId,
    pub against: HirTypeId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprLam {
    pub parameters: Vec<NameId>,
    pub value: HirExprId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub enum HirIfBranch {
    Expr(HirValueId),
    Block(Vec<HirStmtId>),
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprIf {
    pub condition: HirValueId,
    pub then_branch: HirIfBranch,
    pub otherwise_branch: Option<HirIfBranch>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirExprArray {
    pub items: Vec<HirValueId>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub enum HirUnimplemented {}

#[derive(Hash, Clone, Debug)]
#[hir_kind]
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

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirExpr {
    pub span: asena_span::Loc,
    pub id: HirExprId,
    pub kind: HirExprKind,
}
