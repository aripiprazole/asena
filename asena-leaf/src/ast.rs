use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{ControlFlow, Deref, DerefMut, FromResidual, Try};
use std::sync::Arc;

use asena_span::Spanned;

use crate::node::Child;
use crate::token::TokenKind;

use super::node::Tree;
use super::token::Token;

pub type LeafKey = &'static str;

pub trait Leaf: Sized + Clone {
    fn make(from: Spanned<Tree>) -> Option<Self>;
}

pub trait Terminal: Sized {
    fn terminal(from: Spanned<Token>) -> Option<Self>;
}

pub trait Ast: Deref<Target = GreenTree> + DerefMut + Clone + Debug {}

pub struct Cursor<T> {
    value: Arc<RefCell<GreenTree>>,

    /// Children marked with name, to be accessed fast.
    children: HashMap<LeafKey, Spanned<Child>>,

    phantom: PhantomData<T>,
}

impl<T: Leaf> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            children: self.children.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T: Leaf> Cursor<T> {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn set(&self, value: Cursor<T>) {
        self.value.replace(value.value.borrow().clone());
    }

    pub fn new<I: Into<GreenTree>>(value: I) -> Self {
        Self {
            value: Arc::new(RefCell::new(value.into())),
            children: Default::default(),
            phantom: PhantomData,
        }
    }

    pub fn as_new_node(&self) -> Self {
        Self {
            value: Arc::new(RefCell::new(self.value.borrow().clone())),
            children: Default::default(),
            phantom: PhantomData,
        }
    }

    pub fn as_leaf(&self) -> Option<T>
    where
        T: Clone,
    {
        match self.value.borrow().clone() {
            GreenTree::Leaf { data, .. } => T::make(data),
            GreenTree::Error => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match &*self.value.borrow() {
            GreenTree::Leaf { .. } => true,
            GreenTree::Error => false,
        }
    }
}

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

    pub fn memoize<F, T: Leaf + Clone + 'static>(&self, name: &'static str, f: F) -> Cursor<T>
    where
        F: Fn(&Spanned<Tree>) -> Cursor<T>,
    {
        let Self::Leaf { data, names, .. } = self else {
            return Cursor::empty();
        };

        if let Some(x) = names.borrow().get(name) {
            return x.downcast_ref::<Cursor<T>>().unwrap().clone();
        }

        let cursor = f(data);
        names.borrow_mut().insert(name, Box::new(cursor.clone()));
        cursor
    }

    pub fn is_single(&self) -> bool {
        match self {
            GreenTree::Leaf { data, .. } => data.is_single(),
            GreenTree::Error => false,
        }
    }

    pub fn children(&mut self) -> Option<&mut Vec<Spanned<Child>>> {
        match self {
            GreenTree::Leaf { data, .. } => Some(&mut data.children),
            GreenTree::Error => None,
        }
    }

    pub fn filter<T: Leaf>(&self) -> Cursor<Vec<T>> {
        match self {
            GreenTree::Leaf { data, .. } => data.filter(),
            GreenTree::Error => Cursor::empty(),
        }
    }

    pub fn terminal<T: Terminal + Clone>(&self, nth: usize) -> Cursor<T> {
        match self {
            GreenTree::Leaf { data, .. } => data.terminal(nth),
            GreenTree::Error => Cursor::empty(),
        }
    }

    pub fn filter_terminal<T: Terminal + Leaf>(&self) -> Cursor<Vec<T>> {
        match self {
            GreenTree::Leaf { data, .. } => data.filter_terminal(),
            GreenTree::Error => Cursor::empty(),
        }
    }

    pub fn at<T: Leaf>(&self, nth: usize) -> Cursor<T> {
        match self {
            GreenTree::Leaf { data, .. } => data.at(nth),
            GreenTree::Error => Cursor::empty(),
        }
    }

    pub fn has(&self, name: LeafKey) -> bool {
        match self {
            GreenTree::Leaf { data, .. } => data.has(name),
            GreenTree::Error => false,
        }
    }

    pub fn named_at<A: Leaf>(&self, name: LeafKey) -> Cursor<A> {
        match self {
            GreenTree::Leaf { data, .. } => data.named_at(name),
            GreenTree::Error => Cursor::empty(),
        }
    }

    pub fn named_terminal<A: Terminal + Leaf>(&self, name: LeafKey) -> Cursor<A> {
        match self {
            GreenTree::Leaf { data, .. } => data.named_at(name),
            GreenTree::Error => Cursor::empty(),
        }
    }

    pub fn matches(&self, nth: usize, kind: TokenKind) -> bool {
        match self {
            GreenTree::Leaf { data, .. } => data.matches(nth, kind),
            GreenTree::Error => false,
        }
    }
}

impl<T: Leaf> Cursor<Vec<T>> {
    pub fn first(self) -> Cursor<T> {
        todo!()
    }
}

impl Tree {
    pub fn filter<T: Leaf>(&self) -> Cursor<Vec<T>> {
        todo!()
        // self.children
        //     .iter()
        //     .filter_map(|child| match child.value.clone() {
        //         Child::Tree(tree) => Some(T::make(child.replace(tree))),
        //         Child::Token(..) => None,
        //     })
        //     .filter(|node| !node.is_empty())
        //     .collect()
    }

    pub fn terminal<T: Terminal>(&self, nth: usize) -> Cursor<T> {
        todo!()
        // let Some(child) = self.children.get(nth) else {
        //     return Cursor::empty();
        // };

        // match &child.value {
        //     Child::Tree(..) => Cursor::empty(),
        //     Child::Token(token) => T::terminal(child.replace(token.clone())),
        // }
    }

    pub fn filter_terminal<T: Terminal>(&self) -> Cursor<Vec<T>> {
        todo!()
        // self.children
        //     .iter()
        //     .filter_map(|child| match child.value.clone() {
        //         Child::Tree(..) => None,
        //         Child::Token(token) => Some(T::terminal(child.replace(token))),
        //     })
        //     .filter(|node| !node.is_empty())
        //     .collect()
    }

    pub fn at<T: Leaf>(&self, nth: usize) -> Cursor<T> {
        todo!()
        // let Some(child) = self.children.get(nth) else {
        //     return Cursor::empty();
        // };

        // match &child.value {
        //     Child::Tree(tree) => T::make(child.replace(tree.clone())),
        //     Child::Token(..) => Cursor::empty(),
        // }
    }

    pub fn has(&self, name: LeafKey) -> bool {
        todo!()
        // matches!(self.children.get(name), Some(..))
    }

    pub fn named_at<A: Leaf>(&self, name: LeafKey) -> Cursor<A> {
        todo!()
        // let Some(child) = self.children.get(name) else {
        //     return Cursor::empty();
        // };

        // match &child.value {
        //     Child::Tree(tree) => T::make(child.replace(tree.clone())),
        //     Child::Token(..) => Cursor::empty(),
        // }
    }

    pub fn named_terminal<A: Terminal>(&self, name: LeafKey) -> Cursor<A> {
        todo!()
        // let Some(child) = self.children.get(name) else {
        //     return Cursor::empty();
        // };

        // match &child.value {
        //     Child::Tree(..) => Cursor::empty(),
        //     Child::Token(token) => T::terminal(child.replace(token.clone())),
        // }
    }
}

impl From<Spanned<Tree>> for GreenTree {
    fn from(value: Spanned<Tree>) -> Self {
        GreenTree::new(value)
    }
}

impl<T: Leaf> Display for Cursor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Leaf> Debug for Cursor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Leaf> From<GreenTree> for Cursor<T> {
    fn from(value: GreenTree) -> Self {
        Cursor::new(value)
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

impl<T: Terminal + Clone> Leaf for T {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        <T as Terminal>::terminal(from.clone().swap(from.single().clone()))
    }
}

impl<T: Leaf> Leaf for Vec<T> {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}

impl<T: Leaf> Leaf for Option<T> {
    fn make(from: Spanned<Tree>) -> Option<Self> {
        Some(T::make(from))
    }
}

impl<T: Leaf> Default for Cursor<T> {
    fn default() -> Self {
        Self {
            value: Arc::new(RefCell::new(Default::default())),
            children: Default::default(),
            phantom: PhantomData,
        }
    }
}

fn compute_named_children(tree: &GreenTree) -> HashMap<LeafKey, Spanned<Child>> {
    todo!()
    // let mut named_children = HashMap::new();

    // for child in &tree.children {
    //     match child.value() {
    //         Child::Tree(tree) => {
    //             if let Some(name) = tree.name {
    //                 named_children.insert(name, child.clone());
    //             }
    //         }
    //         Child::Token(token) => {
    //             if let Some(name) = token.name {
    //                 named_children.insert(name, child.clone());
    //             }
    //         }
    //     }
    // }

    // named_children
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
    type Output = Spanned<Tree>;

    type Residual = Option<std::convert::Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Self::new(GreenTree::new(output))
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match &*self.value.borrow() {
            GreenTree::Leaf { data, .. } => ControlFlow::Continue(data.clone()),
            GreenTree::Error => ControlFlow::Break(None),
        }
    }
}
