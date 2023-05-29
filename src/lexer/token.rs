use std::fmt::{Debug, Display};

/// Represents a true-false value, just like an wrapper to [bool], this represents if an integer
/// value is signed, or unsigned.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signed {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // keywords
    Let,    // let
    True,   // true
    False,  // false
    If,     // if
    Else,   // else
    Then,   // then
    Type,   // type
    Record, // record
    Enum,   // enum
    Trait,  // trait
    Class,  // class
    Case,   // case
    Where,  // where
    Match,  // match
    Use,    // use
    In,     // in

    // unicode
    Lambda, // λ
    Forall, // ∀
    Pi,     // Π
    Sigma,  // Σ

    // control symbols
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    Comma,        // ,
    Semi,         // ;
    Colon,        // :
    Dot,          // .
    Help,         // ?
    Equal,        // =

    DoubleArrow,  // =>
    Arrow,        // ->
    InverseArrow, // <-

    // integers
    Int8(u8, Signed),     // <n>u8
    Int16(u16, Signed),   // <n>u16
    Int32(u32, Signed),   // <n>u32
    Int64(u64, Signed),   // <n>u64
    Int128(u128, Signed), // <n>u128

    // floats
    Float32(f32),
    Float64(f64),

    // literals
    Symbol(String),
    Ident(String),
    String(String),

    // end of file
    Eof,
}

impl Token {
    pub fn symbol(s: &str) -> Token {
        Token::Symbol(s.into())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Then => write!(f, "then"),
            Token::Type => write!(f, "type"),
            Token::Record => write!(f, "record"),
            Token::Enum => write!(f, "enum"),
            Token::Trait => write!(f, "trait"),
            Token::Class => write!(f, "class"),
            Token::Case => write!(f, "case"),
            Token::Where => write!(f, "where"),
            Token::Match => write!(f, "match"),
            Token::Use => write!(f, "use"),
            Token::In => write!(f, "in"),
            Token::Lambda => write!(f, "λ"),
            Token::Forall => write!(f, "∀"),
            Token::Pi => write!(f, "Π"),
            Token::Sigma => write!(f, "Σ"),
            Token::LeftBracket => write!(f, "{{"),
            Token::RightBracket => write!(f, "}}"),
            Token::LeftBrace => write!(f, "["),
            Token::RightBrace => write!(f, "]"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Semi => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),
            Token::Help => write!(f, "?"),
            Token::Equal => write!(f, "="),
            Token::DoubleArrow => write!(f, "=>"),
            Token::Arrow => write!(f, "->"),
            Token::InverseArrow => write!(f, "<-"),
            Token::Int8(n, Signed::Signed) => write!(f, "{n}i8"),
            Token::Int8(n, Signed::Unsigned) => write!(f, "{n}u8"),
            Token::Int16(n, Signed::Signed) => write!(f, "{n}i16"),
            Token::Int16(n, Signed::Unsigned) => write!(f, "{n}u16"),
            Token::Int32(n, Signed::Signed) => write!(f, "{n}i32"),
            Token::Int32(n, Signed::Unsigned) => write!(f, "{n}u32"),
            Token::Int64(n, Signed::Signed) => write!(f, "{n}i64"),
            Token::Int64(n, Signed::Unsigned) => write!(f, "{n}u64"),
            Token::Int128(n, Signed::Signed) => write!(f, "{n}i128"),
            Token::Int128(n, Signed::Unsigned) => write!(f, "{n}u128"),
            Token::Float32(n) => write!(f, "{n}f32"),
            Token::Float64(n) => write!(f, "{n}f64"),
            Token::Symbol(symbol) => write!(f, "{symbol}"),
            Token::Ident(ident) => write!(f, "'{ident}"),
            Token::String(string) => write!(f, "\"{string}\""),
            Token::Eof => write!(f, "<<EOF>>"),
        }
    }
}
