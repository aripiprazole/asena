use crate::{decl_resolver::AstResolver, *};

pub struct ScopeResolver<'gctx, 'a> {
    pub db: Driver,
    pub local_scope: Rc<RefCell<ScopeData>>,
    pub frames: Vec<Rc<RefCell<ScopeData>>>,
    pub resolver: &'gctx mut AstResolver<'a>,
}

impl<'gctx, 'a> ScopeResolver<'gctx, 'a> {
    pub fn new(name: BindingId, resolver: &'gctx mut AstResolver<'a>) -> Self {
        let global_scope = resolver.db.global_scope();
        let local_scope = {
            let named_scope = global_scope.borrow().fork();
            let mut scope_mut = named_scope.borrow_mut();
            scope_mut.variables.insert(name.to_fn_id(), 0);
            named_scope.clone()
        };

        Self {
            db: resolver.db.clone(),
            local_scope: local_scope.clone(),
            frames: vec![local_scope],
            resolver,
        }
    }

    pub fn last_scope(&mut self) -> Rc<RefCell<ScopeData>> {
        self.frames
            .last()
            .cloned()
            .unwrap_or_else(|| self.db.global_scope())
    }
}

impl AsenaListener for ScopeResolver<'_, '_> {
    // >>> Enter/Exit scope abstractions
    fn enter_pi(&mut self, pi: asena_ast::Pi) {
        let scope = self.last_scope().borrow().fork();
        if let Some(name) = pi.parameter_name() {
            let name = name.to_fn_id();
            let value = pi.parameter_type();
            let mut scope = scope.borrow_mut();
            scope.functions.insert(name, Value::Expr(Arc::new(value)));
        }

        self.frames.push(scope);
    }

    fn exit_pi(&mut self, _: asena_ast::Pi) {
        self.frames.pop();
    }

    fn enter_case(&mut self, _: Case) {
        let scope = self.last_scope().borrow().fork();
        self.frames.push(scope);
    }

    fn exit_case(&mut self, _: Case) {
        self.frames.pop();
    }

    fn enter_lam(&mut self, _: Lam) {
        let scope = self.last_scope().borrow().fork();
        self.frames.push(scope);
    }

    fn exit_lam(&mut self, _: Lam) {
        self.frames.pop();
    }
    // <<< Enter/Exit scope abstractions

    /// Just bridges to ast resolver, which will search, and report if it's bound.
    fn enter_qualified_path(&mut self, value: asena_ast::QualifiedPath) {
        self.resolver.visit_qualified_path(value);
    }

    fn enter_lam_parameter(&mut self, value: LamParameter) {
        let scope = self.last_scope();
        let mut scope = scope.borrow_mut();

        let value = Arc::new(value);
        scope
            .functions
            .insert(value.name().to_fn_id(), Value::LamParam(value));
    }

    fn enter_local_expr(&mut self, value: LocalExpr) {
        let scope = self.last_scope();
        let scope = scope.borrow();
        match scope.functions.get(&value.to_fn_id()) {
            Some(_) => {}
            None => {
                self.resolver
                    .reporter
                    .report(&value.segments(), UnresolvedNameError(value.to_fn_id()));
            }
        }
    }

    fn enter_global_pat(&mut self, value: asena_ast::GlobalPat) {
        let name = value.name();
        let file = self.resolver.file.clone();

        match self.db.constructor_data(value.name(), file) {
            VariantResolution::Variant(_) => {}
            VariantResolution::Binding(name) => {
                let scope = self.last_scope();
                let mut scope = scope.borrow_mut();
                let name = name.to_fn_id();
                let value = Arc::new(value.into());
                scope.functions.insert(name, Value::Pat(value));
            }
            VariantResolution::None => {
                let fn_id = name.to_fn_id();
                self.resolver
                    .reporter
                    .report(&name, UnresolvedNameError(fn_id));
            }
        }
    }

    fn enter_constructor_pat(&mut self, value: asena_ast::ConstructorPat) {
        let name = value.name();
        let file = self.resolver.file.clone();

        match self.db.constructor_data(value.name(), file) {
            VariantResolution::Binding(_) if !value.arguments().is_empty() => {
                let fn_id = name.to_fn_id();
                self.resolver
                    .reporter
                    .report(&name, UnresolvedNameError(fn_id));
            }
            VariantResolution::Variant(_) | VariantResolution::Binding(_) => {}
            VariantResolution::None => {
                let fn_id = name.to_fn_id();
                self.resolver
                    .reporter
                    .report(&name, UnresolvedNameError(fn_id));
            }
        }
    }
}
