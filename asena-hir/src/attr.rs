use asena_hir_derive::*;

use crate::{HirVisitor, NameId};

#[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_node(HirAttr)]
#[hir_debug]
pub struct HirAttrInline {
    pub kind: data::HirInlineKind,
}

#[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_node(HirAttr)]
#[hir_debug]
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

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirAttr {
    pub kind: HirAttrKind,
}

pub mod data {
    use super::*;

    #[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub enum HirInlineKind {
        #[default]
        Never,
        Always,
    }
}
