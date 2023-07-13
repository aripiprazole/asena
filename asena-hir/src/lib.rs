//! This crates provides an High Level API for the Asena Abstract Syntax Tree, with a focus on
//! object oriented programming, to make it easier to use.
//!
//! It's an abstraction layer over the AST, and it's not meant to be used for parsing, but for
//! semantic analysis and code generation.

#![feature(auto_traits)]
#![feature(associated_type_bounds)]

use asena_leaf::ast::Located;
use salsa::InternKey;

pub mod attr;
pub mod expr;
pub mod file;
pub mod hir_type;
pub mod interner;
pub mod literal;
pub mod pattern;
pub mod stmt;
pub mod top_level;
pub mod value;

#[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HirLoc;
impl Located for HirLoc {
    fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
        todo!()
    }
}
#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeId(usize);

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
    fn visit_expr_group(&mut self, _expr: &mut expr::HirExprGroup) -> T {
        T::default()
    }

    fn visit_expr_unit(&mut self) -> T {
        T::default()
    }
}
