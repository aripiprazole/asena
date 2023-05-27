use std::ops::Range;

pub type Loc = Range<usize>;

/// Localized reference in the heap, using [Box], and [Loc], to localize stuff in the source code
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Loc,
    pub value: Box<T>,
}

impl<T> Spanned<T> {
    /// Creates a new [Spanned] with boxed [T].
    pub fn new(span: Loc, value: T) -> Self {
        Self {
            span,
            value: Box::new(value),
        }
    }

    /// Borrow the current location with [Loc]
    pub fn span(&self) -> &Loc {
        &self.span
    }

    /// Borrow the current value
    pub fn value(&self) -> &T {
        &self.value
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
        Spanned::new(self.span, f(*self.value))
    }
}
