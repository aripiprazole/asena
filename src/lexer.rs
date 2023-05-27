use std::fmt::{Debug, Display};

use chumsky::prelude::*;

use crate::span::Spanned;

pub const SYMBOLS: &[&str] = &[
    "=", "!", ">", "<", "$", "#", "+", "-", "*", "/", "&", "|", ".", "@", "^", ":",
];

pub type Span = SimpleSpan<usize>;

pub type LexToken = (Token, Span);

pub type TokenSet = Vec<LexToken>;

pub type LexError<'a> = extra::Err<Rich<'a, char, Span>>;

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

    // end of file TODO
    Eof,
}

pub struct Lexer<'a> {
    pub source: Box<dyn Iterator<Item = Spanned<Token>>>,
    pub errs: Vec<Rich<'a, char>>,
}

/// It's the programming language, lexer, that transforms the string, into a set of [Token].
pub fn lexer<'a>() -> impl Parser<'a, &'a str, TokenSet, LexError<'a>> {
    let num = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .slice()
        .from_str()
        .unwrapped()
        .map(Token::Float64)
        .labelled("number"); // TODO: implement another float/integer variants

    let string = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map_slice(|string: &str| Token::String(string.into()))
        .labelled("string literal");

    let symbol = one_of(SYMBOLS.join(""))
        .repeated()
        .at_least(1)
        .map_slice(|content: &str| Token::Symbol(content.into()))
        .labelled("symbol");

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded()
        .labelled("comment");

    let token = control_lexer()
        .or(symbol)
        .or(num)
        .or(string)
        .or(ident_lexer());

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

fn control_lexer<'a>() -> impl Parser<'a, &'a str, Token, LexError<'a>> {
    one_of("()[]{};,.")
        .map(|control: char| match control {
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ';' => Token::Semi,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '.' => Token::Dot,
            // This code is unreachable, because its matched by the [one_of]
            // functions
            _ => panic!("unreachable"),
        })
        .labelled("control flow symbol")
}

fn ident_lexer<'a>() -> impl Parser<'a, &'a str, Token, LexError<'a>> {
    text::ident()
        .map(|ident: &str| match ident {
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "then" => Token::Then,
            "type" => Token::Type,
            "record" => Token::Record,
            "enum" => Token::Enum,
            "trait" => Token::Trait,
            "class" => Token::Class,
            "case" => Token::Case,
            "where" => Token::Where,
            "match" => Token::Match,
            "use" => Token::Use,
            _ => Token::Ident(ident.into()),
        })
        .labelled("keyword")
}

impl<'a> Lexer<'a> {
    /// Creates a new [Lexer] based in a source code
    pub fn new(code: &'a str) -> Self {
        let (tokens, errs) = lexer().parse(code).into_output_errors();

        Self {
            source: Box::new(
                tokens
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(value, span)| Spanned::new(span.into_range(), value)),
            ),
            errs,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source
            .next()
            .or_else(|| Some(Spanned::new(0..0, Token::Eof)))
    }
}

impl Token {
    pub fn sym(s: &str) -> Token {
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
            Token::Symbol(symbol) => write!(f, "`{symbol}"),
            Token::Ident(ident) => write!(f, "'{ident}"),
            Token::String(string) => write!(f, "\"{string}\""),
            Token::Eof => write!(f, "<<EOF>>"),
        }
    }
}
