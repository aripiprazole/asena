use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::ops::{ControlFlow, Deref, DerefMut, FromResidual, Try};
use std::rc::Rc;

use asena_interner::Intern;
use asena_span::{Loc, Span, Spanned};

use crate::node::{Child, TreeKind};
use crate::token::kind::TokenKind;

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
    fn make(tree: GreenTree) -> Option<Self> {
        let _ = tree;
        None
    }

    fn terminal(token: Intern<Spanned<Token>>) -> Option<Self> {
        let _ = token;
        None
    }
}

/// Represents a type that can creates a new terminal if matched the certain conditions,
/// atherwise returns `None`.
///
/// A `Literal` enum should be a good example for this trait.
pub trait Terminal: Leaf + Sized + Debug + Sized + Clone + Default {
    fn terminal(token: Intern<Spanned<Token>>) -> Option<Self>;
}

pub trait VirtualNode: Node {
    const KIND: TreeKind;

    fn create() -> Self {
        Self::new(GreenTree::of(Self::KIND))
    }
}

pub trait Located {
    fn location(&self) -> Cow<'_, Loc>;
}

impl<T> Located for Spanned<T> {
    fn location(&self) -> Cow<'_, Loc> {
        Cow::Borrowed(&self.span)
    }
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

impl<T: Located> Located for Vec<T> {
    fn location(&self) -> Cow<'_, Loc> {
        if self.is_empty() {
            return Cow::Owned(Loc::default());
        }

        let start = self.first().unwrap().location().into_owned();
        let end = self.last().unwrap().location().into_owned();

        Cow::Owned(start.on(end))
    }
}

impl<T: Terminal + 'static> Leaf for T {
    fn make(tree: GreenTree) -> Option<Self> {
        match tree {
            GreenTree::Leaf(leaf) => {
                if leaf.data.children.is_empty() {
                    return None;
                }

                let first_item = leaf.data.single().clone();
                let spanned = leaf.data.replace(first_item);

                Leaf::terminal(Intern::new(spanned))
            }
            GreenTree::Token(lexeme) => Leaf::terminal(lexeme.token),
            _ => None,
        }
    }

    #[inline(always)]
    fn terminal(token: Intern<Spanned<Token>>) -> Option<Self> {
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
            GreenTree::Leaf(leaf) => {
                let mut items = vec![];
                for child in &leaf.data.children {
                    match &child {
                        Child::Tree(tree) => {
                            let green_child = GreenTree::new(tree.clone());

                            if let Some(item) = T::make(green_child) {
                                items.push(item);
                            }
                        }
                        Child::Token(token) => {
                            if let Some(item) = T::terminal(token.clone()) {
                                items.push(item);
                            }
                        }
                    }
                }
                Some(items)
            }
            GreenTree::Vec(children) => children
                .into_iter()
                .filter_map(|child| T::make(child))
                .collect::<Vec<_>>()
                .into(),
            GreenTree::Token(_) => None,
            GreenTree::None => None,
            GreenTree::Empty => None,
        }
    }
}

mod cursor;
mod green;
mod key;
mod lexeme;
mod listener;
mod tree;
mod visitor;
mod walk;

pub use cursor::*;
pub use green::*;
pub use key::*;
pub use lexeme::*;
pub use listener::*;
pub use tree::*;
pub use visitor::*;
pub use walk::*;
