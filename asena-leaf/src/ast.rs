use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::ops::{ControlFlow, Deref, DerefMut, FromResidual, Try};

use asena_span::{Loc, Spanned};

use crate::node::Child;
use crate::token::TokenKind;

use super::node::Tree;
use super::token::Token;

pub type LeafKey = &'static str;

/// Represents a type that can creates a new node if matched the certain conditions,
/// atherwise returns `None`.
///
/// A `Expr` enum should be a good example for this trait.
///
/// # Example
/// ```rust,norun
/// struct Group { ... }
///
/// impl Ast for Group { ... }
///
/// enum Expr {
///   Literal(Literal),
///   Group(Group),
/// }
///
/// impl Leaf for Expr { ... }
/// ```
pub trait Leaf: Sized + Clone {
    fn make(tree: Spanned<Tree>) -> Option<Self>;
}

/// Represents a type that can creates a new terminal if matched the certain conditions,
/// atherwise returns `None`.
///
/// A `Literal` enum should be a good example for this trait.
pub trait Terminal: Sized {
    fn terminal(token: Spanned<Token>) -> Option<Self>;
}

pub trait Located {
    fn location(&self) -> Cow<'_, Loc>;
}

/// Represents a green tree used on the [Leaf] enum variants.
pub trait Ast: Node + Deref<Target = GreenTree> + DerefMut + Clone + Debug {}

pub trait Node: Sized {
    fn new<I: Into<GreenTree>>(tree: I) -> Self;

    fn unwrap(self) -> GreenTree;
}

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

    pub fn map_token<U>(self, f: impl FnOnce(T, &Spanned<Token>) -> U) -> Lexeme<U> {
        let value = f(self.value, &self.token);
        Lexeme {
            token: self.token,
            value,
        }
    }
}

impl<T> Deref for Lexeme<T> {
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
            None => GreenTree::Error,
        }
    }
}

impl<T: Default> Default for Lexeme<T> {
    fn default() -> Self {
        Lexeme {
            token: Spanned::new(Loc::Synthetic, Token::eof()),
            value: T::default(),
        }
    }
}

impl<T: Terminal> From<T> for Lexeme<T> {
    fn from(value: T) -> Self {
        Lexeme {
            token: Spanned::new(Loc::Synthetic, Token::eof()),
            value,
        }
    }
}

impl<T> Located for Lexeme<T> {
    fn location(&self) -> Cow<'_, Loc> {
        Cow::Borrowed(&self.token.span)
    }
}

impl<T: Display> Display for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl<T: Debug> Debug for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl<T: Terminal + Clone> Terminal for Lexeme<T> {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let spanned = token.clone();
        let terminal = T::terminal(token)?;

        Some(Self {
            token: spanned,
            value: terminal,
        })
    }
}

impl<T: Leaf + Default> Node for Lexeme<T> {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        let tree: GreenTree = tree.into();
        let tree = tree.or_empty();

        Self {
            token: tree.clone().swap(tree.single().clone()),
            value: T::make(tree).unwrap_or_default(),
        }
    }

    fn unwrap(self) -> GreenTree {
        GreenTree::Error
    }
}

impl<T: Terminal + Clone> Leaf for T {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        if from.children.is_empty() {
            return None;
        }

        <T as Terminal>::terminal(from.clone().swap(from.single().clone()))
    }
}

impl<T: Leaf> Leaf for Option<T> {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        Some(T::make(from))
    }
}

impl<T: Leaf> Leaf for Vec<T> {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        let mut items = vec![];
        for child in &tree.children {
            match &child.value {
                Child::Tree(tree) => items.push(T::make(child.replace(tree.clone()))?),
                Child::Token(..) => {}
            }
        }
        Some(items)
    }
}

mod cursor;
mod green;
mod tree;
mod walk;

pub use cursor::*;
pub use green::*;
pub use tree::*;
pub use walk::*;
