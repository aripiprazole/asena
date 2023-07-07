use asena_hir_derive::*;

use crate::{expr::HirExprId, HirVisitor};

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirBlock {
    pub instructions: Vec<HirValueId>,
}

#[derive(Hash, Clone, Debug)]
#[hir_kind]
pub enum HirValueKind {
    Error,
    Block(HirBlock),
    Expr(HirExprId),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirValue {
    pub span: asena_span::Loc,
    pub id: HirValueId,
    pub kind: HirValueKind,
}
