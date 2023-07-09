use asena_hir_derive::*;

use crate::{pattern::HirPatternId, value::HirValueId, HirVisitor};

#[derive(Hash, Clone, Debug)]
#[hir_node(HirStmt)]
pub struct HirStmtAsk {
    pub pattern: HirPatternId,
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node(HirStmt)]
pub struct HirStmtReturn {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node(HirStmt)]
pub struct HirStmtValue(pub HirValueId);

#[derive(Default, Hash, Clone, Debug)]
#[hir_kind(HirStmt)]
pub enum HirStmtKind {
    #[default]
    Error,
    Ask(HirStmtAsk),
    Return(HirStmtReturn),
    Value(HirStmtValue),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug)]
pub struct HirStmt {
    pub kind: HirStmtKind,
}
