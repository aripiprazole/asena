use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Default, Hash, Clone, Debug)]
#[hir_kind(HirPattern)]
pub enum HirPatternKind {
    #[default]
    Error,
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug)]
pub struct HirPattern {
    pub kind: HirPatternKind,
}
