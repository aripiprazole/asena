use std::{fmt::Debug, hash::Hash, sync::Arc};

use crate::file::InternalAsenaFile;

#[derive(Default, Debug, Clone)]
pub struct HirLoc {
    pub file: Arc<InternalAsenaFile>,
    pub location: asena_span::Loc,
}

impl HirLoc {
    pub fn new(file: Arc<InternalAsenaFile>, location: asena_span::Loc) -> Self {
        Self { file, location }
    }
}

impl Hash for HirLoc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.location.hash(state);
    }
}

impl Eq for HirLoc {}

impl PartialEq for HirLoc {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.file, &other.file) && self.location == other.location
    }
}

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
