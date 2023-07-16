use asena_ast_db::{
    db::{AstDatabase, AstDatabaseStorage},
    vfs::VfsFile,
};
use asena_ast_lowering::db::{AstLowerrer, AstLowerrerStorage};
use asena_ast_resolver::db::{AstResolverDatabase, AstResolverStorage};
use asena_hir::interner::HirStorage;
use asena_hir_db::db::HirDatabaseStorage;
use asena_prec::{db::PrecStorage, PrecDatabase};
use std::{
    fmt::Debug,
    panic::{resume_unwind, AssertUnwindSafe},
    sync::Mutex,
};

#[salsa::database(
    PrecStorage,
    AstDatabaseStorage,
    HirDatabaseStorage,
    AstLowerrerStorage,
    AstResolverStorage,
    HirStorage
)]
#[derive(Default)]
pub struct IdeDatabase {
    pub storage: salsa::Storage<IdeDatabase>,
    pub logs: Mutex<Vec<salsa::Event>>,
}

impl IdeDatabase {
    pub fn run_pipeline_catching(&self, vfs_file: VfsFile) {
        let db = AssertUnwindSafe(self);
        let result = std::panic::catch_unwind(|| {
            let file = db.ast(vfs_file);
            let file = db.infix_commands(file.into());
            let file = db.ordered_prec(file.into());
            let file = db.ast_resolved_file(file.into());
            let _hir = db.hir_file(file.into());
        });

        match result {
            Ok(value) => value,
            Err(err) => {
                eprintln!("pipeline of the compiler during lowering:");
                db.dump_and_write_logs();
                resume_unwind(err);
            }
        }
    }

    pub fn dump_and_write_logs(&self) {
        use salsa::EventKind::*;

        let mut i = 0;
        let vec = self.logs.lock().unwrap();
        while let Some(event) = vec.get(i) {
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
                    let debug = database_key.debug(self);
                    let mut count = 0;
                    loop {
                        i += 1;
                        let next = vec.get(i).map(|a| &a.kind);
                        match next {
                            Some(WillExecute { database_key }) => {
                                let next = database_key.debug(self);
                                if format!("{next:?}") == format!("{debug:?}") {
                                    count += 1;
                                    continue;
                                } else {
                                    break;
                                }
                            }
                            Some(_) => {}
                            None => break,
                        }
                    }

                    if let 0 = count {
                        println!("  -> {debug:?}");
                    } else {
                        println!("  -> {debug:?} x{count}");
                    }
                }
                WillCheckCancellation => {
                    log::debug!("will check cancellation");
                }
                DidValidateMemoizedValue { database_key } => {
                    log::debug!("did validate memoized value {:?}", database_key.debug(self));
                }
            }

            i += 1;
        }
    }
}

impl salsa::Database for IdeDatabase {
    fn salsa_event(&self, event_fn: salsa::Event) {
        self.logs.lock().unwrap().push(event_fn);
    }
}

impl salsa::ParallelDatabase for IdeDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(IdeDatabase {
            storage: self.storage.snapshot(),
            logs: Mutex::new(Vec::new()),
        })
    }
}

impl std::panic::UnwindSafe for IdeDatabase {}

impl Debug for IdeDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdeDatabase").finish_non_exhaustive()
    }
}
