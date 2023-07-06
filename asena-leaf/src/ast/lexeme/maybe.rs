use std::ops::{Deref, DerefMut};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Maybe<T> {
    Just(T),
    Default(T),
}

impl<T> Maybe<T> {
    pub fn unwrap(self) -> T {
        match self {
            Maybe::Just(t) => t,
            Maybe::Default(t) => t,
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Maybe<U> {
        match self {
            Maybe::Just(t) => Maybe::Just(f(t)),
            Maybe::Default(t) => Maybe::Default(f(t)),
        }
    }
}

impl<T> DerefMut for Maybe<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Maybe::Just(t) => t,
            Maybe::Default(t) => t,
        }
    }
}

impl<T> Deref for Maybe<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Maybe::Just(t) => t,
            Maybe::Default(t) => t,
        }
    }
}
