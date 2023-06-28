use asena_derive::ast_leaf;
use asena_leaf::{ast::*, ast_virtual};

use crate::*;

pub trait Binary: Ast {
    #[ast_leaf]
    fn lhs(&self) -> Expr {
        self.at(0)
    }

    #[ast_leaf]
    fn fn_id(&self) -> Lexeme<FunctionId> {
        self.terminal(1)
    }

    /// FIXME: this stackoverflow
    #[ast_leaf]
    fn rhs(&self) -> Expr {
        Cursor::empty()
    }

    #[ast_leaf]
    fn operators(&self) -> Vec<Lexeme<FunctionId>> {
        self.filter_terminal().skip(1)
    }

    #[ast_leaf]
    fn operands(&self) -> Vec<Expr> {
        self.filter().skip(1)
    }
}

/// Binary operation represented by `fn_id`: `.`, and the two operands: `receiver`, `name`
impl Binary for Accessor {}

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
        Accessor,
        Ann,
        Qual,
    }
}
