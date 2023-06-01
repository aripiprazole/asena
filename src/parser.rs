use std::cell::Cell;

use crate::ast::node::Token;
use crate::lexer::span::{Localized, Spanned};
use crate::lexer::Lexer;
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
            tokens,
            errors: Default::default(),
            events: Default::default(),
        }
    }
}

impl<'a> From<Lexer<'a>> for Parser<'a> {
    fn from(value: Lexer<'a>) -> Self {
        Self::new(value.source, value.tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Binary;
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn it_works() {
        let code = "1 + 1 + 1";

        let mut parser = Parser::from(Lexer::new(code));
        parser.expr_binary();

        println!("{:#?}", Binary::new(parser.build_tree()));
    }

    #[test]
    fn sig_decl() {
        let code = "cond : (f true) -> ((f false) -> (f cond));";

        let mut parser = Parser::from(Lexer::new(code));
        parser.decl();

        println!("{:#?}", parser.build_tree());
    }

    #[test]
    fn lam_expr() {
        let code = "\\a b -> c";

        let mut parser = Parser::from(Lexer::new(code));
        parser.decl();

        println!("{:#?}", parser.build_tree());
    }

    #[test]
    fn sigma_expr() {
        let code = "[a: t] -> b";

        let mut parser = Parser::from(Lexer::new(code));
        parser.decl();

        println!("{:#?}", parser.build_tree());
    }

    #[test]
    fn unicode_expr() {
        let code = "Π (d: t) -> e";

        let mut parser = Parser::from(Lexer::new(code));
        parser.decl();

        println!("{:#?}", parser.build_tree());
    }

    #[test]
    fn group_expr() {
        let code = "[Monad m] => m a";

        let mut parser = Parser::from(Lexer::new(code));
        parser.decl();

        println!("{:#?}", parser.build_tree());
    }

    #[test]
    fn pi_expr() {
        let code = "(a: t) -> b";

        let mut parser = Parser::from(Lexer::new(code));
        parser.decl();

        println!("{:#?}", parser.build_tree());
    }

    #[test]
    fn ask_stmt() {
        let code = "do { (Just a) <- findUser 105; }";

        let mut parser = Parser::from(Lexer::new(code));
        parser.decl();

        println!("{:#?}", parser.build_tree());
    }
}
