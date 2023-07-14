use asena_hir_derive::*;

use crate::{pattern::HirPattern, value::HirValue};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirStmt)]
pub struct HirStmtAsk {
    pub pattern: HirPattern,
    pub value: HirValue,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirStmt)]
pub struct HirStmtLet {
    pub pattern: HirPattern,
    pub value: HirValue,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirStmt)]
pub struct HirStmtReturn {
    pub value: HirValue,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirStmt)]
pub struct HirStmtValue(pub HirValue);

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirStmt)]
pub enum HirStmtKind {
    #[default]
    Error,
    Ask(HirStmtAsk),
    Let(HirStmtLet),
    Return(HirStmtReturn),
    Value(HirStmtValue),
}

#[hir_struct]
pub struct HirStmt {
    pub kind: HirStmtKind,
}
