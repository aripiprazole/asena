use asena_hir_derive::*;

use crate::NameId;

#[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_node(HirAttr)]
pub struct HirAttrInline {
    pub kind: data::HirInlineKind,
}

#[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_node(HirAttr)]
pub struct HirAttrExternal {
    pub ffi_name: NameId,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirAttr)]
pub enum HirAttrKind {
    #[default]
    Error,
    HirAttrInline(HirAttrInline),
    HirAttrExternal(HirAttrExternal),
}

#[hir_struct]
pub struct HirAttr {
    pub kind: HirAttrKind,
}

pub mod data {
    #[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HirInlineKind {
        #[default]
        Never,
        Always,
    }
}
