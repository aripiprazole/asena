use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Hash, Clone, Debug)]
#[hir_kind(HirAttr)]
pub enum HirAttrKind {
    Error,
    Inline(data::HirInlineKind),
}

#[hir_struct(HirVisitor)]
#[derive(Hash, Clone, Debug)]
pub struct HirAttr {
    pub kind: HirAttrKind,
}

pub mod data {
    use super::*;

    #[derive(Hash, Clone, Copy, Debug)]
    #[hir_debug]
    pub enum HirInlineKind {
        Always,
        Never,
    }
}
