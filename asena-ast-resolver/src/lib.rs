use std::sync::Arc;

use asena_ast::{reporter::Reporter, AsenaVisitor, Decl, FunctionId};
use im::HashMap;

#[derive(Hash)]
pub struct CanonicalPath {
    pub path: String,
}

#[derive(Hash)]
pub enum ModuleRef {
    NotFound,
    Found(CanonicalPath),
}

pub struct VfsFile {
    pub id: FunctionId,
}

pub trait AstDatabase {
    fn get_module(&self, path: &str) -> ModuleRef;
    fn get_file(&self, module: ModuleRef) -> Arc<VfsFile>;
    fn items(&self, module: FunctionId) -> Arc<HashMap<FunctionId, Decl>>;
    fn resolve_ast(&self, vfs_file: Arc<VfsFile>) -> Arc<asena_ast::AsenaFile>;
}

pub struct AstResolver<'a> {
    pub db: Arc<dyn AstDatabase>,
    pub imports: Vec<CanonicalPath>,
    pub canonical_paths: HashMap<FunctionId, CanonicalPath>,
    pub reporter: &'a mut Reporter,
}

pub struct NonResolvingAstDatabase {}

#[derive(Hash)]
pub struct GetFileQuery(ModuleRef);

impl AstDatabase for NonResolvingAstDatabase {
    fn get_module(&self, path: &str) -> ModuleRef {
        ModuleRef::NotFound
    }

    fn get_file(&self, path: ModuleRef) -> Arc<VfsFile> {
        todo!()
    }

    fn items(&self, module: FunctionId) -> Arc<HashMap<FunctionId, Decl>> {
        todo!()
    }

    fn resolve_ast(&self, vfs_file: Arc<VfsFile>) -> Arc<asena_ast::AsenaFile> {
        todo!()
    }
}

impl AsenaVisitor<()> for AstResolver<'_> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.get_module(value.path().to_fn_id().as_str());
        let file = self.db.get_file(module_ref);
        self.db.items(file.id.clone());
    }

    fn visit_qualified_path(&mut self, qualified_path: asena_ast::QualifiedPath) {
        println!("visit_qualified_path: {:?}", qualified_path);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asena_ast::AsenaFile;
    use asena_grammar::parse_asena_file;
    use asena_leaf::ast::Node;
    use asena_prec::{default_prec_table, InfixHandler, PrecReorder};

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();

        let mut tree = parse_asena_file!("./test.ase");

        let file = AsenaFile::new(tree.clone())
            .walks(InfixHandler {
                prec_table: &mut prec_table,
                reporter: &mut tree.reporter,
            })
            .walks(PrecReorder {
                prec_table: &prec_table,
                reporter: &mut tree.reporter,
            })
            .walks(super::AstResolver {
                imports: Vec::new(),
                db: Arc::new(super::NonResolvingAstDatabase {}),
                canonical_paths: Default::default(),
                reporter: &mut tree.reporter,
            });

        tree.reporter.dump();

        println!("{file:#?}");
    }
}
