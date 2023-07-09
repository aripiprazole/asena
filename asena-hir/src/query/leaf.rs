use std::{fmt::Debug, hash::Hash, sync::Arc};

pub type HirLoc = asena_span::Loc;

pub trait HirId: Debug + Copy + Hash {
    type Node: HirNode;

    fn new(node: Self::Node) -> Self;
}

pub trait HirNode: From<<Self::Id as HirId>::Node> {
    type Id: HirId<Node: Into<Self>>;
    type Visitor<'a, T>: ?Sized;

    fn hash_id(&self) -> Self::Id;

    fn accept<O: Default>(&mut self, visitor: &mut Self::Visitor<'_, O>) -> O;
}

pub trait HirInterned {
    type Id;
    type Database: ?Sized;

    fn interned(db: std::sync::Arc<Self::Database>, id: Self::Id) -> Arc<Self>;
}

pub trait HirLocated {
    fn location(&self) -> HirLoc;
}

pub struct HirBorrow<'a, N: HirNode> {
    pub node: &'a N,
}

pub trait HirBaseDatabase {}
