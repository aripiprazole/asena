use std::hash::Hash;
use std::sync::Arc;
use std::{any::Any, borrow::Cow, collections::HashMap};

use asena_span::Spanned;
use dashmap::DashMap;

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
pub enum GreenTreeKind {
    Leaf(AstLeaf),
    Vec(Vec<GreenTree>),
    Token(Lexeme<Arc<dyn Any + Send + Sync>>),

    /// A node that is supposed to be None.
    None,

    /// An empty node, that is used to mark the tree as invalid, and not fail the compiler.
    Empty,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct GreenTree {
    parent: Arc<Option<GreenTree>>,
    next: Arc<Option<GreenTree>>,
    prev: Arc<Option<GreenTree>>,
    data: GreenTreeKind,
}

impl<T> Cursor<T> {
    pub fn parent(&self) -> Arc<Option<GreenTree>> {
        self.read().parent.clone()
    }

    pub fn set_parent(&self, value: Option<GreenTree>) {
        self.write().parent = Arc::new(value);
    }
}

impl GreenTree {
    pub fn new<I: Into<Arc<Spanned<Tree>>>>(data: I) -> Self {
        let data = data.into();

        Self::new_raw(GreenTreeKind::Leaf(AstLeaf {
            children: compute_named_children(&data),
            names: Arc::new(DashMap::new()),
            keys: Arc::new(DashMap::new()),
            synthetic: false,
            data,
        }))
    }

    /// Creates a new node virtual node, that is not a part of the original tree, with the given
    /// tree kind.
    pub fn of(kind: TreeKind) -> Self {
        let mut data: Spanned<Tree> = Spanned::default();
        data.value.kind = kind;

        Self::new_raw(GreenTreeKind::Leaf(AstLeaf {
            children: HashMap::default(),
            names: Arc::new(DashMap::new()),
            keys: Arc::new(DashMap::new()),
            data: Arc::new(data),
            synthetic: true,
        }))
    }

    /// Creates a new node, based on the this green tree.
    pub fn as_node<T>(&self) -> T
    where
        T: Leaf,
    {
        T::make(self.clone()).unwrap_or_default()
    }

    pub fn parent(&self) -> Arc<Option<GreenTree>> {
        self.parent.clone()
    }

    pub(crate) fn new_raw(data: GreenTreeKind) -> Self {
        Self {
            parent: Default::default(),
            next: Default::default(),
            prev: Default::default(),
            data,
        }
    }

    pub fn data(&self) -> &GreenTreeKind {
        &self.data
    }

    pub fn into_data(self) -> GreenTreeKind {
        self.data
    }
}

impl DerefMut for GreenTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Deref for GreenTree {
    type Target = GreenTreeKind;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Hash for GreenTree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl PartialEq for GreenTree {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for GreenTree {}

impl From<GreenTreeKind> for GreenTree {
    fn from(data: GreenTreeKind) -> Self {
        Self::new_raw(data)
    }
}

impl GreenTreeKind {
    /// Returns a cursor to the named child, if it's not an error node.
    pub fn named_at<A: Leaf + Node + Send + Sync + 'static>(&self, name: LeafKey) -> Cursor<A> {
        let Self::Leaf(leaf) = self else {
            return Cursor::empty();
        };

        let cursor = leaf
            .names
            .get(name)
            .and_then(|value| value.clone().downcast::<Cursor<A>>().ok());
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

        (*child).clone()
    }

    /// Returns a cursor to the named terminal, if it's not an error node.
    pub fn named_terminal<A: Terminal + 'static>(&self, name: LeafKey) -> Cursor<Lexeme<A>>
    where
        A: Send + Sync,
    {
        let Self::Leaf(leaf) = self else {
            return Cursor::empty();
        };

        let cursor = leaf
            .names
            .get(name)
            .and_then(|value| value.clone().downcast::<Cursor<Lexeme<A>>>().ok());
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

        (*child).clone()
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
                names: Arc::new(DashMap::new()),
                keys: Arc::new(DashMap::new()),
            }),
            _ => self.clone(),
        }
    }

    /// Inserts a key into the tree, and returns the value. It's not the same of [GreenTree::insert]
    /// because, [GreenTree::insert] sets in the `names` field
    pub fn dynamic<T: Key + Send + Sync>(&self, key: T, value: T::Value) -> Arc<T::Value> {
        let Self::Leaf(leaf) = self else {
            return Arc::new(value);
        };

        let rc = Arc::new(value);
        leaf.keys.insert(key.name(), rc.clone());
        rc as Arc<T::Value>
    }

    /// Returns the value of the key, if it exists, otherwise it will return the default value.
    pub fn key<T: Key + Send + Sync>(&self, key: T) -> Arc<T::Value> {
        let value = T::Value::default();
        let Self::Leaf(leaf) = self else {
            return Arc::new(value);
        };

        if let Some(value) = leaf.keys.get(key.name()) {
            return value.clone().downcast::<T::Value>().unwrap();
        }

        let rc = Arc::new(value);
        leaf.keys.insert(key.name(), rc.clone());
        rc as Arc<T::Value>
    }

    pub fn insert<T: Send + Sync + 'static>(&self, name: LeafKey, value: T)
    where
        T: Node + Leaf,
    {
        if let Self::Leaf(leaf) = self {
            leaf.names.insert(name, Arc::new(Cursor::of(value)));
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
        T: Node + Sync + Send,
    {
        let tree @ Self::Leaf(leaf) = self else {
            return Cursor::empty();
        };

        if let Some(x) = leaf.names.get(name) {
            return x.downcast_ref::<Cursor<T>>().unwrap().clone();
        }

        let cursor = f(tree);
        leaf.names.insert(name, Arc::new(cursor.clone()));
        cursor
    }
}

impl Default for GreenTree {
    fn default() -> Self {
        Self::new_raw(GreenTreeKind::Leaf(AstLeaf {
            data: Default::default(),
            children: HashMap::new(),
            synthetic: false,
            keys: Arc::new(DashMap::new()),
            names: Arc::new(DashMap::new()),
        }))
    }
}

impl Debug for GreenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data {
            GreenTreeKind::Token(ref lexeme) => f
                .debug_struct("Token")
                .field("kind", &lexeme.token.kind.name())
                .field("value", lexeme)
                .finish(),
            GreenTreeKind::Vec(ref children) => f.debug_tuple("Vec").field(children).finish(),
            GreenTreeKind::Leaf(ref leaf) => write!(f, "Leaf({:#?})", leaf.data),
            GreenTreeKind::Empty => write!(f, "Empty"),
            GreenTreeKind::None => write!(f, "None"),
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
        match self.data {
            GreenTreeKind::Leaf(ref leaf) => leaf.data.tokens(),
            GreenTreeKind::Vec(ref vec) => vec.iter().flat_map(|tree| tree.tokens()).collect(),
            GreenTreeKind::Token(ref lexeme) => vec![lexeme.token.clone()],
            GreenTreeKind::None => vec![],
            GreenTreeKind::Empty => vec![],
        }
    }
}

impl Located for GreenTree {
    fn location(&self) -> Cow<'_, Loc> {
        match self.data {
            GreenTreeKind::Leaf(ref leaf) => Cow::Borrowed(&leaf.data.span),
            GreenTreeKind::Token(ref lexeme) => Cow::Borrowed(&lexeme.token.span),
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
