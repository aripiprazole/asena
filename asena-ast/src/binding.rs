use asena_derive::{node_leaf, Leaf};
use asena_leaf::ast::Cursor;

use crate::*;

#[derive(Leaf, Clone)]
pub struct Binding(GreenTree);

impl Binding {
    #[node_leaf]
    pub fn name(&self) -> Cursor<Local> {
        todo!()
    }

    #[node_leaf]
    pub fn value(&self) -> Cursor<Expr> {
        todo!()
    }
}

pub type BindingRef = Binding;
