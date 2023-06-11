use asena_derive::Leaf;

use asena_leaf::green::GreenTree;
use asena_leaf::spec::Node;

use asena_span::Spanned;

use crate::*;

#[derive(Leaf, Clone)]
pub struct Parameter(GreenTree);

impl Parameter {
    /// Optional parameter's name
    pub fn name(&self) -> Option<Node<Spanned<Local>>> {
        self.filter_terminal::<Local>().first().cloned()
    }

    /// Parameter's type
    pub fn parameter_type(&self) -> Node<Spanned<Type>> {
        self.filter::<Type>().first().cloned().into()
    }

    /// If the parameter is explicit, or if it's a constraint or a type that can have the hole filled
    /// in the compile time, like a generic.
    pub fn explicit(&self) -> bool {
        self.matches(0, TokenKind::LeftParen)
    }
}
