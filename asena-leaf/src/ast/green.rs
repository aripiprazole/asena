use std::{any::Any, borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use asena_span::Spanned;

use crate::node::{Child, Named, Tree};

use super::*;

/// A wrapper for the [Tree] to make it mutable and have mutable named children.
///
/// It is used to traverse the tree, and to modify it, and can be an [GreenTree::Empty] node,
/// that is used to mark the tree as invalid, and not fail the compiler.
#[derive(Default, Clone)]
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
        names: Rc<RefCell<HashMap<LeafKey, Box<dyn std::any::Any>>>>,
    },

    Token(Lexeme<Rc<dyn Any>>),

    #[default]
    Empty,
}

impl GreenTree {
    pub fn new(data: Spanned<Tree>) -> Self {
        Self::Leaf {
            data,
            names: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn location(&self) -> Cow<'_, Loc> {
        match self {
            GreenTree::Leaf { ref data, .. } => Cow::Borrowed(&data.span),
            GreenTree::Token(ref lexeme) => Cow::Borrowed(&lexeme.token.span),
            GreenTree::Empty => Cow::Owned(Loc::Synthetic),
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
        T: Node,
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
            GreenTree::Token(..) => true,
            GreenTree::Empty => false,
        }
    }

    /// Returns the tree children, if it's not an error node.
    pub fn children(&mut self) -> Option<&mut Vec<Spanned<Child>>> {
        match self {
            GreenTree::Leaf { data, .. } => Some(&mut data.children),
            GreenTree::Token(..) => None,
            GreenTree::Empty => None,
        }
    }

    /// Returns filtered cursor to the children, if it's not an error node.
    pub fn filter<T: Default + Node + Leaf>(&self) -> Cursor<Vec<T>> {
        match self {
            GreenTree::Leaf { data, .. } => data.filter(),
            GreenTree::Token(..) => Cursor::empty(),
            GreenTree::Empty => Cursor::empty(),
        }
    }

    /// Returns a terminal node, if it's not an error node.
    pub fn terminal<T>(&self, nth: usize) -> Cursor<Lexeme<T>>
    where
        T: Debug + Terminal + Default + Clone + 'static,
    {
        match self {
            GreenTree::Leaf { data, .. } => data.terminal(nth),
            GreenTree::Token(..) => Cursor::empty(),
            GreenTree::Empty => Cursor::empty(),
        }
    }

    /// Returns terminal filtered cursor to the children, if it's not an error node.
    pub fn filter_terminal<T>(&self) -> Cursor<Vec<Lexeme<T>>>
    where
        T: Debug + Terminal + Default + Clone + 'static,
    {
        match self {
            GreenTree::Leaf { data, .. } => data.filter_terminal(),
            GreenTree::Token(..) => Cursor::empty(),
            GreenTree::Empty => Cursor::empty(),
        }
    }

    /// Returns a leaf node, if it's not an error node.
    pub fn at<T: Node + Leaf>(&self, nth: usize) -> Cursor<T> {
        match self {
            GreenTree::Leaf { data, .. } => data.at(nth),
            GreenTree::Token(..) => Cursor::empty(),
            GreenTree::Empty => Cursor::empty(),
        }
    }

    /// Returns if the tree has the given name in the current name hash map.
    pub fn has(&self, name: LeafKey) -> bool {
        match self {
            GreenTree::Leaf { names, .. } => matches!(names.borrow().get(name), Some(..)),
            GreenTree::Token(..) => false,
            GreenTree::Empty => false,
        }
    }

    /// Returns a cursor to the named child, if it's not an error node.
    pub fn named_at<A: Node + Leaf + 'static>(&self, name: LeafKey) -> Cursor<A> {
        match self {
            GreenTree::Leaf { names, .. } => {
                let borrow = names.borrow();
                let Some(child) = borrow.get(name).and_then(|x| x.downcast_ref::<Cursor<A>>()) else {
                    return Cursor::empty();
                };

                let value = child.value.borrow();

                match &*value {
                    GreenTree::Leaf { data, .. } => A::make(data.clone()).into(),
                    GreenTree::Token(..) => Cursor::empty(),
                    GreenTree::Empty => Cursor::empty(),
                }
            }
            GreenTree::Token(..) => Cursor::empty(),
            GreenTree::Empty => Cursor::empty(),
        }
    }

    /// Returns a cursor to the named terminal, if it's not an error node.
    pub fn named_terminal<A>(&self, name: LeafKey) -> Cursor<Lexeme<A>>
    where
        A: Debug + Default + Leaf + Terminal + 'static,
    {
        match self {
            GreenTree::Leaf { names, .. } => {
                let borrow = names.borrow();
                let Some(child) = borrow.get(name).and_then(|x| x.downcast_ref::<Cursor<Lexeme<A>>>()) else {
                    return Cursor::empty();
                };

                let value = child.value.borrow();

                match &*value {
                    GreenTree::Leaf { .. } => Cursor::empty(),
                    GreenTree::Token(lexeme) => Lexeme::<A>::terminal(lexeme.token.clone()).into(),
                    GreenTree::Empty => Cursor::empty(),
                }
            }
            GreenTree::Token(..) => Cursor::empty(),
            GreenTree::Empty => Cursor::empty(),
        }
    }

    pub fn matches(&self, nth: usize, kind: TokenKind) -> bool {
        match self {
            GreenTree::Leaf { data, .. } => data.matches(nth, kind),
            GreenTree::Token(..) => false,
            GreenTree::Empty => false,
        }
    }

    /// Returns the [Spanned] value of the tree, if it's not an error node, then it should
    /// return the default value.
    pub fn or_empty(self) -> Spanned<Tree> {
        match self {
            GreenTree::Leaf { data, .. } => data,
            GreenTree::Token(..) => Spanned::default(),
            GreenTree::Empty => Spanned::default(),
        }
    }

    pub fn as_child(self) -> Spanned<Child> {
        match self {
            GreenTree::Leaf { data, .. } => data.map(Child::Tree),
            GreenTree::Token(lexeme) => lexeme.token.map(Child::Token),
            GreenTree::Empty => Spanned::new(Loc::default(), Child::Tree(Tree::default())),
        }
    }
}

impl From<Spanned<Tree>> for GreenTree {
    fn from(value: Spanned<Tree>) -> Self {
        GreenTree::new(value)
    }
}

impl Debug for GreenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf { data, names } => f
                .debug_struct("Leaf")
                .field("data", data)
                .field("names", names)
                .finish(),
            Self::Token(lexeme) => f
                .debug_struct("Token")
                .field("kind", &lexeme.token.kind.name())
                .field("value", lexeme)
                .finish(),
            Self::Empty => write!(f, "Empty"),
        }
    }
}
