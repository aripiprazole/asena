use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Default, Hash, Clone, Debug)]
#[hir_node(HirAttr)]
pub struct HirAttrInline {
    pub kind: data::HirInlineKind,
}

#[derive(Default, Hash, Clone, Debug)]
#[hir_kind(HirAttr)]
pub enum HirAttrKind {
    #[default]
    Error,
    HirAttrInline(HirAttrInline),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug)]
pub struct HirAttr {
    pub kind: HirAttrKind,
}

pub mod data {
    use super::*;

    #[derive(Default, Hash, Clone, Copy, Debug)]
    #[hir_debug]
    pub enum HirInlineKind {
        #[default]
        Never,
        Always,
    }
}
