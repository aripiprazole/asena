use std::{rc::Rc, sync::Mutex};

use asena_leaf::{ast::Located, node::Tree};
use asena_report::{BoxInternalError, Diagnostic, InternalError, Report};
use asena_span::Spanned;
use im::HashSet;

#[derive(Default, Debug)]
pub struct Reporter {
    pub src: String,
    pub tree: Spanned<Tree>,
    pub(crate) errors: Mutex<HashSet<Diagnostic<BoxInternalError>>>,
}

pub trait Reports {
    fn reports(&mut self) -> &mut Reporter;
}

impl Clone for Reporter {
    fn clone(&self) -> Self {
        Self {
            src: self.src.clone(),
            tree: self.tree.clone(),
            errors: Mutex::new(self.errors.lock().unwrap().clone()),
        }
    }
}

impl Reporter {
    pub fn new(src: &str, tree: Spanned<Tree>) -> Self {
        Self {
            src: src.to_owned(),
            tree,
            ..Default::default()
        }
    }

    pub fn report<E: InternalError + 'static, T: Located>(&self, at: &T, error: E) {
        self.diagnostic(Spanned::new(at.location().into_owned(), ()), error);
    }

    pub fn diagnostic<E: InternalError + 'static, T>(&self, at: Spanned<T>, error: E) {
        self.errors.lock().unwrap().insert(Diagnostic::new(
            at.replace(BoxInternalError(Rc::new(error))),
        ));
    }

    pub fn dump(&mut self) {
        if self.errors.lock().unwrap().is_empty() {
            return;
        }

        let mut report = Report::<BoxInternalError>::new(&self.src, self.tree.clone());
        report.diagnostics = self.errors.lock().unwrap().iter().cloned().collect();
        report.dump();
    }

    pub fn dump_tree(&mut self) {
        let mut report = Report::<BoxInternalError>::new(&self.src, self.tree.clone());
        report.dump();
    }
}
