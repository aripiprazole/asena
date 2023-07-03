use std::sync::Arc;

use asena_ast::{reporter::Reporter, AsenaVisitor, FunctionId};
use asena_ast_db::{driver::Driver, CanonicalPath};
use im::HashMap;

pub struct AstResolver<'a> {
    pub db: Driver,
    pub current_file: Arc<asena_ast_db::VfsFile>,
    pub imports: Vec<CanonicalPath>,
    pub canonical_paths: HashMap<FunctionId, CanonicalPath>,
    pub reporter: &'a mut Reporter,
}

impl AsenaVisitor<()> for AstResolver<'_> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.module_ref(value.path().to_fn_id().as_str());

        self.db.add_path_dep(self.current_file.clone(), module_ref);
    }

    fn visit_qualified_path(&mut self, qualified_path: asena_ast::QualifiedPath) {
        println!("visit_qualified_path: {:?}", qualified_path);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asena_ast::reporter::Reporter;
    use asena_ast_db::{
        driver::Driver, implementation::NonResolvingAstDatabase, FileSystem, PackageData, VfsFile,
    };
    use asena_prec::{default_prec_table, InfixHandler, PrecReorder};

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();

        let vfs = Arc::new(FileSystem::default());
        let db = Driver(Arc::new(NonResolvingAstDatabase::default()));
        let local_pkg = PackageData::new(&db, "Local", "0.0.0", Arc::new(Default::default()));
        let current_file = VfsFile::new(&db, &vfs, local_pkg, "Test", "./Test.ase".into());
        // stub files
        VfsFile::new(&db, &vfs, local_pkg, "Nat", "./Nat.ase".into());
        VfsFile::new(&db, &vfs, local_pkg, "IO", "./IO.ase".into());

        let mut reporter = Reporter::default();

        let file = db
            .abstract_syntax_tree(current_file.clone())
            .arc_walks(InfixHandler {
                prec_table: &mut prec_table,
                reporter: &mut reporter,
            })
            .arc_walks(PrecReorder {
                prec_table: &prec_table,
                reporter: &mut reporter,
            })
            .arc_walks(super::AstResolver {
                db,
                current_file: current_file.clone(),
                imports: Vec::new(),
                canonical_paths: Default::default(),
                reporter: &mut reporter,
            });

        reporter.dump();

        println!("{:#?}", current_file);

        // println!("{file:#?}");
    }
}
