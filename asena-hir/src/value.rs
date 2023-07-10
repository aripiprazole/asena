use std::sync::Arc;

use asena_hir_derive::*;

use crate::{expr::HirExprId, hir_dbg, query::HirDebug, stmt::HirStmtId, HirVisitor};

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
    HirInstrObjectDrop(runtime::HirInstrObjectDrop),
    HirInstrObjectClone(runtime::HirInstrObjectClone),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirValue {
    pub kind: HirValueKind,
}

/// This module holds all the runtime instructions that are used by the compiler. These instructions
/// are not part of the language itself, but are used to implement the language.
///
/// The instructions are:
pub mod runtime {
    use asena_hir_derive::{hir_debug, hir_node};

    use super::{HirValue, HirValueId, HirValueKind};
    use crate::NameId;

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
    pub struct HirInstrObjectClone(pub HirValueId);

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_node(HirValue)]
    #[hir_debug]
    pub struct HirInstrObjectDrop(pub HirValueId);
}
