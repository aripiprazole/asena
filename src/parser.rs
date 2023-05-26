use std::iter::Peekable;

use crate::lexer::Token;

/// The language parser struct, it takes a [Token] iterator, that can be lazy or eager initialized
/// to advance and identify tokens on the programming language.
pub struct Parser<'a, TokStream: Iterator<Item = Token>> {
    pub source: &'a str,
    pub index: usize,
    pub stream: Peekable<TokStream>,
}
