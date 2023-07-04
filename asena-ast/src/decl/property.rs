use asena_leaf::ast::{Leaf, Node};
use asena_leaf::kind::TreeKind::*;

use crate::*;

/// A field node is a record node's field.
///
/// The syntax is like:
/// ```haskell
/// name : String;
/// ```
///
/// The constraint node should be wrote in a class context.
#[derive(Default, Node, Located, Clone)]
pub struct Field(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Field {
    #[ast_leaf]
    pub fn name(&self) -> BindingId {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn field_type(&self) -> Typed {
        self.filter().first()
    }
}

impl Leaf for Field {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            ClassField => Field::new(tree),
            _ => return None,
        })
    }
}

/// A method node is a record function associated to a record, this can be used in implementation
/// declarations too.
///
/// The syntax is like:
/// ```haskell
/// fun sayHello(self): IO () {
//    printf "Hello, I'm {}" self.name
//  }
/// ```
///
/// The method node is a simple sugar for declaring it on the top level with the class name concatenated,
/// like: `sayHello`, in the `Person` class, should be simply `Person.sayHello`.
#[derive(Default, Node, Located, Clone)]
pub struct Method(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Method {
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

impl Leaf for Method {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            ClassMethod => Method::new(tree),
            _ => return None,
        })
    }
}
