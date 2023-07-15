use asena_ast_db::{commands::CommandHandlerEval, db::AstDatabase};
use asena_leaf::ast::AstParam;

use super::*;

#[salsa::query_group(PrecStorage)]
pub trait PrecDatabase: AstDatabase {
    fn prec_table(&self) -> PrecTable;

    fn ordered_prec(&self, file: AstParam<AsenaFile>) -> AsenaFile;
    fn infix_commands(&self, file: AstParam<AsenaFile>) -> AsenaFile;
}

fn prec_table(_db: &dyn PrecDatabase) -> PrecTable {
    PrecTable::default()
}

fn ordered_prec(db: &dyn PrecDatabase, file: AstParam<AsenaFile>) -> AsenaFile {
    file.data.walks(PrecReorder { db })
}

fn infix_commands(db: &dyn PrecDatabase, file: AstParam<AsenaFile>) -> AsenaFile {
    let mut handler = InfixHandler::new(db);
    let eval = CommandHandlerEval::new(db, &mut handler);
    file.data.walks(eval)
}
