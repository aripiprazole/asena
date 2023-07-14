use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Range;
use std::path::PathBuf;
use std::rc::Rc;
use std::{error::Error, fmt::Display};

use ariadne::{Color, Config, LabelAttach};
use asena_leaf::node::Tree;
use asena_span::{Loc, Spanned};

pub use ide_diagnostic::*;
pub use Fragment::*;

pub mod ide_diagnostic;

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
    pub tree: Spanned<Tree>,
    pub diagnostics: Vec<Diagnostic<T>>,
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
        // println!("  -> Recovered `Concrete Syntax Tree`:");
        // println!();
        // println!("{:#?}", self.tree);
        // println!();
        for diagnostic in self.diagnostics.iter() {
            diagnostic.dump(&self.source);
        }
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
