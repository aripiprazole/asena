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

use asena_ast::reporter::Reporter;
use asena_leaf::{ast::GreenTree, node::Tree};
use asena_span::Spanned;

#[macro_export]
macro_rules! asena_expr {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);
        $crate::Reportable::new(string, asena_parser::Parser::from(asena_lexer::Lexer::new(None, string))
            .run(|p| $crate::expr(p, $crate::Linebreak::Cont))
            .build_tree()
            .unwrap())
    }};
}

#[macro_export]
macro_rules! asena_decl {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);

        $crate::Reportable::new(string, asena_parser::Parser::from(asena_lexer::Lexer::new(None, string))
            .run($crate::decl)
            .build_tree()
            .unwrap())
    }};
}

#[macro_export]
macro_rules! asena_stmt {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);

        $crate::Reportable::new(string, asena_parser::Parser::from(asena_lexer::Lexer::new(None, string))
            .run($crate::stmt)
            .build_tree()
            .unwrap())
    }};
}

#[macro_export]
macro_rules! asena_file {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);
        let file = std::path::PathBuf::from($file);

        $crate::Reportable::new(string, asena_parser::Parser::from(asena_lexer::Lexer::new(file, string))
            .run($crate::file)
            .build_tree()
            .unwrap())
    }};
}

#[macro_export]
macro_rules! parse_asena_file {
    ($file:expr) => {{
        let string = include_str!($file);
        let file = std::path::PathBuf::from($file);

        $crate::Reportable::new(
            string,
            asena_parser::Parser::from(asena_lexer::Lexer::new(file, string))
                .run($crate::file)
                .build_tree()
                .unwrap(),
        )
    }};
}

#[derive(Clone)]
pub struct Reportable {
    pub reporter: Reporter,
    pub data: Spanned<Tree>,
}

impl std::borrow::Borrow<Spanned<Tree>> for Reportable {
    fn borrow(&self) -> &Spanned<Tree> {
        &self.data
    }
}

impl From<Reportable> for GreenTree {
    fn from(value: Reportable) -> Self {
        value.data.into()
    }
}

impl Reportable {
    pub fn new(src: &str, tree: Spanned<Tree>) -> Reportable {
        let reporter = Reporter::new(src, tree.clone());

        Reportable {
            data: tree,
            reporter,
        }
    }

    #[inline(always)]
    pub fn reporting<U>(&mut self, f: impl FnOnce(&mut Reporter) -> U) -> U {
        f(&mut self.reporter)
    }

    #[inline(always)]
    pub fn unwrap(&self) -> Spanned<Tree> {
        self.data.clone()
    }
}

impl DerefMut for Reportable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Deref for Reportable {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub use asena_expr;
pub use asena_file;
