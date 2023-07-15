use std::path::PathBuf;

use asena_report::{DiagnosticKind, InternalError};
use thiserror::Error;

use crate::*;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    #[error("module not found: `{0}`")]
    ModuleNotFoundError(FunctionId),

    #[error("file not found: `{0}`")]
    FileNotFoundError(PathBuf),
}

impl BuildError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl InternalError for BuildError {
    fn code(&self) -> u16 {
        self.discriminant() as u16
    }

    fn kind(&self) -> DiagnosticKind {
        DiagnosticKind::Error
    }
}
