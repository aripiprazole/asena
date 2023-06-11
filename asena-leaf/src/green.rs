use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use asena_span::Spanned;

use super::node::{Child, Tree};
use super::spec::{Node, Spec, Terminal};

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

    pub fn has(&self, name: &'static str) -> bool {
        matches!(self.children.get(name), Some(..))
    }

    pub fn named_at<T: Spec>(&self, name: &'static str) -> Node<Spanned<T>> {
        let Some(child) = self.children.get(name) else {
            return Node::empty();
        };

        match &child.value {
            Child::Tree(tree) => T::make(child.replace(tree.clone())),
            Child::Token(..) => Node::empty(),
        }
    }

    pub fn named_terminal<T: Terminal>(&self, name: &'static str) -> Node<Spanned<T>> {
        let Some(child) = self.children.get(name) else {
            return Node::empty();
        };

        match &child.value {
            Child::Tree(..) => Node::empty(),
            Child::Token(token) => T::terminal(child.replace(token.clone())),
        }
    }

    pub fn lazy<F, T: 'static>(&self, name: &'static str, f: F) -> Green<T>
    where
        T: Clone,
        F: Fn(&Self) -> T,
    {
        if let Some(x) = self.lazy_names.borrow().get(name) {
            return x.downcast_ref::<Green<T>>().unwrap().clone();
        }

        let cell: Arc<RefCell<T>> = Arc::new(RefCell::new(f(self)));
        let node = Green::<T>(cell, PhantomData);

        self.lazy_names
            .borrow_mut()
            .insert(name, Box::new(node.clone()));

        node
    }

    fn compute_named_children(tree: &Spanned<Tree>) -> HashMap<&'static str, Spanned<Child>> {
        let mut named_children = HashMap::new();

        for child in &tree.children {
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
