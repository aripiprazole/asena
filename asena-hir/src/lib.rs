//! This crates provides an High Level API for the Asena Abstract Syntax Tree, with a focus on
//! object oriented programming, to make it easier to use.
//!
//! It's an abstraction layer over the AST, and it's not meant to be used for parsing, but for
//! semantic analysis and code generation.

#![feature(auto_traits)]
#![feature(associated_type_bounds)]

use std::sync::Arc;

use database::HirBag;
use query::HirDebug;

pub mod attr;
pub mod database;
pub mod expr;
pub mod file;
pub mod hir_type;
pub mod literal;
pub mod pattern;
pub mod query;
pub mod stmt;
pub mod top_level;
pub mod value;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeId(usize);

#[derive(Default, Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameId(pub usize);

impl NameId {
    pub fn intern(db: Arc<dyn HirBag>, name: &str) -> Self {
        db.clone().intern_name(name.into())
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

impl HirDebug for NameId {
    type Database = dyn database::HirBag;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", db.name_data(*self))
    }
}

impl_hir_dbg!(
    dyn database::HirBag,
    String,
    usize,
    u128,
    u64,
    u32,
    u16,
    u8,
    isize,
    i128,
    i64,
    i32,
    i16,
    i8,
    bool
);
