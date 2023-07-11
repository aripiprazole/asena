use std::sync::Arc;

use asena_hir_derive::*;

use crate::{
    database::HirBag,
    expr::HirExprId,
    hir_dbg,
    query::{leaf::HirLoc, HirDebug},
    stmt::HirStmtId,
    HirVisitor,
};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
pub struct HirValueBlock {
    pub instructions: Vec<HirStmtId>,
    pub value: HirValueId,
}

impl HirDebug for HirValueBlock {
    type Database = dyn crate::database::HirBag;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.instructions.is_empty() {
            self.value.fmt(db, f)
        } else {
            write!(f, "HirValueBlock(")?;
            let mut s = f.debug_list();
            for instruction in self.instructions.iter() {
                s.entry(&hir_dbg!(db.clone(), instruction));
            }
            s.entry(&hir_dbg!(db.clone(), &self.value));
            s.finish()?;
            write!(f, ")")
        }
    }
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
pub struct HirValueExpr(pub HirExprId);

impl HirDebug for HirValueExpr {
    type Database = dyn crate::database::HirBag;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(db, f)
    }
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirValue)]
pub enum HirValueKind {
    #[default]
    Error,
    HirValueUnit,
    HirValueBlock(HirValueBlock),
    HirValueExpr(HirValueExpr),
    HirInstrNull,
    HirInstrLet(runtime::HirInstrLet),
    HirInstrVariable(runtime::HirInstrVariable),
    HirInstrPure(runtime::HirInstrPure),
    HirInstrObjectDrop(runtime::HirInstrObjectDrop),
    HirInstrObjectClone(runtime::HirInstrObjectClone),
    HirValueInstrBlock(runtime::HirValueInstrBlock),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirValue {
    pub kind: HirValueKind,
}

impl HirValue {
    pub fn value(db: Arc<dyn HirBag>, expr: HirExprId) -> HirValueId {
        let kind = HirValueKind::from(HirValueExpr(expr));
        let db = db.clone();

        HirValue::new(db.clone(), kind, HirLoc::default())
    }
}

/// This module holds all the runtime instructions that are used by the compiler. These instructions
/// are not part of the language itself, but are used to implement the language.
///
/// The instructions are:
pub mod runtime {
    use std::{fmt::Formatter, sync::Arc};

    use asena_hir_derive::{hir_debug, hir_node};

    use super::{HirValue, HirValueId, HirValueKind};
    use crate::database::HirBag;
    use crate::hir_dbg;
    use crate::query::{leaf::HirLoc, HirDebug};
    use crate::NameId;

    impl HirValue {
        pub fn pure_unit(db: Arc<dyn HirBag>) -> HirValueId {
            let value = HirValue::new(db.clone(), HirValueKind::HirValueUnit, HirLoc::default());

            let kind = HirValueKind::from(HirInstrPure(value));
            let db = db.clone();

            HirValue::new(db.clone(), kind, HirLoc::default())
        }
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_node(HirValue)]
    #[hir_debug]
    pub struct HirInstrLet(pub NameId, pub HirValueId);

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_node(HirValue)]
    #[hir_debug]
    pub struct HirInstrVariable(pub NameId);

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_node(HirValue)]
    #[hir_debug]
    pub struct HirInstrPure(pub HirValueId);

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_node(HirValue)]
    #[hir_debug]
    pub struct HirInstrObjectClone(pub HirValueId);

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_node(HirValue)]
    #[hir_debug]
    pub struct HirInstrObjectDrop(pub HirValueId);

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_node(HirValue)]
    pub struct HirValueInstrBlock {
        pub instructions: Vec<HirValueId>,
        pub value: HirValueId,
    }

    impl HirDebug for HirValueInstrBlock {
        type Database = dyn crate::database::HirBag;

        fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
            if self.instructions.is_empty() {
                self.value.fmt(db, f)
            } else {
                write!(f, "HirValueInstrBlock(")?;
                let mut s = f.debug_list();
                for instruction in self.instructions.iter() {
                    s.entry(&hir_dbg!(db.clone(), instruction));
                }
                s.entry(&hir_dbg!(db.clone(), &self.value));
                s.finish()?;
                write!(f, ")")
            }
        }
    }
}
