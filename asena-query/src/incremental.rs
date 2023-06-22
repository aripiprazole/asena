use std::path::PathBuf;

use asena_ast::AsenaFile;

use crate::graph::Graph;
use crate::node::Declaration;

pub fn query_file_path(_db: &Graph, _declaration: &Declaration) -> Option<PathBuf> {
    PathBuf::new().into()
}

pub fn query_ast(_db: &Graph, declaration: &Declaration) -> AsenaFile {
    println!("[Query -- AST] {}", declaration.name);

    AsenaFile::default()
}
