use std::{hash::Hash, sync::Arc};

use asena_ast_db::ModuleRef;
use asena_leaf::ast::GreenTree;
use im::HashSet;

use crate::top_level::HirTopLevel;

#[derive(Default, Debug, Clone)]
pub struct InternalAsenaFile {
    pub path: ModuleRef,
    pub content: Arc<String>,
    pub tree: GreenTree,
    pub declarations: HashSet<HirTopLevel>,
}

impl Eq for InternalAsenaFile {}

impl PartialEq for InternalAsenaFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for InternalAsenaFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.content.hash(state);
    }
}
