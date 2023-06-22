use std::{cell::RefCell, collections::HashMap};

use asena_span::Spanned;

use crate::node::{Child, Tree};

use super::*;

/// A wrapper for the [Tree] to make it mutable and have mutable named children.
///
/// It is used to traverse the tree, and to modify it, and can be an [GreenTree::Error] node,
/// that is used to mark the tree as invalid, and not fail the compiler.
#[derive(Default)]
pub enum GreenTree {
    Leaf {
        data: Spanned<Tree>,

        /// Lazy names' hash map, they have to exist, to make the tree mutable.
        ///
        /// E.g: I can't set the `lhs` node for `binary` tree, if the tree is immutable, so the
        /// lazy names should be used to compute that things.
        /// ```rs
        /// binary.lhs()
        /// ```
        names: RefCell<HashMap<LeafKey, Box<dyn std::any::Any>>>,
    },

    #[default]
    Error,
}

impl GreenTree {
    pub fn new(data: Spanned<Tree>) -> Self {
        Self::Leaf {
            data,
            names: RefCell::new(HashMap::new()),
        }
    }

    /// Memoizes the value of the given function, and returns a new [Cursor] instance, and
    /// if the value is already memoized, it will return the memoized value.
    ///
    /// This function is used to memoize the values of the named children, to make the tree
    /// mutable.
    pub fn memoize<F, T: Leaf + Clone + 'static>(&self, name: &'static str, f: F) -> Cursor<T>
    where
        F: Fn(&Self) -> Cursor<T>,
    {
        let tree @ Self::Leaf { names, .. } = self else {
            return Cursor::empty();
        };

        if let Some(x) = names.borrow().get(name) {
            return x.downcast_ref::<Cursor<T>>().unwrap().clone();
        }

        let cursor = f(tree);
        names.borrow_mut().insert(name, Box::new(cursor.clone()));
        cursor
    }

    /// Returns if the value is the only element in the tree.
    pub fn is_single(&self) -> bool {
        match self {
            GreenTree::Leaf { data, .. } => data.is_single(),
            GreenTree::Error => false,
        }
    }

    /// Returns the tree children, if it's not an error node.
    pub fn children(&mut self) -> Option<&mut Vec<Spanned<Child>>> {
        match self {
            GreenTree::Leaf { data, .. } => Some(&mut data.children),
            GreenTree::Error => None,
        }
    }

    /// Returns filtered cursor to the children, if it's not an error node.
    pub fn filter<T: Leaf>(&self) -> Cursor<Vec<T>> {
        match self {
            GreenTree::Leaf { data, .. } => data.filter(),
            GreenTree::Error => Cursor::empty(),
        }
    }

    /// Returns a terminal node, if it's not an error node.
    pub fn terminal<T: Terminal + Clone>(&self, nth: usize) -> Cursor<T> {
        match self {
            GreenTree::Leaf { data, .. } => data.terminal(nth),
            GreenTree::Error => Cursor::empty(),
        }
    }

    /// Returns terminal filtered cursor to the children, if it's not an error node.
    pub fn filter_terminal<T: Terminal + Leaf>(&self) -> Cursor<Vec<T>> {
        match self {
            GreenTree::Leaf { data, .. } => data.filter_terminal(),
            GreenTree::Error => Cursor::empty(),
        }
    }

    /// Returns a leaf node, if it's not an error node.
    pub fn at<T: Leaf>(&self, nth: usize) -> Cursor<T> {
        match self {
            GreenTree::Leaf { data, .. } => data.at(nth),
            GreenTree::Error => Cursor::empty(),
        }
    }

    /// Returns if the tree has the given name in the current name hash map.
    pub fn has(&self, name: LeafKey) -> bool {
        match self {
            GreenTree::Leaf { names, .. } => matches!(names.borrow().get(name), Some(..)),
            GreenTree::Error => false,
        }
    }

    /// Returns a cursor to the named child, if it's not an error node.
    pub fn named_at<A: Leaf + 'static>(&self, name: LeafKey) -> Cursor<A> {
        match self {
            GreenTree::Leaf { names, .. } => {
                let borrow = names.borrow();
                let Some(child) = borrow.get(name).and_then(|x| x.downcast_ref::<Cursor<A>>()) else {
                    return Cursor::empty();
                };

                let value = child.value.borrow();

                match &*value {
                    Value::Ref(GreenTree::Leaf { data, .. }) => A::make(data.clone()).into(),
                    Value::Ref(GreenTree::Error) => Cursor::empty(),
                    Value::Value(..) => child.clone(),
                }
            }
            GreenTree::Error => Cursor::empty(),
        }
    }

    /// Returns a cursor to the named terminal, if it's not an error node.
    pub fn named_terminal<A: Terminal + Leaf + 'static>(&self, name: LeafKey) -> Cursor<A> {
        match self {
            GreenTree::Leaf { names, .. } => {
                let borrow = names.borrow();
                let Some(child) = borrow.get(name).and_then(|x| x.downcast_ref::<Cursor<A>>()) else {
                    return Cursor::empty();
                };

                let value = child.value.borrow();

                match &*value {
                    Value::Ref(GreenTree::Leaf { data, .. }) => {
                        A::terminal(data.replace(data.single().clone())).into()
                    }
                    Value::Ref(GreenTree::Error) => Cursor::empty(),
                    Value::Value(..) => child.clone(),
                }
            }
            GreenTree::Error => Cursor::empty(),
        }
    }

    pub fn matches(&self, nth: usize, kind: TokenKind) -> bool {
        match self {
            GreenTree::Leaf { data, .. } => data.matches(nth, kind),
            GreenTree::Error => false,
        }
    }

    /// Returns the [Spanned] value of the tree, if it's not an error node, then it should
    /// return the default value.
    pub fn or_empty(self) -> Spanned<Tree> {
        match self {
            GreenTree::Leaf { data, .. } => data,
            GreenTree::Error => Spanned::default(),
        }
    }
}

impl Clone for GreenTree {
    fn clone(&self) -> Self {
        match self {
            Self::Leaf { data, .. } => Self::Leaf {
                data: data.clone(),
                names: RefCell::new(HashMap::new()),
            },
            Self::Error => Self::Error,
        }
    }
}

impl From<Spanned<Tree>> for GreenTree {
    fn from(value: Spanned<Tree>) -> Self {
        GreenTree::new(value)
    }
}
