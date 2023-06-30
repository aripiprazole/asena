use std::rc::Rc;

use asena_leaf::{ast::Located, node::Tree};
use asena_report::{BoxInternalError, Diagnostic, InternalError, Report};
use asena_span::Spanned;

use crate::*;

pub trait TreeWalker: ExprWalker + PatWalker + StmtWalker {}

pub trait Reporter {
    fn report<E: InternalError + 'static, T: Located>(&mut self, at: &T, error: E) {
        self.diagnostic(Spanned::new(at.location().into_owned(), ()), error);
    }

    fn diagnostic<E: InternalError + 'static, T>(&mut self, at: Spanned<T>, error: E);
}

#[derive(Default, Clone)]
pub struct DefaultReporter {
    pub src: String,
    pub tree: Spanned<Tree>,
    pub(crate) errors: Vec<Diagnostic<BoxInternalError>>,
}

impl Reporter for DefaultReporter {
    fn diagnostic<E: InternalError, T>(&mut self, at: Spanned<T>, error: E)
    where
        E: 'static,
    {
        let error = at.swap(BoxInternalError(Rc::new(error)));

        self.errors.push(Diagnostic::new(error))
    }
}

impl DefaultReporter {
    pub fn new(src: &str, tree: Spanned<Tree>) -> Self {
        Self {
            src: src.to_owned(),
            tree,
            ..Default::default()
        }
    }

    pub fn dump(&mut self) {
        if self.errors.is_empty() {
            return;
        }

        let mut report = Report::<BoxInternalError>::new(&self.src, self.tree.clone());
        report.diagnostics = self.errors.clone();
        report.dump();
    }

    pub fn dump_tree(&mut self) {
        let mut report = Report::<BoxInternalError>::new(&self.src, self.tree.clone());
        report.dump();
    }
}
