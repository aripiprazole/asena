use std::sync::Arc;

use asena_hir_derive::*;

use crate::{expr::HirExprId, query::HirDebug, HirVisitor};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
#[hir_debug]
pub struct HirValueBlock {
    pub instructions: Vec<HirValueId>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
pub struct HirValueExpr(pub HirExprId);

impl HirDebug for HirValueExpr {
    type Database = dyn crate::database::HirBag;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(db, f)
    }
}

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
