use crate::{scope_resolver::ScopeResolver, *};

#[non_exhaustive]
pub struct AstResolver<'a> {
    pub db: Driver,
    pub file: Arc<VfsFile>,
    pub binding_groups: HashMap<FunctionId, Vec<Arc<Decl>>>,
    pub enum_declarations: HashMap<FunctionId, Enum>,
    pub reporter: &'a mut Reporter,
}

impl<'a> AsenaVisitor<()> for AstResolver<'a> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.module_ref(value.to_fn_id().as_str());

        self.db.add_path_dep(self.file.clone(), module_ref);
    }

    fn visit_enum(&mut self, enum_decl: Enum) {
        self.enum_declarations
            .insert(enum_decl.name().to_fn_id(), enum_decl.clone());

        let mut resolver = ScopeResolver::new(enum_decl.name(), self);

        for (name, parameter) in Parameter::compute_parameters(enum_decl.parameters()) {
            let mut scope = resolver.local_scope.borrow_mut();
            let value = Arc::new(parameter);
            scope.functions.insert(name, Value::Param(value));
        }

        resolver.listens(enum_decl.gadt_type());

        for variant in enum_decl.variants() {
            resolver.listens(variant);
        }
    }

    fn visit_signature(&mut self, signature: Signature) {
        // associate the type declaration with the implementations.
        self.binding_groups
            .entry(signature.name().to_fn_id())
            .or_insert(Default::default())
            .push(Arc::new(signature.clone().into()));

        let name = signature.name();
        let mut resolver = ScopeResolver::new(name, self);

        for (name, parameter) in Parameter::compute_parameters(signature.parameters()) {
            let mut scope = resolver.local_scope.borrow_mut();
            let value = Arc::new(parameter);
            scope.functions.insert(name, Value::Param(value));
        }

        resolver.listens(signature.return_type());
        resolver.listens(signature.body());
    }

    fn visit_assign(&mut self, assign: Assign) {
        // associate the type declaration with the implementations.
        self.binding_groups
            .entry(assign.name().to_fn_id())
            .or_insert(Default::default())
            .push(Arc::new(assign.clone().into()));

        let name = assign.name();
        let mut resolver = ScopeResolver::new(name, self);

        for pat in assign.patterns() {
            resolver.listens(pat);
        }

        resolver.listens(assign.body());
    }
}
