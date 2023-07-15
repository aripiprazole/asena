use asena_ast_db::db::AstDatabase;

use super::*;

#[salsa::query_group(PrecStorage)]
pub trait PrecDatabase: AstDatabase {
    fn prec_table(&self) -> PrecTable;

    fn ordered_prec(&self, file: AsenaFile) -> AsenaFile;
    fn infix_commands(&self, file: AsenaFile) -> AsenaFile;
}

fn prec_table(_db: &dyn PrecDatabase) -> PrecTable {
    PrecTable::default()
}

fn ordered_prec(db: &dyn PrecDatabase, file: AsenaFile) -> AsenaFile {
    file.walks(PrecReorder { db })
}

fn infix_commands(db: &dyn PrecDatabase, file: AsenaFile) -> AsenaFile {
    file.walks(InfixHandler::new(db))
}
