use std::borrow::Cow;
use std::cell::Cell;

use crate::error::ParseError;

use asena_leaf::node::{Token, TokenKind, TreeKind};
use asena_report::{Diagnostic, DiagnosticKind};
use asena_span::Spanned;

use super::event::{Event, MarkClosed, MarkOpened};
use super::Parser;

impl<'a> Parser<'a> {
    pub fn open(&mut self) -> MarkOpened {
        let start = self.peek().into_owned();
        let mark = MarkOpened::new(self.events.len(), start.span.clone());
        self.events.push(Event::Open(start.swap(TreeKind::Error)));
        mark
    }

    pub fn open_before(&mut self, mark: MarkClosed) -> MarkOpened {
        let span = mark.span();
        let mark = MarkOpened::new(mark.index(), span.clone());
        let open_at = Spanned::new(span, TreeKind::Error);
        self.events.insert(mark.index(), Event::Open(open_at));
        mark
    }

    pub fn ignore(&mut self, mark: MarkOpened) {
        self.events.remove(mark.index());
    }

    pub fn field(&mut self, name: &'static str) {
        self.events.push(Event::Field(name))
    }

    pub fn close(&mut self, mark: MarkOpened, kind: TreeKind) -> MarkClosed {
        // Build tree position with the initial state, and the current
        let initial = mark.span();
        let current = self.peek().into_owned();
        let position = initial.start..current.span.end;

        // Replace the state in the tree builder
        self.events[mark.index()] = Event::Open(Spanned::new(position, kind));
        self.events.push(Event::Close);

        MarkClosed::new(mark.index(), mark.span())
    }

    pub fn terminal(&mut self, kind: TreeKind) -> MarkClosed {
        let mark = self.open();
        self.advance();
        self.close(mark, kind)
    }

    pub fn advance(&mut self) {
        #[cfg(debug_assertions)]
        assert!(!self.eof(), "Found eof at index {}", self.index);

        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.index += 1;
    }

    pub fn eof(&mut self) -> bool {
        self.tokens.len() == self.index
    }

    pub fn savepoint(&self) -> Self {
        Self {
            errors: vec![],
            source: self.source,
            index: self.index,
            fuel: Cell::new(self.fuel.get()),
            events: self.events.clone(),
            tokens: self.tokens.clone(),
        }
    }

    pub fn return_at(&mut self, point: Self) {
        self.index = point.index;
        self.fuel = point.fuel;
        self.events = point.events;
    }

    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.eat(kind) {
            return;
        }

        let error = self.build_error(ParseError::ExpectedTokenError(kind));
        self.errors.push(Diagnostic::new(error));
    }

    pub fn warning(&mut self, error: ParseError) -> Option<MarkClosed> {
        let error = self.build_error(error);
        let mut error = Diagnostic::new(error);
        error.kind = DiagnosticKind::Warning;
        self.errors.push(error);
        None
    }

    pub fn report(&mut self, error: ParseError) -> Option<MarkClosed> {
        if self.eof() {
            let error = self.build_error(error);
            self.errors.push(Diagnostic::new(error));
            return None;
        }
        let mark = self.open();
        let error = self.build_error(error);
        self.errors.push(Diagnostic::new(error));
        self.advance();

        Some(self.close(mark, TreeKind::Error))
    }

    pub fn at(&self, kind: TokenKind) -> bool {
        kind == self.lookahead(0)
    }

    pub fn lookahead(&self, lookahead: usize) -> TokenKind {
        self.nth(lookahead)
            .map_or(TokenKind::Eof, |token| token.value.kind)
    }

    pub fn nth(&self, lookahead: usize) -> Option<&Spanned<Token>> {
        #[cfg(debug_assertions)]
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }

        self.fuel.set(self.fuel.get() - 1);
        self.tokens.get(self.index + lookahead)
    }

    pub fn peek(&self) -> Cow<Spanned<Token>> {
        self.nth(0).map(Cow::Borrowed).unwrap_or_else(|| {
            let start = self.source.len();
            let end = start;

            Cow::Owned(Spanned::new(start..end, Token::eof()))
        })
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn build_error(&self, error: ParseError) -> Spanned<ParseError> {
        self.peek().into_owned().swap(error)
    }
}
