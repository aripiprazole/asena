use std::fmt::{Debug, Display};
use std::ops::{ControlFlow, Deref, FromResidual, Try};

use crate::lexer::span::Spanned;

use super::node::Tree;
use super::token::Token;

#[derive(Default, Clone, Copy, Hash)]
pub struct Node<T>(Option<T>);

pub trait Spec: Sized {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>>;
}

pub trait Terminal: Sized {
    fn terminal(from: Spanned<Token>) -> Node<Spanned<Self>>;
}

impl<T: Terminal> Spec for T {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        let token = from.single().clone();

        <T as Terminal>::terminal(from.swap(token))
    }
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

    pub fn is_empty(&self) -> bool {
        matches!(self.0, None)
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

impl<T> Try for Node<T> {
    type Output = T;
    type Residual = Node<std::convert::Infallible>;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Node(Some(output))
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Some(v) => ControlFlow::Continue(v),
            None => ControlFlow::Break(Node::empty()),
        }
    }
}

impl<T> FromResidual for Node<T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual.0 {
            Some(..) => unreachable!(),
            None => Self::empty(),
        }
    }
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Node(Some(value))
    }
}

impl<T> From<Option<T>> for Node<T> {
    fn from(value: Option<T>) -> Self {
        Node(value)
    }
}

impl<T> From<Option<Node<T>>> for Node<T> {
    fn from(value: Option<Node<T>>) -> Self {
        match value {
            Some(value) => Node(value.0),
            None => Node::empty(),
        }
    }
}
