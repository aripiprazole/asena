use asena_derive::ast_leaf;
use asena_leaf::ast::Ast;

use crate::*;

pub trait Binary: Ast {
    #[ast_leaf]
    fn lhs(&self) -> Expr {
        self.at(0)
    }

    #[ast_leaf]
    fn fn_id(&self) -> FunctionId {
        self.terminal(1)
    }

    #[ast_leaf]
    fn rhs(&self) -> Expr {
        let mut rhs = self.clone();
        let Some(children) = rhs.children() else {
            return Cursor::empty();
        };

        // Checks the integrity of the length for safety
        match children.len() {
            0 => return Cursor::empty(),
            1 => return rhs.at(0),
            _ => {}
        }

        // Remove the first twice
        children.remove(0);
        children.remove(0);

        if rhs.is_single() {
            rhs.at(0)
        } else {
            Cursor::new(rhs.deref().clone())
        }
    }
}

/// Binary operation represented by `fn_id`: `.`, and the two operands: `receiver`, `name`
impl Binary for Accessor {}

impl Binary for Infix {}

impl Binary for Ann {}

impl Binary for Qual {}
