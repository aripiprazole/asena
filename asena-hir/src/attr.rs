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
    #[derive(Hash, Clone, Copy, Debug)]
    pub enum HirInlineKind {
        Always,
        Never,
    }
}
