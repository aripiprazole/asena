use asena_hir_derive::*;

use crate::HirVisitor;

#[derive(Hash, Clone, Copy, Debug)]
pub enum HirInlineKind {
    Always,
    Never,
}

#[derive(Hash, Clone, Debug)]
#[hir_kind]
pub enum HirAttrKind {
    Error,
    Inline(HirInlineKind),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirAttr {
    pub span: asena_span::Loc,
    pub id: HirAttrId,
    pub kind: HirAttrKind,
}
