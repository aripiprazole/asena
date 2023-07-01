use asena_derive::*;

use asena_leaf::ast::*;
use asena_leaf::kind::TreeKind::*;

use crate::*;

#[derive(Default, Node, Located, Clone)]
pub struct Where(GreenTree);

#[ast_of]
#[ast_debug]
impl Where {
    #[ast_leaf]
    pub fn value(&self) -> Vec<Constraint> {
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

impl<W: BodyWalker + BranchWalker + ExprWalker + PatWalker + StmtWalker> Walkable<W> for Where {
    fn walk(&self, walker: &mut W) {
        self.value().walk(walker);
    }
}
