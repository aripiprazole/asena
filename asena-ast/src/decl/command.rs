use asena_report::{DiagnosticKind, InternalError, Reports};
use thiserror::Error;

use crate::{visitor::AsenaVisitor, *};

pub type Result<T = ()> = std::result::Result<T, CommandError>;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("invalid argument type: expected {expected_node_type}")]
    InvalidArgument { expected_node_type: &'static str },
}

impl Command {
    /// Checks if the command is the given name.
    pub fn is_command(&self, name: &str) -> bool {
        self.name().to_fn_id() == FunctionId::new(name)
    }

    pub fn at<T: TryFrom<Expr>>(&self, nth: usize) -> Result<T> {
        self.arguments()
            .get(nth)
            .cloned()
            .and_then(|x| x.try_into().ok())
            .ok_or_else(|| CommandError::InvalidArgument {
                expected_node_type: std::any::type_name::<T>(),
            })
    }
}

impl InternalError for CommandError {
    fn kind(&self) -> DiagnosticKind {
        DiagnosticKind::Meta
    }
}

pub trait CommandHandler: AsenaVisitor<()> {
    fn on_command(&mut self, value: Command) -> Result {
        let _ = value;
        Ok(())
    }
}

impl<T: CommandHandler + Reports> AsenaVisitor<()> for T {
    fn visit_command(&mut self, value: Command) {
        let name = value.find_name();

        match self.on_command(value) {
            Ok(()) => {}
            Err(err) => todo!(),
        }
    }
}
