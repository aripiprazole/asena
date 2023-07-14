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
    fn reports(&mut self) -> &Reporter;
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

pub struct ContextBuilder {
    pub contexts: Vec<Spanned<BoxInternalError>>,
}

impl ContextBuilder {
    pub fn push<E: InternalError + 'static>(&mut self, at: &dyn Located, error: E) {
        let item = Spanned::new(at.location().into_owned(), ());
        let error = BoxInternalError(Rc::new(error));

        self.contexts.push(item.replace(error));
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

    pub fn report_with<F, E: InternalError + 'static, T: Located>(&self, at: &T, error: F)
    where
        F: FnOnce(&mut ContextBuilder) -> E,
    {
        let mut builder = ContextBuilder { contexts: vec![] };
        let error = error(&mut builder);
        let item = Spanned::new(at.location().into_owned(), ());
        let mut diagnostic = Diagnostic::new(item.replace(BoxInternalError(Rc::new(error))));

        for context in builder.contexts {
            diagnostic = diagnostic.add_child(context);
        }

        self.errors.lock().unwrap().insert(diagnostic);
    }

    pub fn diagnostic<E: InternalError + 'static, T>(&self, at: Spanned<T>, error: E) {
        self.errors.lock().unwrap().insert(Diagnostic::new(
            at.replace(BoxInternalError(Rc::new(error))),
        ));
    }

    pub fn dump(&self) {
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
