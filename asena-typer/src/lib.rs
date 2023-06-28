use std::fmt::Display;

use asena_leaf::node::TreeKind;
use asena_report::InternalError;
use thiserror::Error;

use asena_ast::FunctionId;

pub mod infer;
pub mod validation;

/// Represents a type of a [Type].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// General kind, it unifies with any kind, it's used, so the type inference can be resilient
    /// to errors.
    Error,

    Star,
    Fun(Box<Kind>, Box<Kind>),
}

/// Represents a type, in the type-level context.
///
/// TODO: use a dependent type system, so that we can represent types like `Vec<3, i32>`.
#[derive(Debug, Clone)]
pub enum Type {
    /// General type, it unifies with any type, it's used, so the type inference can be resilient
    /// to errors.
    Error,

    /// Constructor type with a given [Kind].
    Constructor(FunctionId, Kind),

    /// Variable type with a given [Kind].
    Variable(FunctionId, Kind),

    /// Type application.
    Apply(Box<Type>, Box<Type>),

    /// Type application with `->`.
    Arrow(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone)]
pub struct Scheme {
    pub variables: Vec<Kind>,
    pub mono: Type,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TypeError {
    #[error("Unexpected expression kind {0} in a type-level context")]
    UnexpectedExprInType(TreeKind),
}

impl Type {
    /// Returns the kind of this type.
    pub fn kind(&self) -> Kind {
        match self {
            Type::Error => Kind::Error,
            Type::Constructor(_, kind) => kind.clone(),
            Type::Variable(_, kind) => kind.clone(),
            Type::Apply(lhs, _) => lhs.kind(),
            Type::Arrow(a, b) => Kind::Fun(a.kind().into(), b.kind().into()),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Error => write!(f, "Error"),
            Kind::Star => write!(f, "*"),
            Kind::Fun(a, b) => write!(f, "{} -> {}", a, b),
        }
    }
}

impl TypeError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl InternalError for TypeError {
    fn code(&self) -> u16 {
        self.discriminant() as u16
    }

    fn kind(&self) -> asena_report::DiagnosticKind {
        asena_report::DiagnosticKind::Error
    }
}
