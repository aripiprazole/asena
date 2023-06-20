use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use asena_span::Spanned;

use super::ast::{Cursor, Spec, Terminal};
use super::node::{Child, Tree};

pub type GreenChild = Green<Box<dyn Any>>;

#[derive(Default)]
pub struct GreenTree {
    /// TODO: Use tree cursor instead of using directly the [Spanned] tree's [Tree], to invalidate
    /// `lazy_names` references.
    tree: Spanned<Tree>,

    /// Children marked with name, to be accessed fast.
    children: HashMap<&'static str, Spanned<Child>>,

    /// Lazy names' hash map, they have to exist, to make the tree mutable.
    ///
    /// E.g: I can't set the `lhs` node for `binary` tree, if the tree is immutable, so the
    /// lazy names should be used to compute that things.
    /// ```rs
    /// binary.lhs()
    /// ```
    lazy_names: RefCell<HashMap<&'static str, Box<dyn Any>>>,
}

#[derive(Clone)]
pub struct Green<T>(Arc<RefCell<T>>, PhantomData<T>);

impl<T: Debug + Clone> Green<T> {
    pub fn get(&self) -> T {
        self.0.borrow().clone()
    }

    pub fn duplicate(&self) -> Green<T> {
        Green(Arc::new(RefCell::new(self.0.borrow().clone())), PhantomData)
    }

    pub fn set(&self, new_value: T) {
        self.0.replace(new_value);
    }

    pub fn replace(&self, new_value: Green<T>) {
        self.0.replace(new_value.get());
    }
}

impl GreenTree {
    pub fn new(tree: Spanned<Tree>) -> Self {
        let named_children = Self::compute_named_children(&tree);

        Self {
            tree,
            children: named_children,
            lazy_names: Default::default(),
        }
    }
}

impl From<Spanned<Tree>> for GreenTree {
    fn from(value: Spanned<Tree>) -> Self {
        Self::new(value)
    }
}

impl DerefMut for GreenTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

impl Deref for GreenTree {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl Clone for GreenTree {
    fn clone(&self) -> Self {
        Self {
            lazy_names: Default::default(),
            tree: self.tree.clone(),
            children: self.children.clone(),
        }
    }
}

impl Hash for GreenTree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tree.hash(state);
    }
}

impl Debug for GreenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree.fmt(f)
    }
}

impl<T: Clone + Debug> Debug for Green<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Green({:#?})", self.get())
    }
}
