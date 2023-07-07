use asena_hir_derive::*;

use crate::{expr::HirExprId, HirVisitor};

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirBlockValue {
    pub instructions: Vec<HirValueId>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node(transparent)]
pub struct HirExprValue(pub HirExprId);

#[derive(Hash, Clone, Debug)]
#[hir_kind]
pub enum HirValueKind {
    Error,
    Block(HirBlockValue),
    Expr(HirExprValue),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirValue {
    pub span: asena_span::Loc,
    pub id: HirValueId,
    pub kind: HirValueKind,
}
