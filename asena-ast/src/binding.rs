use asena_derive::{ast_debug, ast_leaf, Leaf};

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
