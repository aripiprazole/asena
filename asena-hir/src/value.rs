use asena_hir_derive::*;

use crate::{expr::HirExprId, HirVisitor};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
#[hir_debug]
pub struct HirValueBlock {
    pub instructions: Vec<HirValueId>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
#[hir_debug]
pub struct HirValueExpr(pub HirExprId);

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirValue)]
pub enum HirValueKind {
    #[default]
    Error,
    HirValueBlock(HirValueBlock),
    HirValueExpr(HirValueExpr),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirValue {
    pub kind: HirValueKind,
}
