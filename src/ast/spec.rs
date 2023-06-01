use std::fmt::{Debug, Display};
use std::ops::Deref;

use crate::lexer::span::Spanned;

use super::node::Tree;
use super::token::Token;

#[derive(Default, Clone, Copy, Hash)]
pub struct Node<T>(Option<T>);

pub trait Spec: Sized {
    fn spec(from: Spanned<Tree>) -> Node<Spanned<Self>>;
}

pub trait Terminal: Sized {
    fn spec(from: Spanned<Token>) -> Node<Spanned<Self>>;
}

impl<T> Node<T> {
    pub fn empty() -> Self {
        Self(None)
    }

    pub fn new(value: T) -> Self {
        Self(Some(value))
    }

    pub fn unwrap(self) -> T {
        self.0.unwrap()
    }

    pub fn map<F, U>(self, f: F) -> Node<U>
    where
        F: FnOnce(T) -> U,
    {
        Node(self.0.map(f))
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.0 {
            Some(ref value) => value,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(ref value) => write!(f, "{value}"),
            None => write!(f, "Error"),
        }
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(ref value) => write!(f, "{value:#?}"),
            None => write!(f, "Error"),
        }
    }
}
