use std::fmt::Debug;

use chumsky::prelude::*;

use asena_leaf::node::Token;
use asena_leaf::token::TokenKind::*;

use asena_span::Spanned;

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
    pub source: &'a str,
    pub tokens: Vec<Spanned<Token>>,
    pub errors: Vec<Rich<'a, char>>,
}

/// It's the programming language, lexer, that transforms the string, into a set of [Token].
pub fn lexer<'a>() -> impl Parser<'a, &'a str, TokenSet, LexError<'a>> {
    let num = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .slice()
        .map(|value: &str| Token::new(Float64, value))
        .labelled("number"); // TODO: implement another float/integer variants

    let string = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map_slice(|string: &str| Token::new(Str, string))
        .labelled("string literal");

    let symbol = one_of(SYMBOLS.join(""))
        .repeated()
        .at_least(1)
        .map_slice(|content: &str| match content {
            "->" => Token::new(RightArrow, content),
            "=>" => Token::new(DoubleArrow, content),
            "<-" => Token::new(LeftArrow, content),
            "=" => Token::new(Equal, content),
            ":" => Token::new(Colon, content),
            _ => Token::new(Symbol, content),
        })
        .labelled("symbol");

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded()
        .labelled("comment");

    let semi = just(";")
        .repeated()
        .at_least(1)
        .to(Token::new(Semi, ";"))
        .labelled("semi");

    let unicode = just("λ")
        .to(Token::new(Lambda, "λ"))
        .or(just("∀").to(Token::new(Forall, "∀")))
        .or(just("Π").to(Token::new(Pi, "Π")))
        .or(just("Σ").to(Token::new(Sigma, "Σ")));

    let token = control_lexer()
        .or(semi)
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
    one_of("()[]{},.")
        .map(|control: char| match control {
            '[' => Token::new(LeftBracket, "["),
            ']' => Token::new(RightBracket, "]"),
            '{' => Token::new(LeftBrace, "{"),
            '}' => Token::new(RightBrace, "}"),
            '(' => Token::new(LeftParen, "("),
            ')' => Token::new(RightParen, ")"),
            ',' => Token::new(Comma, ","),
            ':' => Token::new(Colon, ":"),
            '.' => Token::new(Dot, "."),
            // This code is unreachable, because its matched by the [one_of]
            // functions
            _ => panic!("unreachable"),
        })
        .labelled("control flow symbol")
}

fn ident_lexer<'a>() -> impl Parser<'a, &'a str, Token, LexError<'a>> {
    text::ident()
        .map(|ident: &str| match ident {
            "let" => Token::new(LetKeyword, ident),
            "true" => Token::new(TrueKeyword, ident),
            "false" => Token::new(FalseKeyword, ident),
            "if" => Token::new(IfKeyword, ident),
            "else" => Token::new(ElseKeyword, ident),
            "then" => Token::new(ThenKeyword, ident),
            "type" => Token::new(TypeKeyword, ident),
            "record" => Token::new(RecordKeyword, ident),
            "return" => Token::new(ReturnKeyword, ident),
            "enum" => Token::new(EnumKeyword, ident),
            "trait" => Token::new(TraitKeyword, ident),
            "class" => Token::new(ClassKeyword, ident),
            "case" => Token::new(CaseKeyword, ident),
            "where" => Token::new(WhereKeyword, ident),
            "match" => Token::new(MatchKeyword, ident),
            "use" => Token::new(UseKeyword, ident),
            "in" => Token::new(InKeyword, ident),
            _ => Token::new(Identifier, ident),
        })
        .labelled("keyword")
}

impl<'a> Lexer<'a> {
    /// Creates a new [Lexer] based in a source code
    pub fn new(code: &'a str) -> Self {
        let (tokens, errs) = lexer().parse(code).into_output_errors();

        Self {
            index: 0,
            source: code,
            tokens: tokens
                .unwrap_or_default()
                .into_iter()
                .map(|(value, span)| Spanned::new(span.into_range(), value))
                .collect(),
            errors: errs,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.get(self.index) {
            Some(value) => {
                self.index += 1;

                Some(value.clone())
            }

            // eof case
            None if self.source.is_empty() => Some(Spanned::new(0..0, Token::new(Eof, ""))),
            None => {
                let start = self.source.len() - 1;
                let end = self.source.len();
                Some(Spanned::new(start..end, Token::new(Eof, "")))
            }
        }
    }
}
