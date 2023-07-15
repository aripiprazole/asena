use asena_ast::AsenaFile;
use asena_ast_db::db::AstDatabase;
use asena_leaf::ast::{AstParam, Located};

use crate::decl::AstResolver;

#[salsa::query_group(AstResolverStorage)]
pub trait AstResolverDatabase: AstDatabase {
    fn ast_resolved_file(&self, file: AstParam<AsenaFile>) -> AsenaFile;
}

fn ast_resolved_file(db: &dyn AstResolverDatabase, ast: AstParam<AsenaFile>) -> AsenaFile {
    let module = db.location_file(ast.location().into_owned());
    let file = db.vfs_file(module);
    ast.data.walks(AstResolver {
        db,
        file,
        binding_groups: Default::default(),
        class_declarations: Default::default(),
        enum_declarations: Default::default(),
        instance_declarations: Default::default(),
        trait_declarations: Default::default(),
    })
}
