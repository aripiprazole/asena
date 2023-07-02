use asena_leaf::ast::{Leaf, Node};
use asena_leaf::kind::TreeKind::*;

use crate::*;

/// A default method node in a trait, declares a default behavior for a field in a trait.
///
/// The syntax is like:
/// ```haskell
/// default sayHello(self): IO () {
//    printf "Hello, I'm {}" self.name
//  }
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct DefaultMethod(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(FileWalker)]
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
