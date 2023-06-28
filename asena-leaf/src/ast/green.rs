use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use asena_span::Spanned;

use crate::node::{Child, Named, Tree, TreeKind};

use super::*;

/// A wrapper for the [Tree] to make it mutable and have mutable named children.
///
/// It is used to traverse the tree, and to modify it, and can be an [GreenTree::Empty] node,
/// that is used to mark the tree as invalid, and not fail the compiler.
#[derive(Clone)]
pub enum GreenTree {
    Leaf {
        data: Spanned<Tree>,

        children: HashMap<LeafKey, Spanned<Child>>,

        synthetic: bool,

        /// Lazy names' hash map, they have to exist, to make the tree mutable.
        ///
        /// E.g: I can't set the `lhs` node for `binary` tree, if the tree is immutable, so the
        /// lazy names should be used to compute that things.
        /// ```rs
        /// binary.lhs()
        /// ```
        names: Rc<RefCell<HashMap<LeafKey, Rc<dyn std::any::Any>>>>,
    },

    Vec(Vec<GreenTree>),

    /// A terminal node
    Token(Lexeme<Rc<dyn std::any::Any>>),

    /// A node that is supposed to be None.
    None,

    /// An empty node, that is used to mark the tree as invalid, and not fail the compiler.
    Empty,
}

impl Default for GreenTree {
    fn default() -> Self {
        Self::Leaf {
            data: Spanned::default(),
            children: HashMap::new(),
            synthetic: false,
            names: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl GreenTree {
    pub fn new(data: Spanned<Tree>) -> Self {
        Self::Leaf {
            children: compute_named_children(&data),
            names: Rc::new(RefCell::new(HashMap::new())),
            synthetic: false,
            data,
        }
    }

    pub fn of(kind: TreeKind) -> Self {
        let mut data: Spanned<Tree> = Spanned::default();
        data.value.kind = kind;

        Self::Leaf {
            children: HashMap::default(),
            names: Rc::new(RefCell::new(HashMap::new())),
            synthetic: true,
            data,
        }
    }

    pub fn as_new_node(&self) -> Self {
        match self {
            Self::Leaf {
                data, synthetic, ..
            } => Self::Leaf {
                children: compute_named_children(data),
                names: Rc::new(RefCell::new(HashMap::new())),
                synthetic: *synthetic,
                data: data.clone(),
            },
            _ => self.clone(),
        }
    }

    pub fn insert<T: 'static>(&self, name: LeafKey, value: T)
    where
        T: Node + Leaf,
    {
        if let Self::Leaf { names, .. } = self {
            names.borrow_mut().insert(name, Rc::new(Cursor::of(value)));
        }
    }

    pub fn location(&self) -> Cow<'_, Loc> {
        match self {
            Self::Leaf { ref data, .. } => Cow::Borrowed(&data.span),
            Self::Token(ref lexeme) => Cow::Borrowed(&lexeme.token.span),
            _ => Cow::Owned(Loc::Synthetic),
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
        names.borrow_mut().insert(name, Rc::new(cursor.clone()));
        cursor
    }

    /// Returns if the value is the only element in the tree.
    pub fn is_single(&self) -> bool {
        match self {
            Self::Leaf { data, .. } => data.is_single(),
            Self::Token(..) => true,
            _ => false,
        }
    }

    /// Returns the tree children, if it's not an error node.
    pub fn children(&mut self) -> Option<&mut Vec<Spanned<Child>>> {
        match self {
            Self::Leaf { data, .. } => Some(&mut data.children),
            _ => None,
        }
    }

    /// Returns filtered cursor to the children, if it's not an error node.
    pub fn filter<T: Leaf + Node>(&self) -> Cursor<Vec<T>> {
        match self {
            Self::Leaf { data, .. } => data.filter(),
            _ => Cursor::empty(),
        }
    }

    /// Returns a terminal node, if it's not an error node.
    pub fn terminal<T: Terminal + 'static>(&self, nth: usize) -> Cursor<Lexeme<T>> {
        match self {
            Self::Leaf { data, .. } => data.terminal(nth),
            _ => Cursor::empty(),
        }
    }

    /// Returns terminal filtered cursor to the children, if it's not an error node.
    pub fn filter_terminal<T: Terminal + 'static>(&self) -> Cursor<Vec<Lexeme<T>>> {
        match self {
            Self::Leaf { data, .. } => data.filter_terminal(),
            _ => Cursor::empty(),
        }
    }

    /// Returns a leaf node, if it's not an error node.
    pub fn at<T: Node + Leaf>(&self, nth: usize) -> Cursor<T> {
        match self {
            Self::Leaf { data, .. } => data.at(nth),
            _ => Cursor::empty(),
        }
    }

    /// Returns if the tree has the given name in the current name hash map.
    pub fn has(&self, name: LeafKey) -> bool {
        match self {
            Self::Leaf { children, .. } => matches!(children.get(name), Some(..)),
            _ => false,
        }
    }

    /// Returns a cursor to the named child, if it's not an error node.
    pub fn named_at<A: Leaf + Node + 'static>(&self, name: LeafKey) -> Cursor<A> {
        match self {
            Self::Leaf {
                names, children, ..
            } => {
                let borrow = names.borrow();
                let Some(child) = borrow.get(name).and_then(|x| x.downcast_ref::<Cursor<A>>()) else {
                    return match children.get(name) {
                        Some(Spanned { value: Child::Token(..), .. }) => Cursor::empty(),
                        Some(spanned @ Spanned { value: Child::Tree(ref tree), .. }) => {
                            A::make(GreenTree::new(spanned.replace(tree.clone()))).into()
                        },
                        None => Cursor::empty(),
                    };
                };

                child.clone()
            }
            _ => Cursor::empty(),
        }
    }

    /// Returns a cursor to the named terminal, if it's not an error node.
    pub fn named_terminal<A: Terminal + 'static>(&self, name: LeafKey) -> Cursor<Lexeme<A>> {
        match self {
            Self::Leaf {
                names, children, ..
            } => {
                let borrow = names.borrow();
                let Some(child) = borrow.get(name).and_then(|x| x.downcast_ref::<Cursor<Lexeme<A>>>()) else {
                    return match children.get(name) {
                        Some(Spanned { value: Child::Tree(..), .. }) => Cursor::empty(),
                        Some(spanned @ Spanned { value: Child::Token(ref token), .. }) => {
                            Lexeme::<A>::terminal(spanned.replace(token.clone())).into()
                        },
                        None => Cursor::empty(),
                    };
                };

                child.clone()
            }
            _ => Cursor::empty(),
        }
    }

    pub fn matches(&self, nth: usize, kind: TokenKind) -> bool {
        match self {
            Self::Leaf { data, .. } => data.matches(nth, kind),
            _ => false,
        }
    }

    /// Returns the [Spanned] value of the tree, if it's not an error node, then it should
    /// return the default value.
    pub fn or_empty(self) -> Spanned<Tree> {
        match self {
            Self::Leaf { data, .. } => data,
            _ => Spanned::default(),
        }
    }

    pub fn as_child(self) -> Spanned<Child> {
        match self {
            Self::Leaf { data, .. } => data.map(Child::Tree),
            Self::Token(lexeme) => lexeme.token.map(Child::Token),
            _ => Spanned::new(Loc::default(), Child::Tree(Tree::default())),
        }
    }

    pub fn kind(&self) -> TreeKind {
        match self {
            Self::Leaf { data, .. } => data.kind,
            _ => TreeKind::Error,
        }
    }
}

impl From<Spanned<Tree>> for GreenTree {
    fn from(value: Spanned<Tree>) -> Self {
        Self::new(value)
    }
}

impl Debug for GreenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf {
                data,
                names,
                children,
                synthetic,
            } => f
                .debug_struct("Leaf")
                .field("data", data)
                .field("synthetic", synthetic)
                .field("names", names)
                .field("children", children)
                .finish(),
            Self::Token(lexeme) => f
                .debug_struct("Token")
                .field("kind", &lexeme.token.kind.name())
                .field("value", lexeme)
                .finish(),
            Self::Vec(children) => f.debug_tuple("Vec").field(children).finish(),
            Self::Empty => write!(f, "Empty"),
            Self::None => write!(f, "None"),
        }
    }
}

fn compute_named_children(data: &Spanned<Tree>) -> HashMap<LeafKey, Spanned<Child>> {
    let mut named_children = HashMap::new();

    for child in &data.children {
        match child.value() {
            Child::Tree(tree) => {
                if let Some(name) = tree.name {
                    named_children.insert(name, child.clone());
                }
            }
            Child::Token(token) => {
                if let Some(name) = token.name {
                    named_children.insert(name, child.clone());
                }
            }
        }
    }

    named_children
}
