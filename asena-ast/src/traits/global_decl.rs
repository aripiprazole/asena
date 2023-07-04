use asena_leaf::ast::Ast;

use crate::*;

pub trait GlobalDecl: Ast {
    fn name(&self) -> Option<BindingId> {
        let node = self.as_node();
        let named = Decl::downcast_has_name(&node)?;

        Some(named.name())
    }

    #[ast_leaf]
    fn parameters(&self) -> Vec<Parameter> {
        self.filter()
    }
}

impl GlobalDecl for Class {}
impl GlobalDecl for Enum {}
impl GlobalDecl for Signature {}
impl GlobalDecl for Trait {}
