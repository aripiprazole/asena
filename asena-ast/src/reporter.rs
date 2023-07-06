use std::rc::Rc;

use asena_interner::Intern;
use asena_leaf::{ast::Located, node::Tree};
use asena_report::{BoxInternalError, Diagnostic, InternalError, Report};
use asena_span::Spanned;
use im::HashSet;

#[derive(Default, Clone)]
pub struct Reporter {
    pub src: String,
    pub tree: Intern<Spanned<Tree>>,
    pub(crate) errors: HashSet<Diagnostic<BoxInternalError>>,
}

pub trait Reports {
    fn reports(&mut self) -> &mut Reporter;
}

impl Reporter {
    pub fn new(src: &str, tree: Intern<Spanned<Tree>>) -> Self {
        Self {
            src: src.to_owned(),
            tree,
            ..Default::default()
        }
    }

    pub fn report<E: InternalError + 'static, T: Located>(&mut self, at: &T, error: E) {
        self.diagnostic(Spanned::new(at.location().into_owned(), ()), error);
    }

    pub fn diagnostic<E: InternalError + 'static, T>(&mut self, at: Spanned<T>, error: E) {
        self.errors.insert(Diagnostic::new(
            at.replace(BoxInternalError(Rc::new(error))),
        ));
    }

    pub fn dump(&mut self) {
        if self.errors.is_empty() {
            return;
        }

        let mut report = Report::<BoxInternalError>::new(&self.src, self.tree.clone());
        report.diagnostics = self.errors.iter().cloned().collect();
        report.dump();
    }

    pub fn dump_tree(&mut self) {
        let mut report = Report::<BoxInternalError>::new(&self.src, self.tree.clone());
        report.dump();
    }
}
