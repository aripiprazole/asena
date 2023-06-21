use asena_derive::{ast_debug, ast_leaf, Leaf};

use asena_leaf::ast::{Cursor, GreenTree};
use asena_leaf::token::TokenKind;

use crate::*;

#[derive(Leaf, Clone)]
pub struct Parameter(GreenTree);

#[ast_debug]
impl Parameter {
    /// Optional parameter's name
    #[ast_leaf]
    pub fn name(&self) -> Cursor<Local> {
        self.filter_terminal::<Local>().first()
    }

    /// Parameter's type
    #[ast_leaf]
    pub fn parameter_type(&self) -> Cursor<Type> {
        self.filter::<Type>().first()
    }

    /// If the parameter is explicit, or if it's a constraint or a type that can have the hole filled
    /// in the compile time, like a generic.
    pub fn explicit(&self) -> bool {
        self.matches(0, TokenKind::LeftParen)
    }
}
