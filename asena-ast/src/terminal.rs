use asena_leaf::spec::{Node, Terminal};
use asena_leaf::token::{Token, TokenKind};
use asena_span::Spanned;

use crate::*;

impl Terminal for FunctionId {
    fn terminal(token: Spanned<Token>) -> Node<Spanned<Self>> {
        let text = token.text.clone();

        Node::new(token.swap(FunctionId(text)))
    }
}

impl Terminal for ConstructorId {
    fn terminal(token: Spanned<Token>) -> Node<Spanned<Self>> {
        let text = token.text.clone();
        let span = token.span.clone();

        Node::new(token.swap(ConstructorId::new(span, &text)))
    }
}

impl Terminal for Local {
    fn terminal(token: Spanned<Token>) -> Node<Spanned<Self>> {
        if token.kind != TokenKind::Identifier {
            return Node::empty();
        }

        let text = token.text.clone();
        let span = token.span.clone();

        Node::new(token.swap(Local::new(span, &text)))
    }
}

impl Terminal for Literal {
    fn terminal(from: Spanned<Token>) -> Node<Spanned<Self>> {
        use crate::Signed::*;
        use Literal::*;

        let text = &from.text;
        let result = match from.kind {
            TokenKind::TrueKeyword => return from.swap(True).into(),
            TokenKind::FalseKeyword => return from.swap(False).into(),
            TokenKind::Nat => text.parse().map(Nat),
            TokenKind::Int8 => text.parse().map(|value| Int8(value, Signed)),
            TokenKind::UInt8 => text.parse().map(|value| Int8(value, Unsigned)),
            TokenKind::Int16 => text.parse().map(|value| Int16(value, Signed)),
            TokenKind::UInt16 => text.parse().map(|value| Int16(value, Unsigned)),
            TokenKind::Int32 => text.parse().map(|value| Int32(value, Signed)),
            TokenKind::UInt32 => text.parse().map(|value| Int32(value, Unsigned)),
            TokenKind::Int64 => text.parse().map(|value| Int64(value, Signed)),
            TokenKind::UInt64 => text.parse().map(|value| Int64(value, Unsigned)),
            TokenKind::Int128 => text.parse().map(|value| Int128(value, Signed)),
            TokenKind::UInt128 => text.parse().map(|value| Int128(value, Unsigned)),
            TokenKind::String => {
                let text = &from.text[1..(text.len() - 1)];
                let text = text.to_string();

                return Node::new(from.swap(String(text)));
            }
            TokenKind::Float32 => {
                return text
                    .parse()
                    .map(Float32)
                    .map_or(Node::empty(), |value| Node::new(from.swap(value)));
            }
            TokenKind::Float64 => {
                return text
                    .parse()
                    .map(Float64)
                    .map_or(Node::empty(), |value| Node::new(from.swap(value)));
            }
            _ => return Node::empty(),
        };

        result.map_or(Node::empty(), |value| Node::new(from.swap(value)))
    }
}
