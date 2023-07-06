use asena_hir_derive::*;

use asena_hir_leaf::{HirId, HirNode};

use crate::{expr::HirExprId, HirVisitor};

#[derive(Hash, Copy, Clone, Debug)]
#[hir_id]
pub struct HirValueId(usize);

impl HirId for HirValueId {
    type Node = HirValue;

    fn new(node: Self::Node) -> Self {
        node.id
    }
}

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
#[hir_struct]
pub struct HirValue {
    pub span: asena_span::Loc,
    pub id: HirValueId,
    pub kind: HirValueKind,
}

impl HirNode for HirValue {
    type Id = HirValueId;
    type Visitor<'a, T> = dyn HirVisitor<T>;

    fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
        todo!()
    }
}
