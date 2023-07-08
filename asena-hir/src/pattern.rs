use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Hash, Clone, Debug)]
#[hir_kind(HirPattern)]
pub enum HirPatternKind {
    Error,
}

#[hir_struct(HirVisitor)]
#[derive(Hash, Clone, Debug)]
pub struct HirPattern {
    pub kind: HirPatternKind,
}
