use super::*;

pub trait InternalError: Error {
    fn code(&self) -> u16 {
        0
    }

    fn kind(&self) -> DiagnosticKind;
}

#[derive(Clone)]
pub struct BoxInternalError(pub Arc<dyn InternalError + Send + Sync>);

impl BoxInternalError {
    pub fn new<E: InternalError + Send + Sync + 'static>(error: E) -> Self {
        Self(Arc::new(error))
    }
}

impl Debug for BoxInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for BoxInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Error for BoxInternalError {}

impl InternalError for BoxInternalError {
    fn code(&self) -> u16 {
        self.0.code()
    }

    fn kind(&self) -> DiagnosticKind {
        self.0.kind()
    }
}
