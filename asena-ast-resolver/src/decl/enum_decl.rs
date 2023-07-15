use crate::{scopes::*, *};
use asena_ast_db::def::DefWithId;
use asena_leaf::ast::Located;

use super::AstResolver;

impl AstResolver<'_> {
    pub fn resolve_enum_decl(&mut self, enum_decl: Enum) {
        self.enum_declarations
            .insert(enum_decl.name().to_fn_id(), enum_decl.clone());

        self.db
            .global_scope()
            .borrow_mut()
            .create_enum(self.db, &enum_decl, None);

        let mut resolver = ScopeResolver::new(enum_decl.name(), Level::Value, self);

        for (name, parameter) in Parameter::compute_parameters(enum_decl.parameters()) {
            let mut scope = resolver.local_scope.borrow_mut();
            let binding_id = parameter.find_name().as_new_leaf::<BindingId>();
            let def = DefWithId::new(
                resolver.owner.db,
                binding_id,
                parameter.location().into_owned(),
            );

            scope.types.insert(name.clone(), def);
        }

        resolver.listens(enum_decl.gadt_type());
        resolver.listens(enum_decl.variants());
        for method in enum_decl.methods() {
            self.resolve_method(method);
        }
    }
}
