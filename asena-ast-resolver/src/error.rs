use crate::*;

#[derive(Default, Error, Debug, Clone, PartialEq, Eq)]
pub enum ResolutionError {
    #[error("Not resolved")]
    #[default]
    NotResolved,

    #[error("unresolved import: `{0}`")]
    UnresolvedImportError(FunctionId),

    #[error("could not find the value name: `{0}`")]
    UnresolvedNameError(FunctionId),

    #[error("could not find the type name: `{0}`")]
    UnresolvedTypeNameError(FunctionId),

    #[error("could not find the type constructor: `{0}`")]
    UnresolvedConstructorError(FunctionId),
}

impl ResolutionError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl InternalError for ResolutionError {
    fn code(&self) -> u16 {
        self.discriminant() as u16
    }

    fn kind(&self) -> asena_report::DiagnosticKind {
        asena_report::DiagnosticKind::Error
    }
}
