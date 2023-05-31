use std::cell::Cell;
use std::iter::Peekable;

use crate::ast::node::Token;
use crate::lexer::span::{Localized, Spanned};
use crate::parser::error::ParseError;

use self::event::Event;

pub type TokenRef = Localized<Token>;

pub type StringRef = Localized<String>;

pub mod error;
pub mod event;
pub mod grammar;
pub mod support;

/// The language parser struct, it takes a [Token] iterator, that can be lazy or eager initialized
/// to advance and identify tokens on the programming language.
#[derive(Clone)]
pub struct Parser<'a> {
    errors: Vec<Spanned<ParseError>>,
    source: &'a str,
    index: usize,
    fuel: Cell<u32>,
    tokens: Vec<Spanned<Token>>,
    events: Vec<Event>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: Vec<Spanned<Token>>) -> Self {
        Self {
            source,
            index: 0,
            fuel: Cell::new(256),
            errors: Default::default(),
            tokens: Default::default(),
            events: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn it_works() {
        let code = "let combine = [x, MonadIO m, F a, C] => (a: m a) -> [b: m b] -> m c in todo";

        let stream = Lexer::new(code);
    }

    #[test]
    fn sig_decl() {
        let code = "cond : (f true) -> ((f false) -> (f cond));";

        let stream = Lexer::new(code);
    }

    #[test]
    fn lam_expr() {
        let code = "\\a b -> c";

        let stream = Lexer::new(code);
    }

    #[test]
    fn sigma_expr() {
        let code = "[a: t] -> b";

        let stream = Lexer::new(code);
    }

    #[test]
    fn unicode_expr() {
        let code = "Π (d: t) -> e";

        let stream = Lexer::new(code);
    }

    #[test]
    fn group_expr() {
        let code = "[Monad m] => m a";

        let stream = Lexer::new(code);
    }

    #[test]
    fn pi_expr() {
        let code = "(a: t) -> b";

        let lexer = Lexer::new(code);
    }

    #[test]
    fn ask_stmt() {
        let code = "do { (Just a) <- findUser 105; }";

        let lexer = Lexer::new(code);
    }
}
