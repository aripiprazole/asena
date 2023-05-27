use chumsky::prelude::*;

use span::Spanned;
use token::Token;

pub mod span;
pub mod token;

pub const SYMBOLS: &[&str] = &[
    "=", "!", ">", "<", "$", "#", "+", "-", "*", "/", "&", "|", ".", "@", "^", ":",
];

pub type Span = SimpleSpan<usize>;

pub type LexToken = (Token, Span);

pub type TokenSet = Vec<LexToken>;

pub type LexError<'a> = extra::Err<Rich<'a, char, Span>>;

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
