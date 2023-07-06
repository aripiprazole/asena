use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Range;
use std::path::PathBuf;
use std::rc::Rc;
use std::{error::Error, fmt::Display};

use asena_interner::Intern;
use asena_leaf::node::Tree;
use asena_span::{Loc, Spanned};

pub use Fragment::*;

pub trait InternalError: Error {
    fn code(&self) -> u16 {
        0
    }

    fn kind(&self) -> DiagnosticKind;
}

#[derive(Clone)]
pub struct BoxInternalError(pub Rc<dyn InternalError>);

impl Debug for BoxInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Display for BoxInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Position {
    After,
    Before,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fragment {
    Insert(String),
    Remove(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Quickfix {
    pub loc: Loc,
    pub position: Position,
    pub message: Vec<Fragment>,
}

#[derive(Debug, Clone)]
pub struct Report<T: InternalError> {
    pub path: Option<PathBuf>,
    pub source: String,
    pub tree: Intern<Spanned<Tree>>,
    pub diagnostics: Vec<Diagnostic<T>>,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<T: InternalError> {
    pub kind: DiagnosticKind,
    pub code: u16,
    pub message: Spanned<T>,
    pub quickfixes: Vec<Quickfix>,
    pub children: Vec<Diagnostic<T>>,
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
            && self.quickfixes == other.quickfixes
            && self.children == other.children
    }
}

impl<T: InternalError> Hash for Diagnostic<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let message = (
            &self.message.span,
            self.message.value.code(),
            self.message.value.kind(),
        );

        self.kind.hash(state);
        self.code.hash(state);
        message.hash(state);
        self.quickfixes.hash(state);
        self.children.hash(state);
    }
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
    Meta = 8,
}

impl<E: InternalError> Report<E> {
    pub fn new(source: &str, tree: Intern<Spanned<Tree>>) -> Self {
        Self {
            path: None,
            source: source.into(),
            tree,
            diagnostics: vec![],
        }
    }

    pub fn add_diagnostic(&mut self, message: Spanned<E>) -> &mut Diagnostic<E> {
        self.diagnostics.push(Diagnostic {
            kind: message.kind(),
            code: message.code(),
            message,
            children: vec![],
            quickfixes: vec![],
        });

        self.diagnostics.last_mut().unwrap()
    }

    pub fn dump(&mut self)
    where
        E: Clone,
    {
        println!("  -> Recovered `Concrete Syntax Tree`:");
        println!();
        println!("{:#?}", self.tree);
        println!();
        for diagnostic in self.diagnostics.iter() {
            diagnostic.dump(&self.source);
        }
    }
}

impl<E: InternalError> Diagnostic<E> {
    pub fn new(error: Spanned<E>) -> Self {
        Self {
            kind: error.kind(),
            code: error.code(),
            message: error,
            children: vec![],
            quickfixes: vec![],
        }
    }

    pub fn add_fixes(mut self, fixes: Vec<Quickfix>) -> Self {
        self.quickfixes.extend(fixes);
        self
    }

    pub fn add_child(mut self, message: Spanned<E>) -> Self {
        self.children.push(Diagnostic {
            kind: message.kind(),
            code: message.code(),
            message,
            children: vec![],
            quickfixes: vec![],
        });

        self
    }

    fn as_label(&self, colors: &mut ariadne::ColorGenerator) -> ariadne::Label {
        ariadne::Label::new(self.message.span.clone().into_ranged().unwrap_or_default())
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

        let mut builder = Report::<Range<usize>>::build(ReportKind::Error, (), 0)
            .with_code(format!("E{:03X}", self.code))
            .with_message(self.message.value().to_string());
        let mut colors = ColorGenerator::new();
        let mut children = vec![];
        children.push(self.clone());
        children.extend(self.children.clone());

        builder = builder.with_labels(
            children
                .iter()
                .map(|diagnostic| diagnostic.as_label(&mut colors)),
        );
        if !self.quickfixes.is_empty() {
            let mut fixes = vec![];
            for fix in &self.quickfixes.clone() {
                let loc = fix.loc.clone();
                let message = fix
                    .message
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                fixes.push(format!("{message} at {loc}"));
            }
            builder = builder.with_help(format!("Can be fixed by: {}", fixes.join("; ")))
        }
        builder
            .finish()
            .print(Source::from(source.clone()))
            .unwrap();
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Insert(code) => write!(f, "Insert `{}`", code),
            Remove(code) => write!(f, "Remove `{}`", code),
        }
    }
}

#[macro_export]
macro_rules! quickfix {
    (before, $loc:expr, [$($fragment:expr),*]) => {
        [Quickfix {
            loc: $loc.clone(),
            position: $crate::Position::Before,
            message: vec![$($fragment),*],
        }]
    };
    (after, $loc:expr, [$($fragment:expr),*]) => {
        [Quickfix {
            loc: $loc.clone(),
            position: $crate::Position::After,
            message: vec![$($fragment),*],
        }]
    };
}
