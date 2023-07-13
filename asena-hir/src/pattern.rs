use asena_hir_derive::*;

use crate::{literal::HirLiteral, NameId};

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
pub struct HirPatternConstructor {
    pub constructor_name: NameId,
    pub arguments: Vec<HirPattern>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
pub struct HirPatternList {
    pub items: Vec<HirPattern>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
pub struct HirPatternName {
    pub name: NameId,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
pub struct HirPatternLiteral(pub HirLiteral);

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirPattern)]
pub enum HirPatternKind {
    #[default]
    Error,
    Wildcard,
    Spread,
    Unit,
    This,
    HirPatternConstructor(HirPatternConstructor),
    HirPatternList(HirPatternList),
    HirPatternName(HirPatternName),
    HirPatternLiteral(HirPatternLiteral),
}

#[hir_struct]
pub struct HirPattern {
    pub kind: HirPatternKind,
}
