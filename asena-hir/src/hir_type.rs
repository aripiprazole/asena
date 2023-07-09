use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Default, Hash, Clone, Debug)]
#[hir_kind(HirType)]
pub enum HirTypeKind {
    #[default]
    Error,
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug)]
pub struct HirType {
    pub kind: HirTypeKind,
}
