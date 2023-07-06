use asena_span::Spanned;

use crate::token::token_set::HasTokens;
use crate::token::Token;

use super::*;

pub use listener::*;
pub use walkable::*;

pub mod ast;
pub mod bridges;
pub mod listener;
pub mod walkable;

/// Represents a lexeme, a token with a value, represented in the Rust language.
#[derive(Clone)]
pub struct Lexeme<T> {
    pub token: Spanned<Token>,
    pub value: T,

    /// If the lexeme is `None`, it means that the lexeme is a placeholder.
    pub(crate) is_none: bool,
}

impl<T> Lexeme<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Lexeme<U> {
        Lexeme {
            token: self.token,
            value: f(self.value),
            is_none: false,
        }
    }

    /// Maps the token and the value of the lexeme.
    ///
    /// # Example
    /// ```rust,norun
    /// use asena_span::{Loc, Spanned};
    /// use asena_ast::token::{Token, TokenKind};
    /// use asena_ast::ast::Lexeme;
    ///
    /// let lexeme = Lexeme::<String> {
    ///    token: Spanned::new(Loc::default(), Token::new(TokenKind::Error, "")),
    ///    value: "hello".to_string(),
    /// };
    ///
    /// let lexeme = lexeme.map_token(|s, t| {
    ///    format!("{}: {:?}", s, t)
    /// });
    /// ```
    pub fn map_token<U>(self, f: impl FnOnce(T, &Spanned<Token>) -> U) -> Lexeme<U> {
        let value = f(self.value, &self.token);
        Lexeme {
            token: self.token,
            is_none: false,
            value,
        }
    }
}
