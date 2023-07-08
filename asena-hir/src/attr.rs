use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Hash, Clone, Debug)]
#[hir_kind(HirAttr)]
pub enum HirAttrKind {
    Error,
    Inline(data::HirInlineKind),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirAttr {
    pub span: asena_span::Loc,
    pub id: HirAttrId,
    pub kind: HirAttrKind,
}

pub mod data {
    #[derive(Hash, Clone, Copy, Debug)]
    pub enum HirInlineKind {
        Always,
        Never,
    }
}
