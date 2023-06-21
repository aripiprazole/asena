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

pub trait Leaf: Sized + Clone {
    fn make(tree: Spanned<Tree>) -> Option<Self>;
}

pub trait Terminal: Sized {
    fn terminal(token: Spanned<Token>) -> Option<Self>;
}

pub trait IntoGreenTree {
    fn into_green_tree(self) -> GreenTree;
}

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
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
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

pub use cursor::*;
pub use green::*;
pub use tree::*;
pub use value::*;
