use asena_span::Spanned;

use crate::token::Token;

use super::*;

/// Represents a lexeme, a token with a value, represented in the Rust language.
#[derive(Clone)]
pub struct Lexeme<T> {
    pub token: Spanned<Token>,
    pub value: T,
}

impl<T> Lexeme<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Lexeme<U> {
        Lexeme {
            token: self.token,
            value: f(self.value),
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
            value,
        }
    }
}

impl<T: Default> Default for Lexeme<T> {
    fn default() -> Self {
        Self {
            token: Spanned::new(Loc::default(), Token::new(TokenKind::Error, "")),
            value: Default::default(),
        }
    }
}

impl<W, T: Walkable<W>> Walkable<W> for Lexeme<T> {
    fn walk(&self, walker: &mut W) {
        self.value.walk(walker);
    }
}

impl<T: Node> Node for Option<T> {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        Some(T::new(tree))
    }

    fn unwrap(self) -> GreenTree {
        match self {
            Some(vale) => vale.unwrap(),
            None => GreenTree::Empty,
        }
    }
}

impl<T> Located for Lexeme<T> {
    fn location(&self) -> Cow<'_, Loc> {
        Cow::Borrowed(&self.token.span)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.value, f)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.value, f)
    }
}

impl<T> std::ops::Deref for Lexeme<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::borrow::Borrow<T> for Lexeme<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<T: Terminal + 'static> Leaf for Lexeme<T> {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let spanned = token.clone();
        let terminal = <T as Terminal>::terminal(token)?;

        Some(Self {
            token: spanned,
            value: terminal,
        })
    }
}

impl<T: Leaf + 'static> Node for Lexeme<T> {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        match tree.into() {
            GreenTree::Leaf { data, .. } => Self {
                token: data.clone().swap(data.single().clone()),
                value: T::make(data).unwrap_or_default(),
            },
            GreenTree::Token(lexeme) => {
                let value = match lexeme.value.downcast_ref::<T>() {
                    Some(value) => value.clone(),
                    None => {
                        return Self {
                            token: Spanned::new(Loc::default(), Token::new(TokenKind::Error, "")),
                            value: T::default(),
                        }
                    }
                };

                Self {
                    token: lexeme.token,
                    value,
                }
            }
            GreenTree::Empty => Self {
                token: Spanned::new(Loc::default(), Token::new(TokenKind::Error, "")),
                value: T::default(),
            },
        }
    }

    fn unwrap(self) -> GreenTree {
        GreenTree::Token(Lexeme {
            token: self.token,
            value: Rc::new(self.value),
        })
    }
}
