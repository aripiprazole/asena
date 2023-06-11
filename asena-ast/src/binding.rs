use asena_derive::Leaf;
use asena_leaf::green::GreenTree;
use asena_leaf::spec::Node;

use asena_span::Spanned;

use crate::*;

#[derive(Leaf, Clone)]
pub struct Binding(GreenTree);

impl Binding {
    pub fn name(&self) -> Node<Local> {
        todo!()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

pub type BindingRef = Spanned<Binding>;
