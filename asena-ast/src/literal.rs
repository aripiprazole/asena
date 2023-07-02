use std::borrow::Cow;
use std::fmt::Debug;

use asena_leaf::ast::{Lexeme, LexemeWalkable, Located, Terminal};
use asena_leaf::token::{kind::TokenKind::*, Token};
use asena_span::{Loc, Spanned};

use crate::AsenaVisitor;
use crate::Signed::{self, *};

/// Represents a language literal construct, can hold numbers, strings, booleans, etc.
#[derive(Default, Clone)]
pub enum Literal {
    Nat(u128), // <n>n
    String(String),

    // integers
    Int8(u8, Signed),     // <n>u8
    Int16(u16, Signed),   // <n>u32
    Int32(u32, Signed),   // <n>u32
    Int64(u64, Signed),   // <n>u64
    Int128(u128, Signed), // <n>u128

    // floats
    Float32(f32),
    Float64(f64),

    // booleans
    True,
    False,

    #[default]
    Error,
}

impl Literal {
    /// Transforms the literal into a u8 if possible.
    pub fn to_u8(&self) -> Option<u8> {
        self.contents().parse().ok()
    }

    /// Transforms the literal into a u32 if possible.
    pub fn to_u32(&self) -> Option<u32> {
        self.contents().parse().ok()
    }

    /// Transforms the literal into a u64 if possible.
    pub fn to_u64(&self) -> Option<u64> {
        self.contents().parse().ok()
    }

    /// Returns the contents of the literal as a string.
    pub fn contents(&self) -> String {
        match self {
            Literal::Nat(n) => n.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Int8(n, Signed::Signed) => (*n as i8).to_string(),
            Literal::Int8(n, Signed::Unsigned) => n.to_string(),
            Literal::Int16(n, Signed::Signed) => (*n as i16).to_string(),
            Literal::Int16(n, Signed::Unsigned) => n.to_string(),
            Literal::Int32(n, Signed::Signed) => (*n as i32).to_string(),
            Literal::Int32(n, Signed::Unsigned) => n.to_string(),
            Literal::Int64(n, Signed::Signed) => (*n as i64).to_string(),
            Literal::Int64(n, Signed::Unsigned) => n.to_string(),
            Literal::Int128(n, Signed::Signed) => (*n as i128).to_string(),
            Literal::Int128(n, Signed::Unsigned) => n.to_string(),
            Literal::Float32(f) => f.to_string(),
            Literal::Float64(f) => f.to_string(),
            Literal::True => "true".to_string(),
            Literal::False => "false".to_string(),
            Literal::Error => "".to_string(),
        }
    }
}

impl Located for Literal {
    fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
        Cow::Owned(Loc::Synthetic)
    }
}

impl LexemeWalkable for Literal {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn lexeme_walk(value: Lexeme<Self>, walker: &mut Self::Walker<'_>) {
        walker.visit_literal(value);
    }
}

impl Terminal for Literal {
    fn terminal(from: Spanned<Token>) -> Option<Self> {
        let text = from.text.clone();

        match from.kind {
            Nat => text.parse().map(Self::Nat).ok(),
            Int8 => text.parse().map(|value| Self::Int8(value, Signed)).ok(),
            UInt8 => text.parse().map(|value| Self::Int8(value, Unsigned)).ok(),
            Int16 => text.parse().map(|value| Self::Int16(value, Signed)).ok(),
            UInt16 => text.parse().map(|value| Self::Int16(value, Unsigned)).ok(),
            Int32 => text.parse().map(|value| Self::Int32(value, Signed)).ok(),
            UInt32 => text.parse().map(|value| Self::Int32(value, Unsigned)).ok(),
            Int64 => text.parse().map(|value| Self::Int64(value, Signed)).ok(),
            UInt64 => text.parse().map(|value| Self::Int64(value, Unsigned)).ok(),
            Int128 => text.parse().map(|value| Self::Int128(value, Signed)).ok(),
            UInt128 => text.parse().map(|value| Self::Int128(value, Unsigned)).ok(),
            Float64 => text.parse().map(Self::Float64).ok(),
            Float32 => text.parse().map(Self::Float32).ok(),
            TrueKeyword => Some(Self::True),
            FalseKeyword => Some(Self::False),
            Str => {
                let text = &from.text[1..(text.len() - 1)];
                let text = text.to_string();

                Some(Self::String(text))
            }
            _ => None,
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "Error"),
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
