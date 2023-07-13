use asena_derive::*;

use asena_leaf::ast::*;
use asena_leaf::kind::TreeKind::*;

use crate::*;

/// A case is the representation of the pattern matching case and it's value.
///
/// # Examples
///
/// ```asena
/// Just x -> println x
/// ```
#[derive(Default, Node, Located, Clone, Hash, PartialEq, Eq)]
pub struct Case(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Case {
    #[ast_leaf]
    pub fn pat(&self) -> Pat {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Branch {
        self.filter().first()
    }
}

impl Leaf for Case {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            MatchCase => Case::new(tree),
            _ => return None,
        })
    }
}
