use std::sync::{Arc, RwLock};
use std::{any::Any, borrow::Cow, collections::HashMap, rc::Rc};

use asena_span::Spanned;

use crate::node::{Child, Named, Tree, TreeKind};
use crate::token::token_set::HasTokens;

use super::*;

mod ast_leaf;
mod bridges;

pub use ast_leaf::*;

/// A wrapper for the [Tree] to make it mutable and have mutable named children.
///
/// It is used to traverse the tree, and to modify it, and can be an [GreenTree::Empty] node,
/// that is used to mark the tree as invalid, and not fail the compiler.
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum GreenTree {
    Leaf(AstLeaf),
    Vec(Vec<GreenTree>),
    Token(Lexeme<Rc<dyn Any>>),

    /// A node that is supposed to be None.
    None,

    /// An empty node, that is used to mark the tree as invalid, and not fail the compiler.
    Empty,
}

impl GreenTree {
    pub fn new<I: Into<Arc<Spanned<Tree>>>>(data: I) -> Self {
        let data = data.into();

        Self::Leaf(AstLeaf {
            children: compute_named_children(&data),
            names: AstLeaf::new_ref(HashMap::new()),
            keys: AstLeaf::new_ref(HashMap::new()),
            synthetic: false,
            data,
        })
    }

    /// Creates a new node virtual node, that is not a part of the original tree, with the given
    /// tree kind.
    pub fn of(kind: TreeKind) -> Self {
        let mut data: Spanned<Tree> = Spanned::default();
        data.value.kind = kind;

        Self::Leaf(AstLeaf {
            children: HashMap::default(),
            names: AstLeaf::new_ref(HashMap::new()),
            keys: AstLeaf::new_ref(HashMap::new()),
            data: Arc::new(data),
            synthetic: true,
        })
    }

    /// Creates a new node, based on the this green tree.
    pub fn as_node<T>(&self) -> T
    where
        T: Leaf,
    {
        Leaf::make(self.clone()).unwrap_or_default()
    }

    /// Returns a cursor to the named child, if it's not an error node.
    pub fn named_at<A: Leaf + Node + 'static>(&self, name: LeafKey) -> Cursor<A> {
        let Self::Leaf(leaf) = self else {
            return Cursor::empty();
        };

        let names = leaf.names();
        let cursor = names.get(name);
        let cursor = cursor.and_then(|value| value.downcast_ref::<Cursor<A>>());
        let Some(child) = cursor else {
            let Some(child) = leaf.children.get(name) else {
                return Cursor::empty();
            };

            return match &child.value {
                Child::Token(..) => Cursor::empty(),
                Child::Tree(ref tree) => {
                    A::make(GreenTree::new(child.replace(tree.clone()))).into()
                }
            }
        };

        child.clone()
    }

    /// Returns a cursor to the named terminal, if it's not an error node.
    pub fn named_terminal<A: Terminal + 'static>(&self, name: LeafKey) -> Cursor<Lexeme<A>> {
        let Self::Leaf(leaf) = self else {
            return Cursor::empty();
        };

        let names = leaf.names();
        let cursor = names.get(name);
        let cursor = cursor.and_then(|value| value.downcast_ref::<Cursor<Lexeme<A>>>());
        let Some(child) = cursor else {
            let Some(child) = leaf.children.get(name) else {
                return Cursor::empty();
            };

            return match child.value {
                Child::Tree(..) => Cursor::empty(),
                Child::Token(ref token) => {
                    Lexeme::<A>::terminal(child.replace(token.clone())).into()
                }
            }
        };

        child.clone()
    }

    /// Creates a new node from the current node, if it's a leaf node, it will reset the names, and
    /// keys hash maps, and it will compute the named children again, to really duplicate the node,
    /// use [GreenTree::clone].
    ///
    /// This method is useful to create a new node from a leaf node, and then insert it into the
    /// tree.
    pub fn as_new_node(&self) -> Self {
        match self {
            Self::Leaf(leaf) => Self::Leaf(AstLeaf {
                data: leaf.data.clone(),
                synthetic: leaf.synthetic,
                children: compute_named_children(&leaf.data),
                names: AstLeaf::new_ref(HashMap::new()),
                keys: AstLeaf::new_ref(HashMap::new()),
            }),
            _ => self.clone(),
        }
    }

    /// Inserts a key into the tree, and returns the value. It's not the same of [GreenTree::insert]
    /// because, [GreenTree::insert] sets in the `names` field
    pub fn dynamic<T: Key>(&self, key: T, value: T::Value) -> Rc<T::Value> {
        let Self::Leaf(leaf) = self else {
            return Rc::new(value);
        };

        let rc = Rc::new(value);
        leaf.keys_mut().insert(key.name(), rc.clone());
        rc as Rc<T::Value>
    }

    /// Returns the value of the key, if it exists, otherwise it will return the default value.
    pub fn key<T: Key>(&self, key: T) -> Rc<T::Value> {
        let value = T::Value::default();
        let Self::Leaf(leaf) = self else {
            return Rc::new(value);
        };

        if let Some(value) = leaf.keys().get(key.name()) {
            return value.clone().downcast::<T::Value>().unwrap();
        }

        let rc = Rc::new(value);
        leaf.keys_mut().insert(key.name(), rc.clone());
        rc as Rc<T::Value>
    }

    pub fn insert<T: 'static>(&self, name: LeafKey, value: T)
    where
        T: Node + Leaf,
    {
        if let Self::Leaf(leaf) = self {
            leaf.names_mut().insert(name, Arc::new(Cursor::of(value)));
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
        let tree @ Self::Leaf(leaf) = self else {
            return Cursor::empty();
        };

        if let Some(x) = leaf.names().get(name) {
            return x.downcast_ref::<Cursor<T>>().unwrap().clone();
        }

        let cursor = f(tree);
        leaf.names_mut().insert(name, Arc::new(cursor.clone()));
        cursor
    }
}

impl Default for GreenTree {
    fn default() -> Self {
        Self::Leaf(AstLeaf {
            data: Default::default(),
            children: HashMap::new(),
            synthetic: false,
            keys: AstLeaf::new_ref(HashMap::new()),
            names: AstLeaf::new_ref(HashMap::new()),
        })
    }
}

impl Debug for GreenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(lexeme) => f
                .debug_struct("Token")
                .field("kind", &lexeme.token.kind.name())
                .field("value", lexeme)
                .finish(),
            Self::Vec(children) => f.debug_tuple("Vec").field(children).finish(),
            Self::Leaf(leaf) => write!(f, "Leaf({:#?})", leaf.data),
            Self::Empty => write!(f, "Empty"),
            Self::None => write!(f, "None"),
        }
    }
}

impl From<Spanned<Tree>> for GreenTree {
    fn from(value: Spanned<Tree>) -> Self {
        Self::new(value)
    }
}

impl HasTokens for GreenTree {
    fn tokens(&self) -> Vec<Spanned<Token>> {
        match self {
            Self::Leaf(leaf) => leaf.data.tokens(),
            Self::Vec(vec) => vec.iter().flat_map(|tree| tree.tokens()).collect(),
            Self::Token(lexeme) => vec![lexeme.token.clone()],
            Self::None => vec![],
            Self::Empty => vec![],
        }
    }
}

impl Located for GreenTree {
    fn location(&self) -> Cow<'_, Loc> {
        match self {
            Self::Leaf(leaf) => Cow::Borrowed(&leaf.data.span),
            Self::Token(ref lexeme) => Cow::Borrowed(&lexeme.token.span),
            _ => Cow::Owned(Loc::default()),
        }
    }
}

/// Computes the named children of the given tree, and returns a hash map with the named children.
///
/// This function is used to compute the tree that the `name` property is not [None].
fn compute_named_children(data: &Spanned<Tree>) -> HashMap<LeafKey, Arc<Spanned<Child>>> {
    let mut named_children = HashMap::new();

    for child in &data.children {
        match child.value() {
            Child::Tree(tree) => {
                if let Some(name) = tree.name {
                    named_children.insert(name, Arc::new(child.clone()));
                }
            }
            Child::Token(token) => {
                if let Some(name) = token.name {
                    named_children.insert(name, Arc::new(child.clone()));
                }
            }
        }
    }

    named_children
}
