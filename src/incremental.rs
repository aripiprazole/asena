use std::path::PathBuf;

use crate::{
    ast::{green::GreenTree, AsenaFile},
    graph::{Declaration, Graph},
};

pub fn query_file_path(_db: &Graph, _declaration: &Declaration) -> Option<PathBuf> {
    PathBuf::new().into()
}

pub fn query_ast(_db: &Graph, declaration: &Declaration) -> AsenaFile {
    println!("[Query -- AST] {}", declaration.name);

    AsenaFile::new(GreenTree::default())
}
