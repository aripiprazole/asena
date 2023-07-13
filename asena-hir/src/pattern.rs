use asena_hir_derive::*;

use crate::{interner::HirInterner, literal::HirLiteral, Name};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
pub struct HirPatternConstructor {
    pub constructor_name: Name,
    pub arguments: Vec<HirPattern>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
pub struct HirPatternList {
    pub items: Vec<HirPattern>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirPattern)]
pub struct HirPatternName {
    pub name: Name,
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

impl HirPattern {
    pub fn error(db: &dyn HirInterner) -> HirPattern {
        db.intern_pattern(HirPatternData::default())
    }

    pub fn wildcard(db: &dyn HirInterner) -> HirPattern {
        db.intern_pattern(HirPatternData {
            kind: HirPatternKind::Wildcard,
            span: Default::default(),
        })
    }

    pub fn this(db: &dyn HirInterner) -> HirPattern {
        db.intern_pattern(HirPatternData {
            kind: HirPatternKind::This,
            span: Default::default(),
        })
    }

    pub fn spread(db: &dyn HirInterner) -> HirPattern {
        db.intern_pattern(HirPatternData {
            kind: HirPatternKind::Spread,
            span: Default::default(),
        })
    }

    pub fn name(db: &dyn HirInterner, name: Name) -> HirPattern {
        let kind = HirPatternKind::from(HirPatternName { name });

        db.intern_pattern(HirPatternData {
            kind,
            span: Default::default(),
        })
    }

    pub fn new_true(db: &dyn HirInterner) -> HirPattern {
        let kind = HirPatternKind::from(HirPatternLiteral(HirLiteral::TRUE));

        db.intern_pattern(HirPatternData {
            kind,
            span: Default::default(),
        })
    }

    pub fn new_false(db: &dyn HirInterner) -> HirPattern {
        let kind = HirPatternKind::from(HirPatternLiteral(HirLiteral::TRUE));

        db.intern_pattern(HirPatternData {
            kind,
            span: Default::default(),
        })
    }
}
