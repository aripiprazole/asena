use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Range},
};

pub type Localized<T> = Spanned<Box<T>>;

mod loc;

pub use loc::*;

/// Localized reference in the heap, using [Box], and [Loc], to localize stuff in the source code
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    pub span: Loc,
    pub value: T,
}

impl<T> Spanned<T> {
    /// Creates a new [Spanned]
    pub fn new(span: Loc, value: T) -> Self {
        Self { span, value }
    }

    /// Borrow the current location with [Loc]
    pub fn span(&self) -> &Loc {
        &self.span
    }

    /// Borrow the current value
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn on(self, loc: Loc) -> Self
    where
        T: Clone,
    {
        Spanned::new(loc, self.value)
    }

    pub fn swap<U>(self, value: U) -> Spanned<U> {
        Spanned::new(self.span, value)
    }

    pub fn replace<U>(&self, value: U) -> Spanned<U> {
        Spanned::new(self.span.clone(), value)
    }

    pub fn map<U, F>(self, f: F) -> Spanned<U>
    where
        F: Fn(T) -> U,
        T: Clone,
    {
        Spanned::new(self.span, f(self.value))
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.value())?;
        write!(f, " @ ")?;
        write!(f, "{:?}", self.span())
    }
}
