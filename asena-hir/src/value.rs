use asena_hir_derive::*;

use crate::{expr::HirExprId, HirVisitor};

#[derive(Hash, Clone, Debug)]
#[hir_node(HirValue)]
pub struct HirBlockValue {
    pub instructions: Vec<HirValueId>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node(HirValue)]
pub struct HirExprValue(pub HirExprId);

#[derive(Hash, Clone, Debug)]
#[hir_kind(HirValue)]
pub enum HirValueKind {
    Error,
    Block(HirBlockValue),
    Expr(HirExprValue),
}

#[hir_struct(HirVisitor)]
#[derive(Hash, Clone, Debug)]
pub struct HirValue {
    pub kind: HirValueKind,
}
