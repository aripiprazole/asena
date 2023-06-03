use std::error::Error;
use std::path::PathBuf;

use crate::ast::node::Tree;
use crate::lexer::span::Spanned;

pub trait InternalError: Error {
    fn code(&self) -> u16 {
        0
    }

    fn kind() -> DiagnosticKind;
}

#[derive(Debug, Clone)]
pub struct Report<T: InternalError> {
    pub path: Option<PathBuf>,
    pub source: String,
    pub tree: Spanned<Tree>,
    pub diagnostics: Vec<Diagnostic<T>>,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<T: InternalError> {
    pub kind: DiagnosticKind,
    pub code: u16,
    pub message: Spanned<T>,
    pub children: Vec<Diagnostic<T>>,
}

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
}

impl<E: InternalError> Report<E> {
    pub fn new(source: &str, tree: Spanned<Tree>) -> Self {
        Self {
            path: None,
            source: source.into(),
            tree,
            diagnostics: vec![],
        }
    }

    pub fn add_diagnostic(&mut self, message: Spanned<E>) -> &mut Diagnostic<E> {
        self.diagnostics.push(Diagnostic {
            kind: E::kind(),
            code: message.code(),
            message,
            children: vec![],
        });

        self.diagnostics.last_mut().unwrap()
    }

    pub fn dump(&mut self) {
        todo!()
    }
}

impl<E: InternalError> Diagnostic<E> {
    pub fn add_child(mut self, message: Spanned<E>) -> Self {
        self.children.push(Diagnostic {
            kind: E::kind(),
            code: message.code(),
            message,
            children: vec![],
        });

        self.children.last_mut().unwrap();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::node::TreeKind, parser::error::ParseError};

    use super::*;

    #[test]
    fn it_works() {
        let mut report = Report::new("", Spanned::new(0..0, Tree::new(TreeKind::Error)));
        report.add_diagnostic(Spanned::new(0..0, ParseError::EofError));
        report.add_diagnostic(Spanned::new(0..0, ParseError::EofError));
        report.add_diagnostic(Spanned::new(0..0, ParseError::EofError));
        report.dump();
    }
}
