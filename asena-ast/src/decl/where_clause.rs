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
#[derive(Default, Node, Located, Clone)]
pub struct Where(GreenTree);

#[ast_of]
#[ast_debug]
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

impl<W: BodyWalker + BranchWalker + ExprWalker + PatWalker + StmtWalker> Walkable<W> for Where {
    fn walk(&self, walker: &mut W) {
        self.constraints().walk(walker);
    }
}
