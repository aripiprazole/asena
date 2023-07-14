use asena_ast_db::scope::TypeValue;

use crate::{scopes::*, *};

use super::AstResolver;

impl AstResolver<'_, '_> {
    pub fn resolve_class_decl(&mut self, class: Class) {
        self.class_declarations
            .insert(class.name().to_fn_id(), class.clone());

        self.db
            .global_scope()
            .borrow_mut()
            .create_class(&class, None);

        let mut resolver = ScopeResolver::new(class.name(), Level::Value, self);

        for name in Parameter::compute_parameters(class.parameters()).keys() {
            let mut scope = resolver.local_scope.borrow_mut();
            scope.types.insert(name.clone(), TypeValue::Synthetic);
        }

        resolver.listens(class.fields());
        for method in class.methods() {
            self.resolve_method(method);
        }
    }
}
