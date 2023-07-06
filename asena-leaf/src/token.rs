use std::fmt::Display;

use asena_interner::Intern;

use self::{kind::TokenKind, text::Text};

use super::named::Named;

pub mod kind;
pub mod text;
pub mod token_set;

#[derive(Debug, Clone, Hash, Default)]
pub struct Token {
    pub name: Option<&'static str>,
    pub kind: TokenKind,
    pub text: Intern<String>,
    pub full_text: Text,
}

impl Token {
    pub fn new(kind: TokenKind, text: &str) -> Self {
        Self {
            name: None,
            kind,
            text: Intern::new(text.into()),
            full_text: Default::default(),
        }
    }

    pub fn eof() -> Self {
        Self {
            name: None,
            kind: TokenKind::Eof,
            text: Default::default(),
            full_text: Default::default(),
        }
    }

    pub fn is_error(&self) -> bool {
        self.kind == TokenKind::Error
    }

    /// Uses the [std::fmt::Formatter] to write a pretty-printed tree in the terminal for debug
    /// porpuses.
    ///
    /// It usually likes like the following printed code:
    /// ```txt
    /// EXPR_BINARY
    ///     LIT_INT8
    ///         '1' @ 0..1
    ///     '+' @ 2..3
    ///     LIT_INT8
    ///         '1' @ 4..5
    /// @ 0..5
    /// ```
    ///
    /// The use of this code is like the following code, you should never use directly this function
    /// since its local:
    /// ```
    /// let tree = Tree::new(TreeKind::Error); // just to show
    /// println!("{:#?}", tree);
    /// ```
    pub(crate) fn render(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result {
        write!(f, "{tab}")?;
        if let Some(name) = self.name {
            write!(f, "{name} = ")?;
        }
        write!(f, "'{}'", self.text)
    }
}

impl Named for TokenKind {}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
