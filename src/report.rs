use std::error::Error;
use std::path::PathBuf;

use crate::ast::node::Tree;
use crate::lexer::span::{Loc, Spanned};

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

    pub fn dump(&mut self)
    where
        E: Clone,
    {
        for diagnostic in self.diagnostics.iter() {
            diagnostic.dump(&self.source);
        }

        println!();
        println!("  -> Recovered `Concrete Syntax Tree`:");
        println!();
        println!("{:#?}", self.tree);
    }
}

impl<E: InternalError> Diagnostic<E> {
    pub fn new(error: Spanned<E>) -> Self {
        Self {
            kind: E::kind(),
            code: error.code(),
            message: error,
            children: vec![],
        }
    }

    pub fn add_child(mut self, message: Spanned<E>) -> Self {
        self.children.push(Diagnostic {
            kind: E::kind(),
            code: message.code(),
            message,
            children: vec![],
        });

        self
    }

    fn as_label(&self, colors: &mut ariadne::ColorGenerator) -> ariadne::Label {
        ariadne::Label::new(self.message.span.clone())
            .with_message(self.message.value.to_string())
            .with_color(match self.kind {
                DiagnosticKind::Warning | DiagnosticKind::Deprecated => ariadne::Color::Yellow,
                DiagnosticKind::Info => ariadne::Color::Blue,
                DiagnosticKind::HardError
                | DiagnosticKind::Error
                | DiagnosticKind::InternalError => ariadne::Color::Red,
                _ => colors.next(),
            })
    }

    fn dump(&self, source: &str)
    where
        E: Clone,
    {
        use ariadne::{ColorGenerator, Report, ReportKind, Source};

        let mut colors = ColorGenerator::new();
        let mut children = vec![];
        children.push(self.clone());
        children.extend(self.children.clone());

        Report::<Loc>::build(ReportKind::Error, (), 0)
            .with_code(format!("E{:.3X}", self.code))
            .with_message(self.message.value().to_string())
            .with_labels(
                children
                    .iter()
                    .map(|diagnostic| diagnostic.as_label(&mut colors)),
            )
            .finish()
            .print(Source::from(source.clone()))
            .unwrap();
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
