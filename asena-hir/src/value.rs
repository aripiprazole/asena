use asena_hir_derive::*;

use crate::expr::*;
use crate::interner::HirInterner;
use crate::stmt::*;

use self::{instr::HirInstr, monads::HirMonad};

pub mod instr;
pub mod monads;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
pub struct HirValueBlock {
    pub instructions: Vec<HirStmt>,
    pub value: HirValue,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
pub struct HirValueExpr(pub HirExpr);

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirValue)]
pub enum HirValueKind {
    #[default]
    Error,
    Unit,
    Block(HirValueBlock),
    Expr(HirValueExpr),
    Monad(HirMonad),
    Instr(HirInstr),
}

#[hir_struct]
pub struct HirValue {
    pub kind: HirValueKind,
}

impl HirValue {
    pub fn error(db: &dyn HirInterner) -> HirValue {
        db.intern_value(HirValueData {
            kind: HirValueKind::Error,
            span: Default::default(),
        })
    }

    pub fn unit(db: &dyn HirInterner) -> HirValue {
        db.intern_value(HirValueData {
            kind: HirValueKind::Unit,
            span: Default::default(),
        })
    }

    pub fn of_expr(db: &dyn HirInterner, expr: HirExpr) -> HirValue {
        let kind = HirValueKind::from(HirValueExpr(expr));

        db.intern_value(HirValueData {
            kind,
            span: Default::default(),
        })
    }
}
