//! This crates provides an High Level API for the Asena Abstract Syntax Tree, with a focus on
//! object oriented programming, to make it easier to use.
//!
//! It's an abstraction layer over the AST, and it's not meant to be used for parsing, but for
//! semantic analysis and code generation.

#![feature(auto_traits)]
#![feature(associated_type_bounds)]

use expr::HirExpr;
use salsa::InternKey;

pub mod attr;
pub mod expr;
pub mod file;
pub mod hir_type;
pub mod interner;
pub mod literal;
pub mod loc;
pub mod pattern;
pub mod stmt;
pub mod top_level;
pub mod value;

pub use loc::*;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(salsa::InternId);

impl InternKey for Name {
    fn from_intern_id(v: salsa::InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

pub trait HirVisitor<T: Default> {
    fn visit_expr_literal(&mut self, _: HirExpr, _: &mut expr::HirExprLiteral) -> T {
        T::default()
    }

    fn visit_expr_call(&mut self, _: HirExpr, _: &mut expr::HirExprCall) -> T {
        T::default()
    }

    fn visit_expr_reference(&mut self, _: HirExpr, _: &mut expr::HirExprReference) -> T {
        T::default()
    }

    fn visit_expr_match(&mut self, _: HirExpr, _: &mut expr::HirExprMatch) -> T {
        T::default()
    }

    fn visit_expr_help(&mut self, _: HirExpr, _: &mut expr::HirExprHelp) -> T {
        T::default()
    }

    fn visit_expr_ann(&mut self, _: HirExpr, _: &mut expr::HirExprAnn) -> T {
        T::default()
    }

    fn visit_expr_lam(&mut self, _: HirExpr, _: &mut expr::HirExprLam) -> T {
        T::default()
    }

    fn visit_expr_array(&mut self, _: HirExpr, _: &mut expr::HirExprArray) -> T {
        T::default()
    }

    fn visit_expr_this(&mut self, _: HirExpr) -> T {
        T::default()
    }

    fn visit_expr_error(&mut self, _: HirExpr) -> T {
        T::default()
    }

    fn visit_expr_unit(&mut self, _: HirExpr) -> T {
        T::default()
    }

    fn visit_expr_group(&mut self, _: HirExpr, _: &mut expr::HirExprGroup) -> T {
        T::default()
    }
}
