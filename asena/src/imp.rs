use asena_ast_db::db::AstDatabaseStorage;
use asena_ast_lowering::db::AstLowerrerStorage;
use asena_hir::interner::HirStorage;
use asena_hir_db::db::HirDatabaseStorage;
use asena_prec::db::PrecStorage;
use std::sync::Mutex;

#[salsa::database(
    PrecStorage,
    AstDatabaseStorage,
    HirDatabaseStorage,
    AstLowerrerStorage,
    HirStorage
)]
#[derive(Default)]
pub struct DatabaseImpl {
    pub storage: salsa::Storage<DatabaseImpl>,
    pub logs: Mutex<Vec<salsa::Event>>,
}

impl salsa::Database for DatabaseImpl {
    fn salsa_event(&self, event_fn: salsa::Event) {
        self.logs.lock().unwrap().push(event_fn);
    }
}
