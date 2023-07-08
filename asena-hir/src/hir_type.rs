use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Hash, Clone, Debug)]
#[hir_kind(HirType)]
pub enum HirTypeKind {
    Error,
}

#[hir_struct(HirVisitor)]
#[derive(Hash, Clone, Debug)]
pub struct HirType {
    pub kind: HirTypeKind,
}
