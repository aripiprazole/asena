use std::cell::Cell;

use crate::error::ParseError;

use asena_leaf::node::Token;
use asena_lexer::Lexer;
use asena_report::Diagnostic;
use asena_span::{Localized, Spanned};

use self::event::Event;

pub type TokenRef = Localized<Token>;

pub type StringRef = Localized<String>;

pub mod builder;
pub mod error;
pub mod event;
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
