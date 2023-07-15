use asena_ast_db::def::DefWithId;
use asena_leaf::ast::Located;

use crate::{scopes::*, *};

use super::AstResolver;

impl AstResolver<'_> {
    pub fn resolve_trait_decl(&mut self, trait_decl: Trait) {
        self.trait_declarations
            .insert(trait_decl.name().to_fn_id(), trait_decl.clone());

        self.db
            .global_scope()
            .write()
            .unwrap()
            .create_trait(self.db, &trait_decl, None);

        let mut resolver = ScopeResolver::new(trait_decl.name(), Level::Value, self);

        for (name, parameter) in Parameter::compute_parameters(trait_decl.parameters()) {
            let mut scope = resolver.local_scope.write().unwrap();
            let binding_id = parameter.name();
            let def = DefWithId::new(
                resolver.owner.db,
                binding_id,
                parameter.location().into_owned(),
            );

            scope.types.insert(name.clone(), def);
        }

        resolver.listens(trait_decl.fields());
        for method in trait_decl.default_methods() {
            self.resolve_default_method(method);
        }
    }
}
