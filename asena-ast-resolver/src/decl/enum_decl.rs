use asena_ast_db::scope::TypeValue;

use crate::{scopes::*, *};

use super::AstResolver;

impl AstResolver<'_> {
    pub fn resolve_enum_decl(&mut self, enum_decl: Enum) {
        self.enum_declarations
            .insert(enum_decl.name().to_fn_id(), enum_decl.clone());

        self.db
            .global_scope()
            .borrow_mut()
            .create_enum(&enum_decl, None);

        let mut resolver = ScopeResolver::new(enum_decl.name(), Level::Value, self);

        for name in Parameter::compute_parameters(enum_decl.parameters()).keys() {
            let mut scope = resolver.local_scope.borrow_mut();
            scope.types.insert(name.clone(), TypeValue::Synthetic);
        }

        resolver.listens(enum_decl.gadt_type());
        resolver.listens(enum_decl.variants());
        for method in enum_decl.methods() {
            self.resolve_method(method);
        }
    }
}
