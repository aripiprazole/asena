use asena_ast::{command::Result, AsenaVisitor, Command};
use asena_report::WithError;

use crate::db::AstDatabase;
use crate::package::HasDiagnostic;

pub trait CommandHandler {
    fn on_command(&mut self, value: Command) -> Result {
        let _ = value;
        Ok(())
    }
}

pub struct CommandHandlerEval<'db, 'handler> {
    pub db: &'db dyn AstDatabase,
    pub handler: &'handler mut dyn CommandHandler,
}

impl<'db, 'handler> CommandHandlerEval<'db, 'handler> {
    pub fn new(db: &'db dyn AstDatabase, handler: &'handler mut dyn CommandHandler) -> Self {
        Self { db, handler }
    }
}

impl<'db, 'handler> AsenaVisitor<()> for CommandHandlerEval<'db, 'handler> {
    fn visit_command(&mut self, value: Command) {
        let name = value.name();

        match self.handler.on_command(value) {
            Ok(()) => {}
            Err(err) => name.fail(err).push(self.db),
        }
    }
}
