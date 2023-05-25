use chumsky::prelude::*;

use crate::ast::{Binary, Expr, ExprRef, FunctionId, Literal};
use crate::lexer::{Span, Spanned, Token};

/// The type of the input that our parser operates on. The input is the `&[(Token, Span)]` token buffer generated by the
/// lexer, wrapped in a `SpannedInput` which 'splits' it apart into its constituent parts, tokens and spans, for chumsky
/// to understand.
pub type ParserInput<'tokens> = chumsky::input::SpannedInput<Token, Span, &'tokens [(Token, Span)]>;

pub type ParseError<'a> = extra::Err<Rich<'a, Token, Span>>;

pub fn expr_parser<'a>() -> impl Parser<'a, ParserInput<'a>, ExprRef, ParseError<'a>> {
    spanned!(recursive(|expr| {
        let group = spanned!(expr)
            .clone()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .map(Expr::Group);

        let literal = literal_parser().or(group);

        // `^`, `>>`, `<<`, `|`, `&`
        let bitwise = spanned!(literal).then(
            op("^")
                .or(op(">>"))
                .or(op("<<"))
                .or(op("|"))
                .or(op("&"))
                .then(spanned!(literal))
                .repeated()
                .collect::<Vec<_>>(),
        );
        let bitwise = spanned!(binary!(bitwise));

        // `>`, `>=`, `<=`, `<`
        let int_cmp = bitwise.clone().then(
            op(">")
                .or(op(">="))
                .or(op("<="))
                .or(op("<"))
                .then(bitwise)
                .repeated()
                .collect::<Vec<_>>(),
        );
        let int_cmp = spanned!(binary!(int_cmp));

        // `==`, `!=`
        let eq_cmp = int_cmp.clone().then(
            //
            op("==")
                .or(op("!="))
                .then(int_cmp)
                .repeated()
                .collect::<Vec<_>>(),
        );
        let eq_cmp = spanned!(binary!(eq_cmp));

        // `&&`, `||`
        let bool_cmp = eq_cmp.clone().then(
            //
            op("&&")
                .or(op("||"))
                .then(eq_cmp)
                .repeated()
                .collect::<Vec<_>>(),
        );
        let bool_cmp = spanned!(binary!(bool_cmp));

        // `$`, `%`, `->`, `=>`, `=>>`, `@`
        let infix_fn = bool_cmp.clone().then(
            op("$")
                .or(op("%"))
                .or(op("->"))
                .or(op("=>"))
                .or(op("=>>"))
                .or(op("@"))
                .then(bool_cmp)
                .repeated()
                .collect::<Vec<_>>(),
        );
        let infix_fn = spanned!(binary!(infix_fn));

        // `^^`
        let pow = infix_fn
            .clone()
            .then(op("^^").then(infix_fn).repeated().collect::<Vec<_>>());
        let pow = spanned!(binary!(pow));

        // `*`, `/`
        let factor = pow
            .clone()
            .then(op("*").or(op("/")).then(pow).repeated().collect::<Vec<_>>());
        let factor = spanned!(binary!(factor));

        // `+`, `-`
        let term = factor.clone().then(
            op("+")
                .or(op("-"))
                .then(factor)
                .repeated()
                .collect::<Vec<_>>(),
        );
        let term = binary!(term);

        term
    }))
}

pub fn literal_parser<'a>() -> impl Parser<'a, ParserInput<'a>, Expr, ParseError<'a>> + Clone {
    select! {
        Token::True => Expr::Literal(Literal::True),
        Token::False => Expr::Literal(Literal::False),
        Token::String(string) => Expr::Literal(Literal::String(string)),
    }
    .labelled("primary")
}

fn op<'a>(name: &str) -> impl Parser<'a, ParserInput<'a>, String, ParseError<'a>> + Clone {
    let name: String = name.into();
    symbol().filter(move |x| x.clone() == name.clone())
}

fn symbol<'a>() -> impl Parser<'a, ParserInput<'a>, String, ParseError<'a>> + Clone {
    any()
        .try_map(|tok, span| match tok {
            Token::Symbol(symbol) => Ok(symbol),
            _ => Err(Rich::custom(span, "Expected infix symbol")),
        })
        .repeated()
        .collect()
        .map(|x: Vec<String>| x.join(""))
}

macro_rules! spanned {
    ($e:expr) => {
        $e.clone()
            .map_with_span(|expr, span: Span| Spanned::new(span.into(), expr))
    };
}

macro_rules! binary {
    ($e:expr) => {
        $e.map(|(lhs, rhs)| {
            rhs.iter()
                .fold(lhs, |lhs, (fn_id, rhs)| {
                    let fn_id = FunctionId::new(fn_id);
                    let span = lhs.span().start()..rhs.span().end();
                    Expr::binary(lhs.clone(), fn_id, rhs.clone(), span)
                })
                .value()
                .clone()
        })
    };
}

use binary;
use spanned;
