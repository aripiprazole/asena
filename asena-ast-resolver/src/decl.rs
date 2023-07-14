use asena_ast_db::db::AstDatabase;

use crate::{scopes::*, *};

mod class_decl;
mod enum_decl;
mod instance_decl;
mod trait_decl;

pub struct AstResolver<'db, 'a> {
    pub db: &'db dyn AstDatabase,
    pub file: Arc<VfsFile>,
    pub reporter: &'a mut Reporter,
    pub binding_groups: im::HashMap<FunctionId, Vec<Arc<Decl>>>,
    pub enum_declarations: im::HashMap<FunctionId, Enum>,
    pub class_declarations: im::HashMap<FunctionId, Class>,
    pub trait_declarations: im::HashMap<FunctionId, Trait>,
    pub instance_declarations: Vec<Instance>, // TODO: change to hashset
}

impl<'db, 'a> AstResolver<'db, 'a> {
    pub fn new(db: &'db dyn AstDatabase, file: Arc<VfsFile>, reporter: &'a mut Reporter) -> Self {
        AstResolver {
            db,
            file,
            reporter,
            binding_groups: Default::default(),
            enum_declarations: Default::default(),
            class_declarations: Default::default(),
            trait_declarations: Default::default(),
            instance_declarations: Default::default(),
        }
    }

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

impl<'ctx, 'a> AsenaVisitor<()> for AstResolver<'ctx, 'a> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.module_ref(value.to_fn_id().to_string());

        self.db.add_path_dep(self.file.clone(), module_ref);
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

    fn visit_enum(&mut self, enum_decl: Enum) {
        self.resolve_enum_decl(enum_decl);
    }

    fn visit_class(&mut self, class: Class) {
        self.resolve_class_decl(class);
    }

    fn visit_trait(&mut self, trait_decl: Trait) {
        self.resolve_trait_decl(trait_decl);
    }

    fn visit_instance(&mut self, instance: Instance) {
        self.resolve_instance_decl(instance);
    }
}
