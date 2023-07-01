use asena_leaf::ast::{Leaf, Node};
use asena_leaf::node::TreeKind::*;

use crate::*;

/// A constraint is a part of the abstract syntax tree, that represents an unnamed implicit [Parameter].
///
/// The syntax is like:
/// ```haskell
/// class Monad m : Functor m { ... }
/// ```
///
/// The constraint node can be used on `where` clauses.
#[derive(Default, Node, Located, Clone)]
pub struct Constraint(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, BodyWalker, ExprWalker, PatWalker, StmtWalker)]
impl Constraint {
    #[ast_leaf]
    pub fn value(&self) -> Typed {
        self.filter().first()
    }
}

impl Leaf for Constraint {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            TypeConstraint => Constraint::new(tree),
            _ => return None,
        })
    }
}
