#![feature(type_changing_struct_update)]
use std::fmt::Debug;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::{error::Error, fmt::Display};

use asena_leaf::node::Tree;
use asena_span::{Loc, Spanned};

pub use errors::*;
pub use ide_diagnostic::*;
pub use quickfix::*;
pub use report::*;

pub use Fragment::*;

pub mod errors;
pub mod ide_diagnostic;
pub mod quickfix;
pub mod report;

pub trait Reports {
    fn errors(&self) -> Arc<RwLock<Vec<Diagnostic<BoxInternalError>>>>;

    #[track_caller]
    fn diagnostic<E>(&self, diagnostic: Diagnostic<E>)
    where
        E: Clone + Send + Sync + InternalError + 'static,
    {
        let errors = self.errors();
        let mut errors = errors.try_write().unwrap();
        errors.push(Diagnostic {
            message: diagnostic.message.map(BoxInternalError::new),
            kind: diagnostic.kind,
            code: diagnostic.code,
            children: vec![],
        });
    }
}
