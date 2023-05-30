use crate::lexer::span::Spanned;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;

use super::Parser;

pub type Diagnostic = Vec<Spanned<ParseError>>;

impl<'a, S: Iterator<Item = Spanned<Token>> + Clone> Parser<'a, S> {}
