use chumsky::prelude::*;

use crate::{
    ast::Expr,
    lexer::{Span, TokenSet},
};

pub type ParseError<'a> = extra::Err<Rich<'a, char, Span>>;

// pub fn expr_parser<'a>() -> impl Parser<'a, TokenSet, ParseError> {
//     Expr
// }
