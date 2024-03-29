use asena_derive::*;

use asena_leaf::ast::*;
use asena_leaf::kind::TreeKind::*;

use crate::*;

/// A lambda expression parameter, is a parameter of a lambda expression.
#[derive(Default, Node, Located, Clone, Hash, PartialEq, Eq)]
pub struct LamParameter(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl LamParameter {
    #[ast_leaf]
    pub fn name(&self) -> Lexeme<Local> {
        self.filter_terminal().first()
    }
}

impl Leaf for LamParameter {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            LamParam => LamParameter::new(tree),
            _ => return None,
        })
    }
}
