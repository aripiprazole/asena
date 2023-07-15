use asena_leaf::ast::Located;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub enum DiagnosticKind {
    Error = 1,
    HardError = 2,
    InternalError = 3,
    Warning = 4,
    Deprecated = 5,
    Info = 6,
    Tip = 7,
    Meta = 8,
    SyntaxError = 9,
    TypeError = 11,
    ResolutionError = 12,
    Lint = 13,
    LoweringError = 14,
    Context = 15,
    BuildError = 16,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<T> {
    pub kind: DiagnosticKind,
    pub code: u16,
    pub message: Spanned<T>,
    pub children: Vec<Diagnostic<T>>,
}

pub trait WithError {
    fn fail<E: InternalError>(self, error: E) -> Diagnostic<E>;
}

impl<T: Located> WithError for T {
    fn fail<E: InternalError>(self, error: E) -> Diagnostic<E> {
        Diagnostic::located(self, error)
    }
}

impl<E: InternalError> Diagnostic<E> {
    pub fn new(error: Spanned<E>) -> Self {
        Self {
            kind: error.kind(),
            code: error.code(),
            message: error,
            children: vec![],
        }
    }

    pub fn of(loc: Loc, error: E) -> Self {
        Self::new(Spanned::new(loc, error))
    }

    pub fn located<T: Located>(loc: T, error: E) -> Self {
        Self::new(Spanned::new(loc.location().into_owned(), error))
    }

    pub fn add_child(mut self, message: Spanned<E>) -> Self {
        self.children.push(Diagnostic {
            kind: message.kind(),
            code: message.code(),
            message,
            children: vec![],
        });

        self
    }
}

impl<T: InternalError> Eq for Diagnostic<T> {}

impl<T: InternalError> PartialEq for Diagnostic<T> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.code == other.code
            && self.message.span == other.message.span
            && self.message.value.code() == other.message.value.code()
            && self.message.value.kind() == other.message.value.kind()
            && self.message.value.to_string() == other.message.value.to_string()
            && self.children == other.children
    }
}

impl<T: InternalError> Hash for Diagnostic<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Build message hasher
        let message = &self.message;
        let code = message.value.code();
        let kind = message.value.kind();

        self.kind.hash(state);
        self.code.hash(state);
        message.span.hash(state);
        code.hash(state);
        kind.hash(state);
        self.children.hash(state);
    }
}
