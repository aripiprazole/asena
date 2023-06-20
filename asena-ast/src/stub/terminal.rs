use asena_leaf::ast::Terminal;
use asena_leaf::token::{Token, TokenKind};
use asena_span::Spanned;

use crate::*;

impl Terminal for FunctionId {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let text = token.text.clone();

        Some(FunctionId(text))
    }
}

impl Terminal for ConstructorId {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let text = token.text.clone();
        let span = token.span;

        Some(ConstructorId::new(span, &text))
    }
}

impl Terminal for Local {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        if token.kind != TokenKind::Identifier {
            todo!();
        }

        let text = token.text.clone();
        let span = token.span;

        Some(Local::new(span, &text))
    }
}

impl Terminal for Literal {
    fn terminal(from: Spanned<Token>) -> Option<Self> {
        use crate::Signed::*;
        use Literal::*;

        let text = from.text.clone();

        match from.kind {
            TokenKind::TrueKeyword => Some(True),
            TokenKind::FalseKeyword => Some(False),
            TokenKind::Nat => text.parse().map(Nat).ok(),
            TokenKind::Int8 => text.parse().map(|value| Int8(value, Signed)).ok(),
            TokenKind::UInt8 => text.parse().map(|value| Int8(value, Unsigned)).ok(),
            TokenKind::Int16 => text.parse().map(|value| Int16(value, Signed)).ok(),
            TokenKind::UInt16 => text.parse().map(|value| Int16(value, Unsigned)).ok(),
            TokenKind::Int32 => text.parse().map(|value| Int32(value, Signed)).ok(),
            TokenKind::UInt32 => text.parse().map(|value| Int32(value, Unsigned)).ok(),
            TokenKind::Int64 => text.parse().map(|value| Int64(value, Signed)).ok(),
            TokenKind::UInt64 => text.parse().map(|value| Int64(value, Unsigned)).ok(),
            TokenKind::Int128 => text.parse().map(|value| Int128(value, Signed)).ok(),
            TokenKind::UInt128 => text.parse().map(|value| Int128(value, Unsigned)).ok(),
            TokenKind::Float64 => text.parse().map(Float64).ok(),
            TokenKind::Float32 => text.parse().map(Float32).ok(),
            TokenKind::String => {
                let text = &from.text[1..(text.len() - 1)];
                let text = text.to_string();

                Some(String(text))
            }
            _ => None,
        }
    }
}
