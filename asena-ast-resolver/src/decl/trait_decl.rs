use asena_ast_db::scope::TypeValue;

use crate::{scopes::*, *};

use super::AstResolver;

impl AstResolver<'_> {
    pub fn resolve_trait_decl(&mut self, trait_decl: Trait) {
        self.trait_declarations
            .insert(trait_decl.name().to_fn_id(), trait_decl.clone());

        self.db
            .global_scope()
            .borrow_mut()
            .create_trait(&trait_decl, None);

        let mut resolver = ScopeResolver::new(trait_decl.name(), Level::Value, self);

        for name in Parameter::compute_parameters(trait_decl.parameters()).keys() {
            let mut scope = resolver.local_scope.borrow_mut();
            scope.types.insert(name.clone(), TypeValue::Synthetic);
        }

        resolver.listens(trait_decl.fields());
        for method in trait_decl.default_methods() {
            self.resolve_default_method(method);
        }
    }
}
