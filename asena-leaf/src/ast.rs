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
pub trait Leaf: Debug + Sized + Clone + Default {
    /// Creates a new node from the given tree and if the given tree is not matched,
    /// returns `None`.
    ///
    /// TODO: Change parameter to GreenTree, please.
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        None
    }

    fn terminal(_token: Spanned<Token>) -> Option<Self> {
        None
    }
}

/// Represents a type that can creates a new terminal if matched the certain conditions,
/// atherwise returns `None`.
///
/// A `Literal` enum should be a good example for this trait.
pub trait Terminal: Leaf + Sized + Debug + Sized + Clone + Default {
    fn terminal(token: Spanned<Token>) -> Option<Self>;
}

pub trait Located {
    fn location(&self) -> Cow<'_, Loc>;
}

pub trait IntoVirtual<T>: Sized {
    fn into_virtual(self) -> Option<T>;
}

pub trait FromVirtual<T>: Sized {
    fn from_virtual(value: T) -> Option<Self>;
}

impl<T, U: FromVirtual<T>> IntoVirtual<U> for T {
    fn into_virtual(self) -> Option<U> {
        U::from_virtual(self)
    }
}

/// Represents a green tree used on the [Leaf] enum variants.
pub trait Ast: Node + Deref<Target = GreenTree> + DerefMut + Clone + Debug {}

pub trait Node: Sized + Debug + Clone {
    fn new<I: Into<GreenTree>>(tree: I) -> Self;

    fn unwrap(self) -> GreenTree;
}

impl<T: Terminal + Debug + Default + Clone + 'static> Leaf for T {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        if from.children.is_empty() {
            return None;
        }

        <Self as Leaf>::terminal(from.clone().swap(from.single().clone()))
    }

    #[inline(always)]
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        <T as Terminal>::terminal(token)
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
mod lexeme;
mod tree;
mod walk;

pub use cursor::*;
pub use green::*;
pub use lexeme::*;
pub use tree::*;
pub use walk::*;
