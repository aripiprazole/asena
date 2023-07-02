use asena_derive::*;

use asena_leaf::ast::*;
use asena_leaf::kind::TreeKind::*;

use crate::*;

#[derive(Default, Node, Located, Clone)]
pub struct AccessorSegment(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl AccessorSegment {
    #[ast_leaf]
    pub fn name(&self) -> Lexeme<Local> {
        self.filter_terminal().first()
    }

    #[ast_leaf]
    pub fn arguments(&self) -> Vec<Expr> {
        self.filter()
    }
}

impl Leaf for AccessorSegment {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            AccessorArg => AccessorSegment::new(tree),
            _ => return None,
        })
    }
}