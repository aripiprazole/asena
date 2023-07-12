use asena_report::{DiagnosticKind, InternalError};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AstLoweringError {
    #[error("not resolved")]
    NotResolved,

    #[error("duplicate signature definition")]
    DuplicatedSignatureDefinitionError,

    #[error("duplicate method definition")]
    DuplicatedMethodDefinitionError,

    #[error("duplicate abstract field definition")]
    DuplicatedAbstractFieldDefinitionError,

    #[error("variant parameter type can not be infer")]
    VariantParameterCanNotBeInferError,

    #[error("variant type can not be infer")]
    VariantTypeCanNotBeInferError,

    #[error("field type can not be infer")]
    FieldTypeCanNotBeInferError,

    #[error("self parameter can not be implicit")]
    SelfParameterBayMeExplicitError,

    #[error("dependent types aren't supported yet")]
    UnsupportedDependentTypesError,

    #[error("type literals aren't supported yet")]
    UnsupportedTypeLiteralsError,

    #[error("type exporessions aren't supported yet")]
    UnsupportedTypeExprsError,
}

impl AstLoweringError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl InternalError for AstLoweringError {
    fn code(&self) -> u16 {
        self.discriminant() as u16
    }

    fn kind(&self) -> DiagnosticKind {
        DiagnosticKind::LoweringError
    }
}
