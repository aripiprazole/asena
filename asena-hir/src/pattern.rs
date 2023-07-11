use std::sync::Arc;

use asena_hir_derive::*;

use crate::{
    database::HirBag,
    literal::{HirISign, HirISize, HirLiteral},
    HirVisitor, NameId,
};

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

impl HirPattern {
    pub fn create(db: Arc<dyn HirBag>, kind: HirPatternKind) -> HirPatternId {
        Self::new(db, kind, Default::default())
    }

    pub fn new_true(db: Arc<dyn HirBag>) -> HirPatternId {
        let literal = HirLiteral::Int(1, HirISize::U1, HirISign::Unsigned);
        Self::create(db, HirPatternKind::from(HirPatternLiteral(literal)))
    }

    pub fn new_false(db: Arc<dyn HirBag>) -> HirPatternId {
        let literal = HirLiteral::Int(0, HirISize::U1, HirISign::Unsigned);
        Self::create(db, HirPatternKind::from(HirPatternLiteral(literal)))
    }
}
