use asena_hir_derive::*;

use asena_hir_leaf::{HirId, HirNode};

use crate::{pattern::HirPatternId, value::HirValueId, HirVisitor};

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirStmtId(usize);

impl HirId for HirStmtId {
    type Node = HirStmt;

    fn new(node: Self::Node) -> Self {
        node.id
    }
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirStmtAsk {
    pub pattern: HirPatternId,
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirStmtReturn {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug)]
#[hir_kind]
pub enum HirStmtKind {
    Error,

    Ask(HirStmtAsk),
    Return(HirStmtReturn),
    Value(HirValueId),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct]
pub struct HirStmt {
    pub span: asena_span::Loc,
    pub id: HirStmtId,
    pub kind: HirStmtKind,
}

impl HirNode for HirStmt {
    type Id = HirStmtId;
    type Visitor<'a, T> = dyn HirVisitor<T>;

    fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
        todo!()
    }
}
