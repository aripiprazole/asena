use asena_derive::{ast_debug, ast_leaf, Leaf};
use asena_leaf::ast::Cursor;

use crate::*;

#[derive(Leaf, Clone)]
pub struct Binding(GreenTree);

#[ast_debug]
impl Binding {
    #[ast_leaf]
    pub fn name(&self) -> Cursor<Local> {
        todo!()
    }

    #[ast_leaf]
    pub fn value(&self) -> Cursor<Expr> {
        todo!()
    }
}
