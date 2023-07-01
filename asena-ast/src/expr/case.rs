use asena_derive::*;

use asena_leaf::ast::*;
use asena_leaf::kind::TreeKind::*;

use crate::*;

/// Match case
#[derive(Default, Node, Located, Clone)]
pub struct Case(GreenTree);

#[ast_of]
#[ast_debug]
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

impl<W: BranchWalker + PatWalker + StmtWalker + ExprWalker> Walkable<W> for Case {
    fn walk(&self, walker: &mut W) {
        self.pat().walk(walker);
        self.value().walk(walker);
    }
}
