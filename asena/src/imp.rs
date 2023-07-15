use asena_ast_db::{
    db::{AstDatabase, AstDatabaseStorage},
    vfs::VfsFile,
};
use asena_ast_lowering::db::{AstLowerrer, AstLowerrerStorage};
use asena_ast_resolver::db::{AstResolverDatabase, AstResolverStorage};
use asena_hir::interner::HirStorage;
use asena_hir_db::db::HirDatabaseStorage;
use asena_prec::{db::PrecStorage, PrecDatabase};
use std::{panic::AssertUnwindSafe, sync::Mutex};

#[salsa::database(
    PrecStorage,
    AstDatabaseStorage,
    HirDatabaseStorage,
    AstLowerrerStorage,
    AstResolverStorage,
    HirStorage
)]
#[derive(Default)]
pub struct DatabaseImpl {
    pub storage: salsa::Storage<DatabaseImpl>,
    pub logs: Mutex<Vec<salsa::Event>>,
}

impl DatabaseImpl {
    pub fn run_pipeline_catching(&self, file: VfsFile) {
        let db = AssertUnwindSafe(self);
        let result = std::panic::catch_unwind(|| {
            let file = db.ast(file);
            let file = db.infix_commands(file);
            let file = db.ordered_prec(file);
            let file = db.ast_resolved_file(file);
            let _hir = db.hir_file(file);
        });

        match result {
            Ok(value) => value,
            Err(err) => {
                eprintln!("compiler panicked during pipeline");
                eprintln!();
                self.dump_and_write_logs();
                std::panic::resume_unwind(err);
            }
        }
    }

    pub fn dump_and_write_logs(&self) {
        use salsa::EventKind::*;

        let logs: Vec<salsa::Event> = self.logs.lock().unwrap().drain(..).collect();
        for event in logs {
            match event.kind {
                WillBlockOn {
                    other_runtime_id,
                    database_key,
                } => {
                    let key = database_key.debug(self);
                    let runtime_id = other_runtime_id;

                    log::debug!("will block on {:?} (runtime id: {:?})", key, runtime_id);
                }
                WillExecute { database_key } => {
                    log::info!("will execute {:?}", database_key.debug(self));
                }
                WillCheckCancellation => {
                    log::debug!("will check cancellation");
                }
                DidValidateMemoizedValue { database_key } => {
                    log::debug!("did validate memoized value {:?}", database_key.debug(self));
                }
            }
        }
    }
}

impl salsa::Database for DatabaseImpl {
    fn salsa_event(&self, event_fn: salsa::Event) {
        self.logs.lock().unwrap().push(event_fn);
    }
}

impl salsa::ParallelDatabase for DatabaseImpl {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(DatabaseImpl {
            storage: self.storage.snapshot(),
            logs: Mutex::new(Vec::new()),
        })
    }
}

impl std::panic::UnwindSafe for DatabaseImpl {}
