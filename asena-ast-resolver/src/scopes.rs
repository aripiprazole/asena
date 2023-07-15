use crate::{decl::AstResolver, *};
use asena_ast_db::{def::Def, package::HasDiagnostic, scope::ScopeRef};
use asena_leaf::ast::Located;
use asena_report::WithError;

pub enum Level {
    Type,
    Value,
}

pub struct ScopeResolver<'db, 'ctx> {
    pub local_scope: ScopeRef,
    pub frames: Vec<ScopeRef>,
    pub level: Level,
    pub owner: &'ctx mut AstResolver<'db>,
}

impl<'db, 'ctx> ScopeResolver<'db, 'ctx> {
    pub fn new(name: BindingId, level: Level, resolver: &'ctx mut AstResolver<'db>) -> Self {
        let global_scope = resolver.db.global_scope();
        let local_scope = {
            let named_scope = global_scope.read().unwrap().fork();
            let mut scope = named_scope.write().unwrap();
            scope.variables.insert(name.to_fn_id(), 0);
            named_scope.clone()
        };

        Self {
            local_scope: local_scope.clone(),
            frames: vec![local_scope],
            level,
            owner: resolver,
        }
    }

    pub fn empty(level: Level, resolver: &'ctx mut AstResolver<'db>) -> Self {
        let global_scope = resolver.db.global_scope();
        let local_scope = global_scope.read().unwrap().fork();

        Self {
            local_scope: local_scope.clone(),
            frames: vec![local_scope],
            level,
            owner: resolver,
        }
    }

    pub fn last_scope(&mut self) -> ScopeRef {
        self.frames
            .last()
            .cloned()
            .unwrap_or_else(|| self.owner.db.global_scope())
    }
}

impl AsenaListener for ScopeResolver<'_, '_> {
    // >>> Enter/Exit scope abstractions
    fn enter_pi(&mut self, pi: asena_ast::Pi) {
        let scope = self.last_scope().read().unwrap().fork();
        if let Some(name) = pi.parameter_name() {
            let value = pi.parameter_type();
            let local = name.to_fn_id();
            let def = DefWithId::new(self.owner.db, name, value.location().into_owned());
            let mut scope = scope.write().unwrap();

            scope.functions.insert(local, def);
        }

        self.frames.push(scope);
    }

    fn exit_pi(&mut self, _: asena_ast::Pi) {
        self.frames.pop();
    }

    fn enter_case(&mut self, _: Case) {
        let scope = self.last_scope().read().unwrap().fork();
        self.frames.push(scope);
    }

    fn exit_case(&mut self, _: Case) {
        self.frames.pop();
    }

    fn enter_lam(&mut self, _: Lam) {
        let scope = self.last_scope().read().unwrap().fork();
        self.frames.push(scope);
    }

    fn exit_lam(&mut self, _: Lam) {
        self.frames.pop();
    }

    fn enter_typed_explicit(&mut self, _: Expr) {
        self.level = Level::Type;
    }

    fn exit_typed_explicit(&mut self, _: Expr) {
        self.level = Level::Value;
    }
    // <<< Enter/Exit scope abstractions

    /// Just bridges to ast resolver, which will search, and report if it's bound.
    fn enter_qualified_path(&mut self, value: asena_ast::QualifiedPath) {
        self.owner.visit_qualified_path(value);
    }

    fn enter_lam_parameter(&mut self, value: LamParameter) {
        let scope = self.last_scope();
        let mut scope = scope.write().unwrap();

        let def = DefWithId::new(self.owner.db, value.name(), value.location().into_owned());

        scope.functions.insert(value.name().to_fn_id(), def);
    }

    fn enter_local_expr(&mut self, value: LocalExpr) {
        let scope = self.last_scope();
        let scope = scope.read().unwrap();
        match self.level {
            Level::Type => match scope.find_type(&value) {
                Def::WithId(id) => {
                    let resolution = TypeResolution::Resolved(id);

                    value.dynamic(TypeResolutionKey, resolution);
                }
                Def::Unresolved => {
                    value
                        .segments()
                        .fail(UnresolvedTypeNameError(value.to_fn_id()))
                        .push(self.owner.db);
                }
            },
            Level::Value => match scope.functions.get(&value.to_fn_id()).cloned() {
                Some(resolved) => {
                    value.dynamic(ExprResolutionKey, ExprResolution::Resolved(resolved));
                }
                None => {
                    value
                        .segments()
                        .fail(UnresolvedNameError(value.to_fn_id()))
                        .push(self.owner.db);
                }
            },
        }
    }

    fn enter_global_pat(&mut self, value: asena_ast::GlobalPat) {
        let name = value.name();
        let file = self.owner.file;

        match self.owner.db.constructor_data(value.name(), file) {
            VariantResolution::Variant(variant) => {
                value.dynamic(PatResolutionKey, PatResolution::Variant(variant));
            }
            VariantResolution::Binding(name) => {
                value.dynamic(PatResolutionKey, PatResolution::LocalBinding(name.clone()));

                let scope = self.last_scope();
                let mut scope = scope.write().unwrap();

                let location = name.location().into_owned();
                let local = name.to_fn_id();
                let def = DefWithId::new(self.owner.db, *name, location);

                scope.functions.insert(local, def);
            }
            VariantResolution::None => {
                let fn_id = name.to_fn_id();

                name.fail(UnresolvedNameError(fn_id)).push(self.owner.db);
            }
        }
    }

    fn enter_constructor_pat(&mut self, value: asena_ast::ConstructorPat) {
        let name = value.name();
        let file = self.owner.file;

        match self.owner.db.constructor_data(value.name(), file) {
            VariantResolution::Binding(name) => {
                name.clone()
                    .fail(UnresolvedNameError(name.to_fn_id()))
                    .push(self.owner.db);

                value.dynamic(PatResolutionKey, PatResolution::LocalBinding(name));
            }
            VariantResolution::Variant(variant) => {
                value.dynamic(PatResolutionKey, PatResolution::Variant(variant));
            }
            VariantResolution::None => {
                name.clone()
                    .fail(UnresolvedNameError(name.to_fn_id()))
                    .push(self.owner.db);
            }
        }
    }
}
