use std::any::type_name;

use super::{maybe::Maybe, *};

impl<T: Default> Default for Lexeme<T> {
    fn default() -> Self {
        Self {
            token: Default::default(),
            value: Maybe::Just(Default::default()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Maybe::Just(ref value) => std::fmt::Display::fmt(&value, f),
            Maybe::Default(_) => write!(f, "None[{}]", type_name::<T>()),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Maybe::Just(ref value) => std::fmt::Debug::fmt(&value, f),
            Maybe::Default(_) => write!(f, "None[{}]", type_name::<T>()),
        }
    }
}

impl<T: Default> std::ops::Deref for Lexeme<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl<T: Default> std::borrow::Borrow<T> for Lexeme<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}
