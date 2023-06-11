use std::fmt::{Debug, Formatter};

use crate::*;

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QualifiedPath(expr) => write!(f, "{expr:#?}"),
            Self::Infix(expr) => write!(f, "{expr:#?}"),
            Self::Accessor(expr) => write!(f, "{expr:#?}"),
            Self::App(expr) => write!(f, "{expr:#?}"),
            Self::Array(expr) => write!(f, "{expr:#?}"),
            Self::Dsl(expr) => write!(f, "{expr:#?}"),
            Self::Lam(expr) => write!(f, "{expr:#?}"),
            Self::Let(expr) => write!(f, "{expr:#?}"),
            Self::Local(expr) => write!(f, "{expr:#?}"),
            Self::Ann(expr) => write!(f, "{expr:#?}"),
            Self::Qual(expr) => write!(f, "{expr:#?}"),
            Self::Pi(expr) => write!(f, "{expr:#?}"),
            Self::Sigma(expr) => write!(f, "{expr:#?}"),
            Self::Literal(expr) => write!(f, "{expr:#?}"),
            Self::Group(expr) => write!(f, "{expr:#?}"),
            Self::Help(expr) => write!(f, "{expr:#?}"),
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nat(n) => write!(f, "{n}n"),
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Int8(i8, Signed::Signed) => write!(f, "{i8}i8"),
            Self::Int8(u8, Signed::Unsigned) => write!(f, "{u8}u8"),
            Self::Int16(i16, Signed::Signed) => write!(f, "{i16}i16"),
            Self::Int16(u16, Signed::Unsigned) => write!(f, "{u16}u16"),
            Self::Int32(i32, Signed::Signed) => write!(f, "{i32}i32"),
            Self::Int32(u32, Signed::Unsigned) => write!(f, "{u32}u32"),
            Self::Int64(i64, Signed::Signed) => write!(f, "{i64}i64"),
            Self::Int64(u64, Signed::Unsigned) => write!(f, "{u64}u64"),
            Self::Int128(i128, Signed::Signed) => write!(f, "{i128}i128"),
            Self::Int128(u128, Signed::Unsigned) => write!(f, "{u128}u128"),
            Self::Float32(f32) => write!(f, "{f32}f32"),
            Self::Float64(f64) => write!(f, "{f64}f64"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Use(decl) => write!(f, "{decl:#?}"),
            Self::Signature(decl) => write!(f, "{decl:#?}"),
            Self::Assign(decl) => write!(f, "{decl:#?}"),
            Self::Command(decl) => write!(f, "{decl:#?}"),
            Self::Class(decl) => write!(f, "{decl:#?}"),
            Self::Instance(decl) => write!(f, "{decl:#?}"),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infer => write!(f, "Infer"),
            Self::Explicit(expr) => write!(f, "Type({:#?})", expr),
        }
    }
}

impl Debug for Pat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spread(spread) => write!(f, "{spread:#?}"),
            Self::Wildcard(wildcard) => write!(f, "{wildcard:#?}"),
            Self::Literal(literal) => write!(f, "{literal:#?}"),
            Self::Local(local) => write!(f, "{local:#?}"),
            Self::QualifiedPath(qualified_path) => write!(f, "{qualified_path:#?}"),
            Self::Constructor(constructor) => write!(f, "{constructor:#?}"),
            Self::List(list) => write!(f, "{list:#?}"),
        }
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ask(stmt) => write!(f, "{stmt:#?}"),
            Self::Set(stmt) => write!(f, "{stmt:#?}"),
            Self::Return(stmt) => write!(f, "{stmt:#?}"),
            Self::Eval(stmt) => write!(f, "{stmt:#?}"),
        }
    }
}
