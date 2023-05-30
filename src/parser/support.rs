use std::borrow::Cow;

use crate::ast::node::{Token, TokenKind, TreeKind};
use crate::lexer::span::Spanned;
use crate::parser::error::ParseError;

use super::event::{Event, MarkOpened};
use super::Parser;

pub type Diagnostic = Vec<Spanned<ParseError>>;

impl<'a> Parser<'a> {
    pub(crate) fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened::new(self.events.len());
        self.events.push(Event::Open(TreeKind::Error));
        mark
    }

    pub(crate) fn close(&mut self, mark: MarkOpened, kind: TreeKind) {
        self.events[mark.index()] = Event::Open(kind);
        self.events.push(Event::Close);
    }

    pub(crate) fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.index += 1;
    }

    pub(crate) fn eof(&mut self) -> bool {
        self.tokens.len() == self.index
    }

    pub(crate) fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) {
        if self.eat(kind) {
            return;
        }

        let error = self.build_error(ParseError::Expected(kind));
        self.errors.push(error);
    }

    pub(crate) fn recover_with(&mut self, error: ParseError) {
        let mark = self.open();
        let error = self.build_error(error);
        self.errors.push(error);
        self.advance();
        self.close(mark, TreeKind::Error)
    }

    pub(crate) fn at(&self, kind: TokenKind) -> bool {
        kind == self.lookahead(0)
    }

    pub(crate) fn lookahead(&self, lookahead: usize) -> TokenKind {
        self.nth(lookahead)
            .map_or(TokenKind::Eof, |token| token.value.kind)
    }

    pub(crate) fn nth(&self, lookahead: usize) -> Option<&Spanned<Token>> {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }

        self.fuel.set(self.fuel.get() - 1);
        self.tokens.get(self.index + lookahead)
    }

    pub(crate) fn peek(&self) -> Cow<Spanned<Token>> {
        self.nth(0).map(Cow::Borrowed).unwrap_or_else(|| {
            let start = self.source.len();
            let end = start;

            Cow::Owned(Spanned::new(start..end, Token::eof()))
        })
    }

    fn build_error(&self, error: ParseError) -> Spanned<ParseError> {
        self.peek().into_owned().swap(error)
    }
}
