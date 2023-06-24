use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::ops::{ControlFlow, Deref, DerefMut, FromResidual, Try};
use std::rc::Rc;

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

    fn terminal(_token: Spanned<Token>) -> Option<Self> {
        None
    }
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

impl<T: Terminal + Clone> Leaf for T {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        if from.children.is_empty() {
            return None;
        }

        Self::terminal(from.clone().swap(from.single().clone()))
    }

    fn terminal(token: Spanned<Token>) -> Option<Self> {
        <T as Terminal>::terminal(token)
    }
}

impl<T: Leaf> Leaf for Option<T> {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        Some(T::make(from))
    }

    fn terminal(token: Spanned<Token>) -> Option<Self> {
        None
    }
}

impl<T: Leaf> Leaf for Vec<T> {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        let mut items = vec![];
        for child in &tree.children {
            match &child.value {
                Child::Tree(tree) => items.push(T::make(child.replace(tree.clone()))?),
                Child::Token(..) => {
                    println!("abuble")
                }
            }
        }
        Some(items)
    }

    fn terminal(token: Spanned<Token>) -> Option<Self> {
        None
    }
}

mod cursor;
mod green;
mod lexeme;
mod tree;
mod walk;

pub use cursor::*;
pub use green::*;
pub use lexeme::*;
pub use tree::*;
pub use walk::*;
