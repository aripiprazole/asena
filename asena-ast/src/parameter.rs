use asena_derive::*;

use asena_leaf::ast::{GreenTree, Leaf, Lexeme, Node};
use asena_leaf::node::TreeKind::*;
use asena_leaf::token::TokenKind;

use crate::*;

#[derive(Default, Node, Clone)]
pub struct Parameter(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
impl Parameter {
    /// Optional parameter's name
    #[ast_leaf]
    pub fn name(&self) -> Lexeme<Local> {
        self.filter_terminal::<Local>().first()
    }

    /// Parameter's type
    #[ast_leaf]
    pub fn parameter_type(&self) -> Type {
        self.filter::<Type>().first()
    }

    /// If the parameter is explicit, or if it's a constraint or a type that can have the hole filled
    /// in the compile time, like a generic.
    pub fn explicit(&self) -> bool {
        self.matches(0, TokenKind::LeftParen)
    }
}

impl Leaf for Parameter {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            Param => Parameter::new(tree),
            _ => return None,
        })
    }
}
