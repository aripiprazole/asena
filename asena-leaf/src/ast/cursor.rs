use std::{cell::RefMut, marker::PhantomData, rc::Rc};

use super::*;

/// A cursor is a reference to a node in the tree.
///
/// It is used to traverse the tree, and to modify it.
pub struct Cursor<'a, T> {
    pub(crate) value: Rc<BowCell<'a, Value<T>>>,
}

impl<'a, T: Leaf> Cursor<'a, T> {
    /// Creates a new cursor without any value.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Updates the value of the current cursor with a new [Cursor].
    pub fn set(&self, value: Cursor<'a, T>)
    where
        T: Default,
    {
        let mut reference = self.value.borrow_mut();
        let awa = reference.deref_mut();

        *awa = Value::Value(value.as_leaf().deref().clone());
    }

    pub fn dup<'b>(&self) -> Cursor<'b, T> {
        unsafe { std::mem::transmute_copy(&self) }
    }

    /// Creates a new cursor with the given value.
    pub fn of(value: T) -> Self {
        Self {
            value: Rc::new(BowCell::new(Value::Value(value))),
        }
    }

    pub fn from_bow(bow: Bow<'a, T>) -> Cursor<'_, T> {
        let value = (*bow).clone();

        Self {
            value: Rc::new(BowCell::new(Value::Value(value))),
        }
    }

    /// Creates a new cursor with a reference to the `concrete syntax tree`, using
    /// the wrapper [GreenTree].
    pub fn new<I: Into<GreenTree>>(value: I) -> Self {
        let tree: GreenTree = value.into();

        Self {
            value: Rc::new(BowCell::new(Value::Ref(tree))),
        }
    }

    /// Deeply duplicates the current cursor and returns a new [Cursor] instance.
    pub fn as_new_node(&self) -> Self
    where
        T: Clone,
    {
        Self {
            value: Rc::new(BowCell::new((*self.value.borrow()).clone())),
        }
    }

    /// Returns the current cursor if it's not empty, otherwise returns [None].
    pub fn to_leaf<'b>(self) -> Option<Bow<'b, T>>
    where
        T: Clone,
    {
        let bm = self.value.borrow_mut();

        match &*bm {
            Value::Ref(GreenTree::Leaf { data, .. }) => T::make(data.clone()).map(Bow::Owned),
            Value::Ref(GreenTree::Error) => None,
            Value::Value(..) => {
                let new_borrow = bm.map(|value| match value {
                    Value::Ref(_) => unreachable!(),
                    Value::Value(value) => value,
                });

                Some(new_borrow.unsafe_copy())
            }
        }
    }
    pub fn try_as_leaf(&self) -> Option<Bow<T>>
    where
        T: Clone,
    {
        let bm = self.value.borrow_mut();

        match &*bm {
            Value::Ref(GreenTree::Leaf { data, .. }) => T::make(data.clone()).map(Bow::Owned),
            Value::Ref(GreenTree::Error) => None,
            Value::Value(..) => {
                let new_borrow = bm.map(|value| match value {
                    Value::Ref(_) => unreachable!(),
                    Value::Value(value) => value,
                });

                Some(new_borrow)
            }
        }
    }

    /// Returns the current cursor if it's not empty, otherwise returns a default value.
    pub fn as_leaf(&self) -> Bow<T>
    where
        T: Clone + Default,
    {
        self.try_as_leaf().unwrap_or_default()
    }

    /// Returns the current cursor if it's not empty, otherwise returns false.
    pub fn is_empty(&self) -> bool {
        match &*self.value.borrow() {
            Value::Ref(GreenTree::Leaf { .. }) => true,
            Value::Ref(GreenTree::Error) => false,
            Value::Value(..) => true,
        }
    }
}

impl<'a, T: Leaf> Default for Cursor<'a, T> {
    fn default() -> Self {
        Self {
            value: Rc::new(BowCell::new(Default::default())),
        }
    }
}

impl<'a, T: Leaf> Cursor<'a, Vec<T>> {
    pub fn first(self) -> Cursor<'a, T> {
        self.as_leaf().first().cloned().into()
    }
}

impl<'a, T: Leaf> From<Vec<T>> for Cursor<'a, Vec<T>> {
    fn from(value: Vec<T>) -> Self {
        Cursor::of(value)
    }
}

impl<'a, T: Leaf> From<Option<T>> for Cursor<'a, T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::of(value),
            None => Self::empty(),
        }
    }
}

impl<'a, T: Leaf + Display + Default> Display for Cursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_leaf())
    }
}

impl<'a, T: Leaf + Debug + Default> Debug for Cursor<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cursor({:?})", self.as_leaf())
    }
}

impl<'a, T: Leaf> From<GreenTree> for Cursor<'a, T> {
    fn from(value: GreenTree) -> Self {
        Cursor::new(value)
    }
}

impl<'a, T: Leaf> Clone for Cursor<'a, T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<'a, T: Leaf> FromResidual for Cursor<'a, T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual {
            Some(_) => unreachable!(),
            None => Cursor::empty(),
        }
    }
}

impl<'a, T: Leaf> Try for Cursor<'a, T> {
    type Output = Bow<'a, T>;

    type Residual = Option<std::convert::Infallible>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        let bm = self.value.borrow();
        match &*bm {
            Value::Ref(GreenTree::Leaf { data, .. }) => match T::make(data.clone()) {
                Some(value) => ControlFlow::Continue(Bow::Owned(value)),
                None => ControlFlow::Break(None),
            },
            Value::Ref(GreenTree::Error) => ControlFlow::Break(None),
            Value::Value(..) => {
                let new_borrow = bm.map(|value| match value {
                    Value::Ref(_) => unreachable!(),
                    Value::Value(value) => value,
                });

                ControlFlow::Continue(new_borrow.unsafe_copy())
            }
        }
    }
}
