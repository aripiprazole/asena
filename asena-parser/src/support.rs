use std::borrow::Cow;
use std::cell::Cell;

use crate::error::ParseError;

use asena_leaf::node::{kind::TokenKind, Token, TreeKind};
use asena_report::{Diagnostic, DiagnosticKind, Quickfix};
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
        // its needed to be closed again
        mark.0.setup();
        mark
    }

    pub fn abandon(&mut self, mark: MarkOpened) {
        mark.0.defuse();

        self.events.remove(mark.index());
    }

    pub fn field(&mut self, name: &'static str) {
        self.events.push(Event::Field(name))
    }

    pub fn close(&mut self, mark: MarkOpened, kind: TreeKind) -> MarkClosed {
        // Build tree position with the initial state, and the current
        let initial = mark.span();
        let current = self.peek().into_owned();
        let position = initial.into_ranged().unwrap_or_default().start
            ..current.span.into_ranged().unwrap_or_default().end;

        // Replace the state in the tree builder
        self.events[mark.index()] = Event::Open(Spanned::new(position.into(), kind));
        self.events.push(Event::Close);

        mark.0.defuse();

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
            fuel: Cell::new(256),
            events: self.events.clone(),
            tokens: self.tokens.clone(),
        }
    }

    pub fn return_at(&mut self, point: Self) {
        self.index = point.index;
        self.events = point.events;
    }

    pub fn at_newline(&mut self, nth: usize) -> bool {
        match self.nth(nth) {
            Some(token) => {
                token.full_text.before_whitespace.contains('\n')
                    || token.full_text.before_whitespace.contains('\r')
                    || token.full_text.before_whitespace.contains('\u{2028}')
                    || token.full_text.before_whitespace.contains('\u{2029}')
            }
            None => false,
        }
    }

    pub fn newline(&mut self) -> bool {
        if self.at_newline(1) {
            self.advance();
            true
        } else {
            false
        }
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

    pub fn fixable<const N: usize, F>(&mut self, error: ParseError, fixes: F)
    where
        F: FnOnce(&Spanned<Token>) -> [Quickfix; N],
    {
        if let Some(token) = self.nth(0) {
            let fixes = fixes(token).to_vec();
            if self.eof() {
                let error = self.build_error(error.clone());
                self.errors
                    .push(Diagnostic::new(error).add_fixes(fixes.clone()));
            }
            let error = self.build_error(error);
            self.errors.push(Diagnostic::new(error).add_fixes(fixes));
        }
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

    pub fn at_any(&self, kind: &[TokenKind]) -> bool {
        kind.contains(&self.lookahead(0))
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

            Cow::Owned(Spanned::new((start..end).into(), Token::eof()))
        })
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn as_succeded(self) -> Option<Parser<'a>> {
        if self.has_errors() {
            None
        } else {
            Some(self)
        }
    }

    pub fn as_closed<A>(self, f: impl FnOnce(&mut Self) -> A) -> Option<(A, Parser<'a>)> {
        let mut parser = self;
        let closed = f(&mut parser);
        if parser.has_errors() {
            None
        } else {
            Some((closed, parser))
        }
    }

    fn build_error(&self, error: ParseError) -> Spanned<ParseError> {
        self.peek().into_owned().swap(error)
    }
}
