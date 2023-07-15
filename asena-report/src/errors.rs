use super::*;

pub trait InternalError: Error {
    fn code(&self) -> u16 {
        0
    }

    fn kind(&self) -> DiagnosticKind;
}

#[derive(Clone)]
pub struct BoxInternalError(pub Rc<dyn InternalError>);

impl BoxInternalError {
    pub fn new<E: InternalError + 'static>(error: E) -> Self {
        Self(Rc::new(error))
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
