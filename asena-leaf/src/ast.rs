use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::{ControlFlow, Deref, DerefMut, FromResidual, Try};
use std::sync::Arc;

use asena_span::Spanned;

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

pub trait IntoGreenTree {
    fn into_green_tree(self) -> GreenTree;
}

/// Represents a green tree used on the [Leaf] enum variants.
pub trait Ast: Deref<Target = GreenTree> + DerefMut + Clone + Debug {}

impl<T: Terminal + Clone> Leaf for T {
    fn make(from: Spanned<Tree>) -> Option<Self> {
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

impl<T: Leaf> Default for Cursor<T> {
    fn default() -> Self {
        Self {
            value: Arc::new(RefCell::new(Default::default())),
            children: Default::default(),
        }
    }
}

mod cursor;
mod green;
mod tree;
mod value;
mod walk;

pub use cursor::*;
pub use green::*;
pub use tree::*;
pub use value::*;
pub use walk::*;
