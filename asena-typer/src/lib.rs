use std::fmt::Display;

use asena_leaf::{node::TreeKind, token::TokenKind};
use asena_report::InternalError;
use thiserror::Error;

use asena_ast::FunctionId;

pub mod infer;
pub mod unify;
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
#[derive(Default, Debug, Clone)]
pub enum Type {
    /// General type, it unifies with any type, it's used, so the type inference can be resilient
    /// to errors.
    #[default]
    Error,

    Unit,

    /// Type variable with a given [FunctionId], to be filled later.
    Hole(Option<FunctionId>),

    /// Constructor type with a given [Kind].
    Constructor(FunctionId, Kind),

    /// Variable type with a given [Kind].
    Variable(FunctionId, Kind),

    /// Type application.
    App(Box<Type>, Box<Type>),

    /// Type application with `->`.
    Arrow(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone)]
pub struct Scheme {
    pub variables: Vec<FunctionId>,
    pub mono: Type,
}

#[derive(Error, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum TypeError {
    #[error("Unexpected expression kind {0} in a type-level context")]
    UnexpectedExprInType(TreeKind),

    #[error("Unexpected expression kind {0} in a constraint context")]
    UnexpectedInConstraint(TreeKind),

    #[error("Unexpected lexeme {0} in type-level, dependent-types aren't implemented yet")]
    UnexpectedTokenInType(TokenKind),

    #[error("Unexpected field accessor in type-level, that is not a path to another type")]
    UnexpectedAccessorInType,

    #[error("Unsupported dependent-pairs in type-level yet")]
    UnsupportedSigmaInType,

    #[error("Unsupported type classes in the type-level yet")]
    UnsupportedQualifiersInType,

    #[error("Expected constraint, like: `\"Set\"` | `Constraint -> Constraint`")]
    ExpectedConstraint,

    #[error("Expected constraint name like `m`, given `m: Set -> Set`")]
    ExpectedConstraintName,
}

impl Type {
    /// Returns the kind of this type.
    pub fn kind(&self) -> Kind {
        match self {
            Type::Error => Kind::Error,
            Type::Unit => Kind::Star,
            Type::Constructor(_, kind) => kind.clone(),
            Type::Variable(_, kind) => kind.clone(),
            Type::App(lhs, _) => lhs.kind(),
            Type::Arrow(a, b) => Kind::Fun(a.kind().into(), b.kind().into()),
            Type::Hole(_) => Kind::Star,
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

#[cfg(test)]
mod tests {
    use asena_ast::*;
    use asena_grammar::*;
    use asena_leaf::ast::*;

    use asena_prec::*;

    use crate::infer::{AsenaTyper, ClassEnvironment, TypeEnvironment};

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();
        let mut type_env = TypeEnvironment::default();
        let mut class_env = ClassEnvironment::default();

        let mut tree = parse_asena_file!("./test.ase");

        let file = AsenaFile::new(tree.clone())
            .walks(AsenaInfixHandler::new(&mut tree, &mut prec_table))
            .walks(AsenaPrecReorder {
                prec_table: &prec_table,
                reporter: &mut tree,
            })
            .walks(AsenaTyper {
                type_env: &mut type_env,
                class_env: &mut class_env,
                reporter: &mut tree,
            });

        tree.reporter.dump();

        println!("{file:#?}");
        println!("{type_env:#?}");
    }
}
