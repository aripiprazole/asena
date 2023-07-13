use asena_hir_derive::*;

use crate::expr::*;
use crate::stmt::*;

use self::{instr::HirInstr, monads::HirMonad};

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
    HirValueUnit,
    HirValueBlock(HirValueBlock),
    HirValueExpr(HirValueExpr),
    HirMonad(HirMonad),
    HirInstr(HirInstr),
}

#[hir_struct]
pub struct HirValue {
    pub kind: HirValueKind,
}
pub mod instr;
pub mod monads;
