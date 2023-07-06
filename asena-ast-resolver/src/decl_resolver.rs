use asena_ast_db::scope::TypeValue;

use crate::{scope_resolver::*, *};

#[non_exhaustive]
pub struct AstResolver<'a> {
    pub db: Driver,
    pub file: Arc<VfsFile>,
    pub binding_groups: im::HashMap<FunctionId, Vec<Arc<Decl>>>,
    pub enum_declarations: im::HashMap<FunctionId, Enum>,
    pub class_declarations: im::HashMap<FunctionId, Class>,
    pub trait_declarations: im::HashMap<FunctionId, Trait>,
    pub instance_declarations: Vec<Instance>, // TODO: change to hashset
    pub reporter: &'a mut Reporter,
}

impl AstResolver<'_> {
    pub fn resolve_method(&mut self, method: Method) {
        let mut resolver = ScopeResolver::new(method.name(), Level::Value, self);

        for (name, parameter) in Parameter::compute_parameters(method.parameters()) {
            let mut scope = resolver.local_scope.borrow_mut();
            let value = Arc::new(parameter);
            scope.functions.insert(name, Value::Param(value));
        }

        resolver.listens(method.return_type());
        resolver.listens(method.body());
    }

    pub fn resolve_default_method(&mut self, method: DefaultMethod) {
        let mut resolver = ScopeResolver::new(method.name(), Level::Value, self);

        for (name, parameter) in Parameter::compute_parameters(method.parameters()) {
            let mut scope = resolver.local_scope.borrow_mut();
            let value = Arc::new(parameter);
            scope.functions.insert(name, Value::Param(value));
        }

        resolver.listens(method.return_type());
        resolver.listens(method.body());
    }
}

impl<'a> AsenaVisitor<()> for AstResolver<'a> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.module_ref(value.to_fn_id().as_str());

        self.db.add_path_dep(self.file.clone(), module_ref);
    }

    fn visit_enum(&mut self, enum_decl: Enum) {
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

    fn visit_class(&mut self, class: Class) {
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

    fn visit_trait(&mut self, trait_decl: Trait) {
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

    fn visit_instance(&mut self, instance: Instance) {
        self.instance_declarations.push(instance.clone());

        let resolver = ScopeResolver::empty(Level::Value, self);

        for name in Parameter::compute_parameters(instance.parameters()).keys() {
            let mut scope = resolver.local_scope.borrow_mut();
            scope.types.insert(name.clone(), TypeValue::Synthetic);
        }

        for method in instance.methods() {
            self.resolve_method(method);
        }
    }

    fn visit_signature(&mut self, signature: Signature) {
        // associate the type declaration with the implementations.
        self.binding_groups
            .entry(signature.name().to_fn_id())
            .or_insert(Default::default())
            .push(Arc::new(signature.clone().into()));

        let name = signature.name();
        let mut resolver = ScopeResolver::new(name, Level::Value, self);

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
        let mut resolver = ScopeResolver::new(name, Level::Value, self);

        for pat in assign.patterns() {
            resolver.listens(pat);
        }

        resolver.listens(assign.body());
    }
}
