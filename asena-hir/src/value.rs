use asena_hir_derive::*;

use crate::{expr::HirExprId, HirVisitor};

#[derive(Hash, Clone, Debug)]
#[hir_node(HirValue)]
pub struct HirValueBlock {
    pub instructions: Vec<HirValueId>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node(HirValue)]
pub struct HirValueExpr(pub HirExprId);

#[derive(Default, Hash, Clone, Debug)]
#[hir_kind(HirValue)]
pub enum HirValueKind {
    #[default]
    Error,
    HirValueBlock(HirValueBlock),
    HirValueExpr(HirValueExpr),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug)]
pub struct HirValue {
    pub kind: HirValueKind,
}
