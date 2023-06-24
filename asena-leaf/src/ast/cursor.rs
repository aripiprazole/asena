use std::rc::Rc;

use super::*;

/// A cursor is a reference to a node in the tree.
///
/// It is used to traverse the tree, and to modify it.
pub struct Cursor<T> {
    pub(crate) value: Arc<RefCell<Value<T>>>,
}

impl<T: Leaf> Cursor<T> {
    /// Creates a new cursor without any value.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Updates the value of the current cursor with a new [Cursor].
    pub fn set(&self, value: Cursor<T>) {
        self.value.replace(value.value.borrow().clone());
    }

    /// Creates a new cursor with the given value.
    pub fn of(value: T) -> Self {
        Self {
            value: Arc::new(RefCell::new(Value::Value(Rc::new(value)))),
        }
    }

    /// Creates a new cursor with the given [Rc] value.
    pub fn from_rc(value: Rc<T>) -> Self {
        Self {
            value: Arc::new(RefCell::new(Value::Value(value))),
        }
    }

    /// Creates a new cursor with a reference to the `concrete syntax tree`, using
    /// the wrapper [GreenTree].
    pub fn new<I: Into<GreenTree>>(value: I) -> Self {
        let tree: GreenTree = value.into();

        Self {
            value: Arc::new(RefCell::new(Value::Ref(tree))),
        }
    }

    /// Deeply duplicates the current cursor and returns a new [Cursor] instance.
    pub fn as_new_node(&self) -> Self
    where
        T: Clone,
    {
        let new_value = self.value.borrow().clone();

        Self {
            value: Arc::new(RefCell::new(new_value)),
        }
    }

    pub fn location(&self) -> Spanned<Rc<T>>
    where
        T: Default,
        T: Located,
    {
        match &*self.value.borrow() {
            Value::Ref(GreenTree::Leaf { data, .. }) => match T::make(data.clone()).map(Rc::new) {
                Some(value) => data.replace(value),
                None => Spanned::default(),
            },
            Value::Ref(GreenTree::Error) => Spanned::default(),
            Value::Value(value) => {
                let location: Loc = value.location().into_owned();

                Spanned::new(location, value.clone())
            }
        }
    }

    /// Returns the current cursor if it's not empty, otherwise returns [None].
    pub fn try_as_leaf(&self) -> Option<Rc<T>>
    where
        T: Clone,
    {
        match &*self.value.borrow() {
            Value::Ref(GreenTree::Leaf { data, .. }) => T::make(data.clone()).map(Rc::new),
            Value::Ref(GreenTree::Error) => None,
            Value::Value(value) => Some(value.clone()),
        }
    }

    /// Returns the current cursor if it's not empty, otherwise returns a default value.
    pub fn as_leaf(&self) -> Rc<T>
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

impl<T: Leaf> Default for Cursor<T> {
    fn default() -> Self {
        Self {
            value: Arc::new(RefCell::new(Default::default())),
        }
    }
}

impl<T: Leaf> Cursor<Vec<T>> {
    pub fn first(self) -> Cursor<T> {
        self.as_leaf().first().cloned().into()
    }

    pub fn skip(self, n: usize) -> Cursor<Vec<T>> {
        self.as_leaf()
            .iter()
            .skip(n)
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }
}

impl<T: Leaf> From<Vec<T>> for Cursor<Vec<T>> {
    fn from(value: Vec<T>) -> Self {
        Cursor::of(value)
    }
}

impl<T: Leaf> From<Option<T>> for Cursor<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::of(value),
            None => Self::empty(),
        }
    }
}

impl<T: Leaf + Display + Default> Display for Cursor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_leaf())
    }
}

impl<T: Leaf + Debug + Default> Debug for Cursor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cursor({:?})", self.as_leaf())
    }
}

impl<T: Leaf> From<GreenTree> for Cursor<T> {
    fn from(value: GreenTree) -> Self {
        Cursor::new(value)
    }
}

impl<T: Leaf> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T: Leaf> FromResidual for Cursor<T> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual {
            Some(_) => unreachable!(),
            None => Cursor::empty(),
        }
    }
}

impl<T: Leaf> Try for Cursor<T> {
    type Output = Rc<T>;

    type Residual = Option<std::convert::Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Self::from_rc(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match &*self.value.borrow() {
            Value::Ref(GreenTree::Leaf { data, .. }) => match T::make(data.clone()) {
                Some(value) => ControlFlow::Continue(Rc::new(value)),
                None => ControlFlow::Break(None),
            },
            Value::Ref(GreenTree::Error) => ControlFlow::Break(None),
            Value::Value(value) => ControlFlow::Continue(value.clone()),
        }
    }
}
