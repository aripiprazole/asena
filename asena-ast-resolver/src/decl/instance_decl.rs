use asena_ast_db::scope::TypeValue;

use crate::{scopes::*, *};

use super::AstResolver;

impl AstResolver<'_, '_> {
    pub fn resolve_instance_decl(&mut self, instance_decl: Instance) {
        self.instance_declarations.push(instance_decl.clone());

        let resolver = ScopeResolver::empty(Level::Value, self);

        for name in Parameter::compute_parameters(instance_decl.parameters()).keys() {
            let mut scope = resolver.local_scope.borrow_mut();
            scope.types.insert(name.clone(), TypeValue::Synthetic);
        }

        for method in instance_decl.methods() {
            self.resolve_method(method);
        }
    }
}
