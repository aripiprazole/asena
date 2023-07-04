use asena_derive::*;

use asena_leaf::ast::{GreenTree, Leaf, Lexeme, Node};
use asena_leaf::node::TreeKind::*;
use asena_leaf::token::kind::TokenKind;

use crate::*;

/// A function parameter, or a generic parameter. It can be explicit, or implicit, like a generic,
/// or either a `self` parameter.
///
/// # Examples
///
/// ```asena
/// foo (a : Int) (b : Int) : Int
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Parameter(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Parameter {
    /// Optional parameter's name
    #[ast_leaf]
    pub fn name(&self) -> Lexeme<Local> {
        self.filter_terminal::<Local>().first()
    }

    /// Parameter's type
    #[ast_leaf]
    pub fn parameter_type(&self) -> Typed {
        self.filter::<Typed>().first()
    }

    /// If the parameter is explicit, or if it's a constraint or a type that can have the hole filled
    /// in the compile time, like a generic.
    pub fn explicit(&self) -> bool {
        self.matches(0, TokenKind::LeftParen)
    }

    /// If the parameter is the `self` parameter.
    pub fn is_self(&self) -> bool {
        !self.token(TokenKind::SelfKeyword).is_error()
    }
}

impl Parameter {
    /// Walks the tree using the given visitor, it will call the visitor's methods for each node
    /// in the tree.
    pub fn walks<T: AsenaVisitor<()>>(self, mut visitor: T) -> Self {
        self.walk(&mut visitor::new_walker(&mut visitor));
        self
    }
}

impl Leaf for Parameter {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            Param => Parameter::new(tree),
            SelfParam => Parameter::new(tree),
            _ => return None,
        })
    }
}
