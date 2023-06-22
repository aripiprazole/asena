use asena_derive::{ast_debug, ast_leaf, Leaf};
use asena_leaf::{
    ast::{Leaf, Walkable},
    node::Tree,
};
use asena_span::Spanned;

use crate::*;

#[derive(Default, Leaf, Clone)]
pub struct Binding(GreenTree);

#[ast_of]
#[ast_debug]
impl Binding {
    #[ast_leaf]
    pub fn name(&self) -> Local {
        todo!()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        todo!()
    }
}

impl Leaf for Binding {
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}

impl<W: ExprWalker + StmtWalker + PatWalker> Walkable<W> for Binding {
    fn walk(&self, walker: &W) {
        self.value().walk(walker)
    }
}
