use std::{hash::Hash, sync::RwLock};

use asena_ast_db::vfs::VfsPath;
use asena_leaf::ast::GreenTree;
use im::HashSet;

use crate::top_level::HirTopLevel;

#[derive(Default, Debug)]
pub struct InternalAsenaFile {
    pub path: VfsPath,
    pub content: String,
    pub tree: GreenTree,
    pub declarations: RwLock<HashSet<HirTopLevel>>,
}

impl Hash for InternalAsenaFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.content.hash(state);
        self.declarations.read().unwrap().hash(state);
    }
}

impl Clone for InternalAsenaFile {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            content: self.content.clone(),
            tree: self.tree.clone(),
            declarations: RwLock::new(self.declarations.read().unwrap().clone()),
        }
    }
}
