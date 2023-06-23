use std::{
    cell::{RefCell, RefMut},
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use super::{GreenTree, Leaf};

#[derive(Clone)]
pub enum Value<T> {
    Ref(GreenTree),
    Value(T),
}

impl<T> Default for Value<T> {
    fn default() -> Self {
        Value::Ref(Default::default())
    }
}

pub enum BowCell<'a, T> {
    Owned(RefCell<T>),
    Bow(Bow<'a, T>),
}

impl<'a, T: Clone> BowCell<'a, T> {
    pub fn new(value: T) -> Self {
        Self::Owned(RefCell::new(value))
    }

    pub fn borrow(&self) -> Bow<'_, T> {
        match self {
            BowCell::Owned(value) => Bow::Borrowed(value.borrow_mut()),
            BowCell::Bow(value) => value.clone(),
        }
    }

    pub fn borrow_mut(&self) -> Bow<'_, T> {
        self.borrow()
    }
}

pub enum Bow<'a, T: Sized + 'a> {
    Owned(T),
    Borrowed(RefMut<'a, T>),
}

impl<T> From<T> for Bow<'_, T> {
    fn from(value: T) -> Self {
        Bow::Owned(value)
    }
}

impl<'a, T: 'a> Bow<'a, T> {
    // Mark to error
    pub fn unsafe_copy<'b>(&self) -> Bow<'b, T> {
        unsafe {
            let new_borrow = std::mem::transmute_copy(self);

            Bow::Borrowed(new_borrow)
        }
    }

    pub fn map<U: Clone, F>(self, f: F) -> Bow<'a, U>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        match self {
            Bow::Owned(mut value) => Bow::Owned(f(&mut value).clone()),
            Bow::Borrowed(borrowed) => Bow::Borrowed(RefMut::map(borrowed, f)),
        }
    }
}

impl<'a, T: 'a> DerefMut for Bow<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Bow::Owned(ref mut value) => value,
            Bow::Borrowed(value) => value.deref_mut(),
        }
    }
}

impl<'a, T: 'a> Deref for Bow<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Bow::Owned(ref value) => value,
            Bow::Borrowed(value) => value.deref(),
        }
    }
}

impl<T: Default> Default for Bow<'_, T> {
    fn default() -> Self {
        Bow::Owned(Default::default())
    }
}

impl<T: Display> Display for Bow<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bow::Owned(owned) => owned.fmt(f),
            Bow::Borrowed(borrowed) => borrowed.fmt(f),
        }
    }
}

impl<T: Debug> Debug for Bow<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bow::Owned(owned) => owned.fmt(f),
            Bow::Borrowed(borrowed) => borrowed.fmt(f),
        }
    }
}

// Mark to error
impl<T: Clone> Clone for Bow<'_, T> {
    fn clone(&self) -> Self {
        match self {
            Bow::Owned(owned) => Bow::Owned(owned.clone()),
            Bow::Borrowed(borrowed) => unsafe {
                let new_borrow = std::mem::transmute_copy(borrowed);

                Bow::Borrowed(new_borrow)
            },
        }
    }
}
