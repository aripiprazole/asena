use crate::parser::error::ParseError;
use crate::span::Spanned;
use crate::token::Token;

use super::error::Result;
use super::{Parser, TokenRef};

impl<'a, S: Iterator<Item = Spanned<Token>>> Parser<'a, S> {
    /// Eat a matching token, and return it if matching correctly.
    pub(crate) fn expect(&mut self, token: Token) -> Result<TokenRef> {
        self.eat(|next| {
            if next.value() == &token {
                Some(next.clone())
            } else {
                None
            }
        })
        .map_err(|error| error.with_error(ParseError::Expected(token)))
    }

    /// Tries to parse using a function [F], but it can't, the index would not be increased, so the
    /// result of the function is going to be Ok(None); but if everything is ok, the result is going
    /// to be the parsed value.
    pub(crate) fn catch<T, F>(&mut self, mut f: F) -> Result<Option<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let current_index = self.index;

        match f(self) {
            Ok(value) => Ok(Some(value)),
            Err(..) if self.index == current_index => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Peeks the current token using a function [F], and jumps to the next token.
    pub(crate) fn eat<T, F>(&mut self, f: F) -> Result<T>
    where
        F: Fn(&TokenRef) -> Option<T>,
    {
        let next = self.peek();
        match f(&next) {
            Some(value) => {
                self.next();
                Ok(value)
            }
            None => Err(next.swap(ParseError::UnexpectedToken)),
        }
    }

    /// Jumps to the next token, and increases the current token index.
    pub(crate) fn next(&mut self) -> TokenRef {
        self.index += 1;

        self.stream.next().unwrap()
    }

    /// End the diagnostic with an error of [ParseError], spanned with the current token location.
    pub(crate) fn end_diagnostic<T>(&mut self, error: ParseError) -> Result<T> {
        Err(self.stream.peek().unwrap().replace(error))
    }

    /// Sees the current token, and return it cloned.
    pub(crate) fn peek(&mut self) -> Spanned<Token> {
        self.stream.peek().unwrap().clone()
    }
}
