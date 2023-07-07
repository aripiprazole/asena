//! This crates provides an High Level API for the Asena Abstract Syntax Tree, with a focus on
//! object oriented programming, to make it easier to use.
//!
//! It's an abstraction layer over the AST, and it's not meant to be used for parsing, but for
//! semantic analysis and code generation.

pub mod attr;
pub mod expr;
pub mod pattern;
pub mod stmt;
pub mod top_level;
pub mod value;

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirTypeId(usize);

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirAttributeId(usize);

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirLiteralId(usize);

#[derive(Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeId(usize);

#[derive(Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameId(usize);

pub trait HirVisitor<T: Default> {
    fn visit_expr_group(&mut self, _expr: &mut expr::HirExprGroup) -> T {
        T::default()
    }

    fn visit_expr_unit(&mut self) -> T {
        T::default()
    }
}
