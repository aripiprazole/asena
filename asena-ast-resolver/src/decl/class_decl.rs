use crate::{scopes::*, *};
use asena_ast_db::def::DefWithId;
use asena_leaf::ast::Located;

use super::AstResolver;

impl AstResolver<'_, '_> {
    pub fn resolve_class_decl(&mut self, class: Class) {
        self.class_declarations
            .insert(class.name().to_fn_id(), class.clone());

        self.db
            .global_scope()
            .borrow_mut()
            .create_class(self.db, &class, None);

        let mut resolver = ScopeResolver::new(class.name(), Level::Value, self);

        for (name, parameter) in Parameter::compute_parameters(class.parameters()) {
            let binding_id = parameter.find_name().as_new_leaf::<BindingId>();
            let def = DefWithId::new(
                resolver.owner.db,
                binding_id,
                parameter.location().into_owned(),
            );

            let mut scope = resolver.local_scope.borrow_mut();

            scope.types.insert(name.clone(), def);
        }

        resolver.listens(class.fields());
        for method in class.methods() {
            self.resolve_method(method);
        }
    }
}
