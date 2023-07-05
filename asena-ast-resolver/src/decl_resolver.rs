use crate::{scope_resolver::ScopeResolver, *};

pub struct AstResolver<'a> {
    pub db: Driver,
    pub curr_vf: Arc<VfsFile>,
    pub binding_groups: HashMap<FunctionId, Vec<Arc<Decl>>>,
    pub reporter: &'a mut Reporter,
}

impl<'a> AsenaVisitor<()> for AstResolver<'a> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.module_ref(value.to_fn_id().as_str());

        self.db.add_path_dep(self.curr_vf.clone(), module_ref);
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

        let mut resolver: &mut dyn AsenaListener<()> = &mut resolver;

        signature.return_type().listen(&mut resolver);
        signature.body().listen(&mut resolver);
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
            let mut resolver: &mut dyn AsenaListener<()> = &mut resolver;

            pat.listen(&mut resolver);
        }

        let mut resolver: &mut dyn AsenaListener<()> = &mut resolver;

        assign.body().listen(&mut resolver);
    }
}
