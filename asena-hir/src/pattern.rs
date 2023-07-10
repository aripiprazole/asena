use asena_hir_derive::*;

use crate::{literal::HirLiteral, HirVisitor, NameId};

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
#[hir_debug]
pub struct HirPatternConstructor {
    pub constructor_name: NameId,
    pub arguments: Vec<HirPatternId>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
#[hir_debug]
pub struct HirPatternList {
    pub items: Vec<HirPatternId>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
#[hir_debug]
pub struct HirPatternName {
    pub name: NameId,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
#[hir_debug]
pub struct HirPatternLiteral(pub HirLiteral);

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirPattern)]
pub enum HirPatternKind {
    #[default]
    Error,
    Wildcard,
    Spread,
    Unit,
    HirPatternConstructor(HirPatternConstructor),
    HirPatternList(HirPatternList),
    HirPatternName(HirPatternName),
    HirPatternLiteral(HirPatternLiteral),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirPattern {
    pub kind: HirPatternKind,
}
