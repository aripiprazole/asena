use asena_ast_db::def::DefWithId;
use asena_leaf::ast::Located;

use crate::{scopes::*, *};

use super::AstResolver;

impl AstResolver<'_> {
    pub fn resolve_instance_decl(&mut self, instance_decl: Instance) {
        self.instance_declarations.push(instance_decl.clone());

        let resolver = ScopeResolver::empty(Level::Value, self);

        for (name, parameter) in Parameter::compute_parameters(instance_decl.parameters()) {
            let mut scope = resolver.local_scope.borrow_mut();
            let binding_id = parameter.find_name().as_new_leaf::<BindingId>();
            let def = DefWithId::new(
                resolver.owner.db,
                binding_id,
                parameter.location().into_owned(),
            );

            scope.types.insert(name.clone(), def);
        }

        for method in instance_decl.methods() {
            self.resolve_method(method);
        }
    }
}
