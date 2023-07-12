use std::sync::Arc;

use asena_hir_derive::*;

use crate::database::HirBag;
use crate::expr::HirExprId;
use crate::hir_dbg;
use crate::query::{leaf::HirLoc, HirDebug};
use crate::stmt::HirStmtId;
use crate::HirVisitor;

use self::{instr::HirInstr, monads::HirMonad};

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
    HirMonad(HirMonad),
    HirInstr(HirInstr),
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

pub mod instr;
pub mod monads;

/// This module holds all the runtime instructions that are used by the compiler. These instructions
/// are not part of the language itself, but are used to implement the language.
///
/// The instructions are:
pub mod runtime {
    use std::sync::Arc;

    use super::monads::HirMonad;
    use super::{HirValue, HirValueId, HirValueKind};
    use crate::database::HirBag;
    use crate::query::leaf::HirLoc;

    impl HirValue {
        pub fn pure_unit(db: Arc<dyn HirBag>) -> HirValueId {
            let db = db.clone();

            let value = HirValue::new(db.clone(), HirValueKind::HirValueUnit, HirLoc::default());
            let kind = HirValueKind::from(HirMonad::Pure(value));

            HirValue::new(db.clone(), kind, HirLoc::default())
        }
    }
}
