use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::ops::{ControlFlow, Deref, DerefMut, FromResidual, Try};
use std::rc::Rc;

use asena_span::{Loc, Spanned};

use crate::node::{Child, TreeKind};
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
    fn make(tree: GreenTree) -> Option<Self> {
        let _ = tree;
        None
    }

    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let _ = token;
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

pub trait Virtual: Node {
    fn create() -> Self {
        Self::new(GreenTree::of(Self::tree_kind()))
    }

    fn tree_kind() -> TreeKind;
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

    fn as_new_ast<T: Node>(&self) -> T {
        T::new(self.clone().unwrap().as_new_node())
    }
}

impl<T: Terminal + Debug + Default + Clone + 'static> Leaf for T {
    fn make(tree: GreenTree) -> Option<Self> {
        match tree {
            GreenTree::Leaf { data, .. } => {
                if data.children.is_empty() {
                    return None;
                }

                <Self as Leaf>::terminal(data.clone().swap(data.single().clone()))
            }
            GreenTree::Token(lexeme) => <Self as Leaf>::terminal(lexeme.token),
            GreenTree::None => None,
            GreenTree::Empty => None,
        }
    }

    #[inline(always)]
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        <T as Terminal>::terminal(token)
    }
}

impl<T: Leaf> Leaf for Option<T> {
    fn make(from: GreenTree) -> Option<Self> {
        Some(T::make(from))
    }
}

impl<T: Leaf> Leaf for Vec<T> {
    fn make(tree: GreenTree) -> Option<Self> {
        match tree {
            GreenTree::Leaf { data, .. } => {
                let mut items = vec![];
                for child in &data.children {
                    match &child.value {
                        Child::Tree(tree) => {
                            let green_child = GreenTree::new(child.replace(tree.clone()));

                            items.push(T::make(green_child)?)
                        }
                        Child::Token(..) => {}
                    }
                }
                Some(items)
            }
            GreenTree::Token(_) => None,
            GreenTree::None => None,
            GreenTree::Empty => None,
        }
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
