use std::fmt::Debug;

use chumsky::prelude::*;

use span::Spanned;
use token::Token;

pub mod span;
pub mod token;

pub const SYMBOLS: &[&str] = &[
    "=", "!", ">", "<", "$", "#", "+", "-", "*", "/", "&", "|", ".", "@", "^", ":", "\\",
];

pub type Span = SimpleSpan<usize>;

pub type LexToken = (Token, Span);

pub type TokenSet = Vec<LexToken>;

pub type LexError<'a> = extra::Err<Rich<'a, char, Span>>;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    index: usize,
    code: &'a str,
    source: Vec<Spanned<Token>>,
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
        .map_slice(|content: &str| match content {
            "->" => Token::Arrow,
            "=>" => Token::DoubleArrow,
            "<-" => Token::InverseArrow,
            "=" => Token::Equal,
            ":" => Token::Colon,
            _ => Token::symbol(content),
        })
        .labelled("symbol");

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded()
        .labelled("comment");

    let unicode = just("λ")
        .to(Token::Lambda)
        .or(just("∀").to(Token::Forall))
        .or(just("Π").to(Token::Pi))
        .or(just("Σ").to(Token::Sigma));

    let token = control_lexer()
        .or(unicode)
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
            "return" => Token::Return,
            "enum" => Token::Enum,
            "trait" => Token::Trait,
            "class" => Token::Class,
            "case" => Token::Case,
            "where" => Token::Where,
            "match" => Token::Match,
            "use" => Token::Use,
            "in" => Token::In,
            _ => Token::Ident(ident.into()),
        })
        .labelled("keyword")
}

impl<'a> Lexer<'a> {
    /// Creates a new [Lexer] based in a source code
    pub fn new(code: &'a str) -> Self {
        let (tokens, errs) = lexer().parse(code).into_output_errors();

        Self {
            index: 0,
            code,
            source: tokens
                .unwrap_or_default()
                .into_iter()
                .map(|(value, span)| Spanned::new(span.into_range(), value))
                .collect(),
            errs,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.get(self.index) {
            Some(value) => {
                self.index += 1;

                Some(value.clone())
            }

            // eof case
            None if self.code.is_empty() => Some(Spanned::new(0..0, Token::Eof)),
            None => {
                let start = self.code.len() - 1;
                let end = self.code.len();
                Some(Spanned::new(start..end, Token::Eof))
            }
        }
    }
}
