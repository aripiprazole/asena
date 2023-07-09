use asena_hir_derive::*;

use crate::{pattern::HirPatternId, value::HirValueId, HirVisitor};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirStmt)]
pub struct HirStmtAsk {
    pub pattern: HirPatternId,
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirStmt)]
pub struct HirStmtReturn {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirStmt)]
pub struct HirStmtValue(pub HirValueId);

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirStmt)]
pub enum HirStmtKind {
    #[default]
    Error,
    HirStmtAsk(HirStmtAsk),
    HirStmtReturn(HirStmtReturn),
    HirStmtValue(HirStmtValue),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirStmt {
    pub kind: HirStmtKind,
}
