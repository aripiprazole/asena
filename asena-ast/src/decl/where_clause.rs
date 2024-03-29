use asena_derive::*;

use asena_leaf::ast::*;
use asena_leaf::kind::TreeKind::*;

use crate::*;

/// A where clause is a part of the abstract syntax tree, that represents a list of [Constraint]s.
///
/// # Examples
///
/// ```asena
/// where Monad m
/// ```
#[derive(Default, Node, Located, Clone, Hash, PartialEq, Eq)]
pub struct Where(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Where {
    #[ast_leaf]
    pub fn constraints(&self) -> Vec<Constraint> {
        self.filter()
    }
}

impl Leaf for Where {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            WhereClause => Where::new(tree),
            _ => return None,
        })
    }
}
