use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Hash, Clone, Debug)]
#[hir_kind(HirPattern)]
pub enum HirPatternKind {
    Error,
}

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirPattern {
    pub span: asena_span::Loc,
    pub id: HirPatternId,
    pub kind: HirPatternKind,
}
