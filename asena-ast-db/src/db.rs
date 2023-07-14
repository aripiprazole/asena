use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use asena_ast::{AsenaFile, BindingId, GlobalName, QualifiedPath, Variant};
use asena_leaf::ast::Node;
use asena_lexer::Lexer;
use asena_parser::Parser;

use crate::package::{Package, PackageData};
use crate::scope::{ScopeData, TypeValue, Value, VariantResolution};
use crate::vfs::VfsFile;
use crate::*;

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase {
    #[salsa::memoized]
    fn module_ref(&self, path: String) -> ModuleRef;

    #[salsa::memoized]
    fn package_data(&self, file: Package) -> Interned<PackageData>;

    #[salsa::memoized]
    fn vfs_file(&self, module: ModuleRef) -> Arc<VfsFile>;

    #[salsa::memoized]
    fn package_of(&self, module: ModuleRef) -> Arc<PackageData>;

    #[salsa::memoized]
    fn items(&self, module: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Decl>>>;

    #[salsa::memoized]
    fn constructors(&self, module: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Variant>>>;

    #[salsa::memoized]
    fn abstract_syntax_tree(&self, vfs_file: Arc<VfsFile>) -> Arc<asena_ast::AsenaFile>;

    #[salsa::memoized]
    fn function_data(&self, name: QualifiedPath, vfs_file: Arc<VfsFile>) -> Value;

    #[salsa::memoized]
    fn constructor_data(&self, name: BindingId, vfs_file: Arc<VfsFile>) -> VariantResolution;

    #[salsa::memoized]
    fn add_path_dep(&self, vfs_file: Arc<VfsFile>, module: ModuleRef) -> ();

    #[salsa::input]
    fn global_scope(&self) -> Rc<RefCell<ScopeData>>;

    #[salsa::input]
    fn intern_package(&self, package: PackageData) -> Package;

    #[salsa::input]
    fn intern_vfs_file(&self, vfs_file: VfsFile) -> Arc<VfsFile>;

    #[salsa::input]
    fn intern_resolved_name(&self, module: FunctionId, decl: Decl) -> Arc<Decl>;
}

fn package_of(db: &dyn AstDatabase, _module: ModuleRef) -> Arc<PackageData> {
    todo!()
}

fn package_data(db: &dyn AstDatabase, package_file: Package) -> Interned<PackageData> {
    todo!()
    // db.internal_packages
    //     .borrow()
    //     .get(&package_file)
    //     .cloned()
    //     .into()
}

fn constructor_data(db: &dyn AstDatabase, name: BindingId, f: Arc<VfsFile>) -> VariantResolution {
    f.scope
        .read()
        .unwrap()
        .find_type_constructor(&name)
        .or_else(|| db.global_scope().borrow().find_type_constructor(&name))
}

fn function_data(db: &dyn AstDatabase, name: QualifiedPath, vfs_file: Arc<VfsFile>) -> Value {
    vfs_file
        .read_scope()
        .find_value(&name)
        .or_else(|| db.global_scope().borrow().find_value(&name))
}

fn module_ref(db: &dyn AstDatabase, path: String) -> ModuleRef {
    todo!()
    // db.internal_module_refs
    //     .borrow()
    //     .get(path)
    //     .cloned()
    //     .unwrap_or(ModuleRef::NotFound)
}

fn vfs_file(db: &dyn AstDatabase, path: ModuleRef) -> Arc<VfsFile> {
    match path {
        ModuleRef::NotFound => todo!("Handle unresolved declarations"),
        _ => todo!()
        // ModuleRef::Found(reference_id) => db
        //     .internal_vfs_files
        //     .borrow()
        //     .get(&reference_id)
        //     .unwrap_or_else(|| panic!("Internal error: VFS file not found: {:?}", reference_id))
        //     .clone(),
    }
}

fn items(db: &dyn AstDatabase, vfs_file: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Decl>>> {
    let ast = db.abstract_syntax_tree(vfs_file);
    let mut decls = HashMap::new();
    for decl in ast.declarations() {
        if let Some(named_decl) = Decl::downcast_has_name(&decl) {
            decls.insert(named_decl.name().to_fn_id(), Arc::new(decl));
        }
    }
    Arc::new(decls)
}

fn abstract_syntax_tree(db: &dyn AstDatabase, vfs_file: Arc<VfsFile>) -> Arc<asena_ast::AsenaFile> {
    let file = vfs_file
        .vfs()
        .read_file(&vfs_file.id.path)
        .expect("Internal error: VFS file not found");

    let lexer = Lexer::new(PathBuf::from(vfs_file.id.path.clone()), &file);
    let parser = Parser::from(lexer).run(asena_grammar::file);
    let tree = parser.build_tree();

    Arc::new(AsenaFile::new(tree.data))
}

fn constructors(db: &dyn AstDatabase, f: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Variant>>> {
    let ast = db.abstract_syntax_tree(f);
    let mut variants = HashMap::new();
    for decl in ast.declarations() {
        let Decl::Enum(enum_decl) = decl else {
                continue;
            };

        variants.extend(enum_decl.constructors());
    }
    Arc::new(variants)
}

fn add_path_dep(db: &dyn AstDatabase, vfs_file: Arc<VfsFile>, module: ModuleRef) {
    let mut scope_data = vfs_file.scope.write().unwrap();
    let from_file = db.vfs_file(module);
    scope_data.import(db, from_file, None);
}

fn intern_vfs_file(db: &dyn AstDatabase, vfs_file: VfsFile) -> Arc<VfsFile> {
    todo!()
    // let mut global_scope = db.scope.borrow_mut();

    // let vf = Arc::new(vfs_file);
    // let name = FunctionId::new(&vf.name);

    // db.internal_module_refs
    //     .borrow_mut()
    //     .insert(vf.name.clone(), ModuleRef::Found(vf.id.clone()));

    // db.internal_vfs_files
    //     .borrow_mut()
    //     .insert(vf.id.clone(), vf.clone());

    // global_scope.import(db, vf.clone(), Some(name));
    // vf
}

fn intern_package(db: &dyn AstDatabase, package: PackageData) -> Package {
    todo!()
    // let package = Arc::new(package);
    // let mut internal_packages = db.internal_packages.borrow_mut();
    // let id = Package(internal_packages.len());
    // internal_packages.insert(id, package);
    // id
}

fn intern_resolved_name(db: &dyn AstDatabase, module: FunctionId, decl: Decl) -> Arc<Decl> {
    todo!()
    // let mut global_scope = db.global_scope().borrow_mut();
    // let decl = Arc::new(decl);

    // global_scope
    //     .types
    //     .insert(module, TypeValue::Decl(decl.clone()));

    // decl
}
