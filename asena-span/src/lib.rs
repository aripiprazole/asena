use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Range},
};

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum Loc {
    #[default]
    Synthetic,
    Concrete(Range<usize>),
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Loc::Synthetic => write!(f, "*synthetic*"),
            Loc::Concrete(range) => write!(f, "{}", range.start),
        }
    }
}

pub trait Span {
    fn on(&self, end: Loc) -> Self;
}

pub type Localized<T> = Spanned<Box<T>>;

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

impl Loc {
    pub fn into_ranged(self) -> Option<Range<usize>> {
        match self {
            Self::Concrete(range) => Some(range),
            _ => None,
        }
    }
}

impl From<Range<usize>> for Loc {
    fn from(value: Range<usize>) -> Self {
        Loc::Concrete(value)
    }
}

impl Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Synthetic => write!(f, "Synthetic"),
            Self::Concrete(range) => write!(f, "{:?}", range),
        }
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.value())?;
        write!(f, " @ ")?;
        write!(f, "{:?}", self.span())
    }
}

impl Span for Loc {
    fn on(&self, end: Loc) -> Self {
        match (self, end) {
            (Loc::Concrete(a), Loc::Concrete(b)) => Loc::Concrete(a.start..b.end),
            (_, _) => Loc::Synthetic,
        }
    }
}
