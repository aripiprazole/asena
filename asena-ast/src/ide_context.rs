use asena_report::{DiagnosticKind, InternalError};
use thiserror::Error;

use crate::FunctionId;

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdeContext {
    #[error("in class declaration {name:?}")]
    InClass { name: FunctionId },

    #[error("in equation {name:?}")]
    InEquation { name: FunctionId },
}

impl InternalError for IdeContext {
    fn kind(&self) -> DiagnosticKind {
        DiagnosticKind::Context
    }
}
