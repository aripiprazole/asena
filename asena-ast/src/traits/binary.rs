use asena_derive::ast_leaf;
use asena_leaf::{ast::*, ast_virtual};

use crate::*;

pub trait Binary: Ast {
    #[ast_leaf]
    fn lhs(&self) -> Cursor<Expr> {
        self.at(0)
    }

    #[ast_leaf]
    fn fn_id(&self) -> Cursor<Lexeme<FunctionId>> {
        self.terminal(1)
    }

    #[ast_leaf]
    fn rhs(&self) -> Cursor<Expr> {
        let mut rhs = self.clone();
        let Some(children) = rhs.children() else {
            return Cursor::empty();
        };

        // Checks the integrity of the length for safety
        match children.len() {
            0 => return Cursor::empty(),
            1 => return Cursor::empty(),
            _ => {}
        }

        // Remove the first twice
        children.remove(0);
        children.remove(0);

        if rhs.is_single() {
            rhs.at(0)
        } else {
            Cursor::new(rhs.as_new_node())
        }
    }
}

/// Binary operation represented by `fn_id`: `.`, and the two operands: `receiver`, `name`

impl Binary for Infix {}

impl Binary for Ann {}

impl Binary for Qual {}

impl Binary for VirtualBinary {}

impl Expr {
    pub fn as_binary(&self) -> Option<VirtualBinary> {
        self.clone().into_virtual()
    }
}

ast_virtual! {
    /// Virtual node for binary operations and tree manipulations in the source code, so it should not
    /// be used in the AST. It is used in the `asena-prec` crate.
    ///
    /// # Examples
    /// It should be used to abstract the binary operations in the AST, so that the `asena-prec` crate
    /// can manipulate the AST without changing the AST itself.
    pub enum VirtualBinary : Expr {
        #[node]
        Infix,
        Ann,
        Qual,
    }
}
