use std::sync::{Arc, RwLock};

use asena_ast::db::ReporterDatabase;

use super::*;

#[salsa::query_group(PrecStorage)]
pub trait PrecDatabase: ReporterDatabase {
    #[salsa::input]
    fn prec_table(&self) -> Arc<RwLock<HashMap<FunctionId, Entry>>>;

    fn ordered_prec(&self, file: AsenaFile) -> AsenaFile;
    fn infix_commands(&self, file: AsenaFile) -> AsenaFile;
}

fn ordered_prec(db: &dyn PrecDatabase, file: AsenaFile) -> AsenaFile {
    file.walks(PrecReorder {
        prec_table: &db.prec_table().read().unwrap(),
        reporter: &db.reporter(),
    })
}

fn infix_commands(db: &dyn PrecDatabase, file: AsenaFile) -> AsenaFile {
    file.walks(InfixHandler {
        prec_table: &mut db.prec_table().write().unwrap(),
        reporter: &db.reporter(),
    })
}
