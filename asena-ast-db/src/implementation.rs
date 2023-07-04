use std::cell::RefCell;
use std::rc::Rc;

use asena_ast::{AsenaFile, BindingId, GlobalName, QualifiedPath, Variant};
use asena_leaf::ast::Node;

use crate::package::{Package, PackageData};
use crate::scope::{ScopeData, Value, VariantResolution};
use crate::vfs::VfsFile;
use crate::*;

#[derive(Default)]
pub struct NonResolvingAstDatabase {
    scope: Rc<RefCell<ScopeData>>,
    internal_module_refs: RefCell<HashMap<String, ModuleRef>>,
    internal_packages: RefCell<HashMap<Package, Arc<PackageData>>>,
    internal_vfs_files: RefCell<HashMap<VfsPath, Arc<VfsFile>>>,
}

impl crate::database::AstDatabase for NonResolvingAstDatabase {
    fn global_scope(&self) -> Rc<RefCell<ScopeData>> {
        self.scope.clone()
    }

    fn package_of(&self, _module: ModuleRef) -> Arc<PackageData> {
        todo!()
    }

    fn package_data(&self, package_file: Package) -> Interned<PackageData> {
        self.internal_packages
            .borrow()
            .get(&package_file)
            .cloned()
            .into()
    }

    fn constructor_data(&self, name: BindingId, vfs_file: Arc<VfsFile>) -> VariantResolution {
        vfs_file
            .scope
            .read()
            .unwrap()
            .find_type_constructor(&name)
            .or_else(|| self.scope.borrow().find_type_constructor(&name))
    }

    fn function_data(&self, name: QualifiedPath, vfs_file: Arc<VfsFile>) -> Value {
        vfs_file
            .read_scope()
            .find_value(&name)
            .or_else(|| self.scope.borrow().find_value(&name))
    }

    fn module_ref(&self, path: &str) -> ModuleRef {
        self.internal_module_refs
            .borrow()
            .get(path)
            .cloned()
            .unwrap_or(ModuleRef::NotFound)
    }

    fn vfs_file(&self, path: ModuleRef) -> Arc<VfsFile> {
        match path {
            ModuleRef::NotFound => todo!("Handle unresolved declarations"),
            ModuleRef::Found(reference_id) => self
                .internal_vfs_files
                .borrow()
                .get(&reference_id)
                .unwrap_or_else(|| panic!("Internal error: VFS file not found: {:?}", reference_id))
                .clone(),
        }
    }

    fn items(&self, vfs_file: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Decl>>> {
        let ast = self.abstract_syntax_tree(vfs_file);
        let mut decls = HashMap::new();
        for decl in ast.declarations() {
            if let Some(named_decl) = Decl::downcast_has_name(&decl) {
                decls.insert(named_decl.name().to_fn_id(), Arc::new(decl));
            }
        }
        Arc::new(decls)
    }

    fn abstract_syntax_tree(&self, vfs_file: Arc<VfsFile>) -> Arc<asena_ast::AsenaFile> {
        let file = vfs_file
            .vfs()
            .read_file(&vfs_file.id.path)
            .expect("Internal error: VFS file not found");

        let lexer = asena_lexer::Lexer::new(&file);
        let parser = asena_parser::Parser::from(lexer).run(asena_grammar::file);
        let tree = parser.build_tree();
        Arc::new(AsenaFile::new(tree.data))
    }

    fn constructors(&self, vfs_file: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Variant>>> {
        let ast = self.abstract_syntax_tree(vfs_file);
        let mut variants = HashMap::new();
        for decl in ast.declarations() {
            let Decl::Enum(enum_decl) = decl else {
                continue;
            };

            variants.extend(enum_decl.constructors());
        }
        Arc::new(variants)
    }

    fn add_path_dep(&self, vfs_file: Arc<VfsFile>, module: ModuleRef) {
        let mut scope_data = vfs_file.scope.write().unwrap();
        let from_file = self.vfs_file(module);
        scope_data.import(self, from_file, None);
    }

    fn intern_vfs_file(&self, vfs_file: VfsFile) -> Arc<VfsFile> {
        let mut global_scope = self.scope.borrow_mut();

        let vf = Arc::new(vfs_file);
        let name = FunctionId::new(&vf.name);

        self.internal_module_refs
            .borrow_mut()
            .insert(vf.name.clone(), ModuleRef::Found(vf.id.clone()));

        self.internal_vfs_files
            .borrow_mut()
            .insert(vf.id.clone(), vf.clone());

        global_scope.import(self, vf.clone(), Some(name));
        vf
    }

    fn intern_package(&self, package: PackageData) -> Package {
        let package = Arc::new(package);
        let mut internal_packages = self.internal_packages.borrow_mut();
        let id = Package(internal_packages.len());
        internal_packages.insert(id, package);
        id
    }

    fn intern_resolved_name(&self, module: FunctionId, decl: Decl) -> Arc<Decl> {
        let mut global_scope = self.scope.borrow_mut();
        let decl = Arc::new(decl);
        global_scope.declarations.insert(module, decl.clone());
        decl
    }
}
