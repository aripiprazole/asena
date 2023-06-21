use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use asena_span::Spanned;

use crate::node::Tree;

#[derive(Clone)]
pub enum GreenTree {
    Leaf { data: Arc<RwLock<Spanned<Tree>>> },
    Error,
}

pub trait Leaf: Deref<Target = GreenTree> + DerefMut + Clone + Debug {
    fn new<I: Into<GreenTree>>(value: I) -> Self;

    fn unwrap(self) -> GreenTree;
}
