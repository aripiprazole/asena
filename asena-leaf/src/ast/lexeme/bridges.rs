use super::*;

impl<T: Default> Default for Lexeme<T> {
    fn default() -> Self {
        Self {
            token: Default::default(),
            value: Default::default(),
            is_none: false,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_none {
            write!(f, "None[{}]", std::any::type_name::<T>())
        } else {
            std::fmt::Display::fmt(&self.value, f)
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Lexeme<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_none {
            write!(f, "None[{}]", std::any::type_name::<T>())
        } else {
            std::fmt::Debug::fmt(&self.value, f)
        }
    }
}

impl<T> std::ops::Deref for Lexeme<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::borrow::Borrow<T> for Lexeme<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}
