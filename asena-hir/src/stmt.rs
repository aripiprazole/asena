use asena_hir_derive::*;

use crate::{pattern::HirPatternId, value::HirValueId, HirVisitor};

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirStmtAsk {
    pub pattern: HirPatternId,
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirStmtReturn {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_kind]
pub enum HirStmtKind {
    Error,
    Ask(HirStmtAsk),
    Return(HirStmtReturn),
    Value(HirValueId),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirStmt {
    pub span: asena_span::Loc,
    pub id: HirStmtId,
    pub kind: HirStmtKind,
}
