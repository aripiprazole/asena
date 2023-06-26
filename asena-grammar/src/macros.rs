//! This files contains macros to be used to parse the AST, and make easier to do it. It will parse
//! the CST node, with the given token stream and the macro name will match the `grammar rule`.
//!
//! # Example
//!
//! ```rust
//! use asena_grammar::*;
//!
//! asena_expr! { 1 + 1 }
//! ```

use std::ops::{Deref, DerefMut};

use asena_ast::walker::{DefaultReporter, Reporter};
use asena_leaf::node::Tree;
use asena_report::InternalError;
use asena_span::Spanned;

#[macro_export]
macro_rules! asena_expr {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);
        $crate::new_reportable(string, asena_parser::Parser::from(asena_lexer::Lexer::new(string))
            .run($crate::expr)
            .build_tree()
            .unwrap())
    }};
}

#[macro_export]
macro_rules! asena_file {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);

        $crate::new_reportable(string, asena_parser::Parser::from(asena_lexer::Lexer::new(string))
            .run($crate::file)
            .build_tree()
            .unwrap())
    }};
}

#[derive(Clone)]
pub struct Reportable<R: asena_ast::walker::Reporter> {
    pub reporter: R,
    pub data: Spanned<Tree>,
}

pub fn new_reportable(src: &str, tree: Spanned<Tree>) -> Reportable<DefaultReporter> {
    let reporter = DefaultReporter::new(src, tree.clone());

    Reportable {
        data: tree,
        reporter,
    }
}

impl Reportable<DefaultReporter> {
    pub fn unwrap(&self) -> Spanned<Tree> {
        self.data.clone()
    }
}

impl<R: asena_ast::walker::Reporter> Reporter for Reportable<R> {
    fn diagnostic<E: InternalError, A>(&mut self, error: E, at: Spanned<A>)
    where
        E: 'static,
    {
        self.reporter.diagnostic(error, at)
    }
}

impl<R: asena_ast::walker::Reporter> DerefMut for Reportable<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<R: asena_ast::walker::Reporter> Deref for Reportable<R> {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub use asena_expr;
pub use asena_file;
