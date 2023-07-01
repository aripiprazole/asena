use asena_leaf::ast::{Leaf, Node};
use asena_leaf::kind::TreeKind::*;

use crate::*;

/// A method node is a record function associated to a record, this can be used in implementation
/// declarations too.
///
/// The syntax is like:
/// ```haskell
/// sayHello(self): IO () {
//    printf "Hello, I'm {}" self.name
//  }
/// ```
///
/// The method node is a simple sugar for declaring it on the top level with the class name concatenated,
/// like: `sayHello`, in the `Person` class, should be simply `Person.sayHello`.
#[derive(Default, Node, Located, Clone)]
pub struct DefaultMethod(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, BodyWalker, ExprWalker, PatWalker, StmtWalker)]
impl DefaultMethod {
    #[ast_leaf]
    pub fn name(&self) -> QualifiedPath {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        self.filter()
    }

    #[ast_leaf]
    pub fn where_clause(&self) -> Option<Where> {
        self.filter().try_as_nth(0)
    }

    #[ast_leaf]
    pub fn return_type(&self) -> Typed {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn body(&self) -> Vec<Stmt> {
        self.filter()
    }
}

impl Leaf for DefaultMethod {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            TraitDefault => DefaultMethod::new(tree),
            _ => return None,
        })
    }
}
