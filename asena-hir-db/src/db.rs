use asena_ast_db::db::AstDatabase;
use asena_hir::interner::HirInterner;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: HirInterner + AstDatabase {}
