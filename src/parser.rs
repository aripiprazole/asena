use std::cell::Cell;

use crate::ast::node::Token;
use crate::lexer::span::{Localized, Spanned};
use crate::lexer::Lexer;
use crate::parser::error::ParseError;
use crate::report::Diagnostic;

use self::event::Event;

pub type TokenRef = Localized<Token>;

pub type StringRef = Localized<String>;

pub mod builder;
pub mod error;
pub mod event;
pub mod grammar;
pub mod support;

/// The language parser struct, it takes a [Token] iterator, that can be lazy or eager initialized
/// to advance and identify tokens on the programming language.
#[derive(Clone)]
pub struct Parser<'a> {
    errors: Vec<Diagnostic<ParseError>>,
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
    use crate::ast::spec::Spec;
    use crate::ast::{Binary, Expr, Infix};
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn it_works() {
        let code = "53 + 75 + 42";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        let infix = Infix::new(parser.build_tree().unwrap().into());

        let lhs = infix.lhs();
        let rhs = infix.rhs().duplicate();

        infix.rhs().replace(lhs);
        infix.lhs().replace(rhs);

        println!("{:#?}", infix);
    }

    #[test]
    fn sig_decl() {
        let code = "cond [Monad m] : 10";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::decl(&mut parser);

        println!("{:#?}", parser.build_tree().unwrap());
    }

    #[test]
    fn lam_expr() {
        let code = "\\a b -> c";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        println!("{:#?}", parser.build_tree().unwrap());
    }

    #[test]
    fn sigma_expr() {
        let code = "(awa {})";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        println!("{:#?}", parser.build_tree().unwrap());
    }

    #[test]
    fn unicode_expr() {
        let code = "Π (d: t) -> e";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        println!("{:#?}", parser.build_tree().unwrap());
    }

    #[test]
    fn qual_app_expr() {
        let code = "a b => a b";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        let tree = parser.build_tree().unwrap();

        println!("{:#?}", Expr::make(tree));
    }

    #[test]
    fn app_expr() {
        let code = "a (@ b)";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        let tree = parser.build_tree().unwrap();

        println!("{:#?}", Expr::make(tree));
    }

    #[test]
    fn qual_expr() {
        let code = "a => b";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        println!("{:#?}", parser.build_tree().data());
    }

    #[test]
    fn group_expr() {
        let code = "(1 + 2)";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        let tree = parser.build_tree().unwrap();

        println!("{:#?}", Expr::make(tree));
    }

    #[test]
    fn pi_expr() {
        let code = "(a: t) -> (b: t) -> a b";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        let tree = parser.build_tree().unwrap();

        println!("{:#?}", Expr::make(tree));
    }

    #[test]
    fn anonymous_pi_expr() {
        let code = "m -> a -> m a";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::expr(&mut parser);

        let tree = parser.build_tree().unwrap();

        println!("{:#?}", Expr::make(tree));
    }

    #[test]
    fn ask_stmt() {
        let code = "do { (Just a) <- findUser 105; }";

        let mut parser = Parser::from(Lexer::new(code));
        grammar::decl(&mut parser);

        let tree = parser.build_tree().unwrap();

        println!("{:#?}", Expr::make(tree));
    }
}
