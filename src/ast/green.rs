use crate::lexer::span::Spanned;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use super::node::Tree;

pub type GreenChild = Green<Box<dyn Any>>;

pub struct GreenTree {
    /// TODO: Use tree cursor instead of using directly the [Spanned] tree's [Tree], to invalidate
    /// `lazy_names` references.
    tree: Spanned<Tree>,

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
    pub fn new(tree: Spanned<Tree>) -> GreenTree {
        Self {
            tree,
            lazy_names: Default::default(),
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
            tree: self.tree.clone(),
            lazy_names: Default::default(),
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
