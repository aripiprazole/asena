use std::{
    fmt::Debug,
    hash::Hash,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use asena_ast::{AsenaFile, Decl};
use asena_leaf::spec::Spec;
use asena_span::Spanned;

use crate::graph::Key;

pub struct Vertex {
    pub name: String,
    pub declaration: RwLock<Option<Declaration>>,

    pub successors: RwLock<Vec<Key>>,
    pub predecessors: RwLock<Vec<Key>>,
}

#[derive(Clone)]
pub struct Declaration {
    pub name: String,
    pub file: Option<PathBuf>,

    pub file_leaf: Arc<asena_leaf::spec::Node<AsenaFile>>,
    pub current_leaf: Arc<asena_leaf::spec::Node<Spanned<Decl>>>,

    /// Recompile flag, if its true, all the other fields will be recompiled
    pub recompile: bool,
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            name: Default::default(),
            file: None,
            file_leaf: Arc::new(AsenaFile::new(Default::default()).into()),
            current_leaf: Arc::new(Decl::make(Default::default())),
            recompile: false,
        }
    }
}

impl Vertex {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            declaration: Default::default(),
            successors: Default::default(),
            predecessors: Default::default(),
        }
    }

    pub fn key(&self) -> Key {
        Key(fxhash::hash(&self))
    }
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Hash for Vertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
