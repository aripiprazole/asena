use std::cell::RefCell;

use asena_ast::AsenaFile;
use asena_leaf::ast::Node;

use crate::package::{Package, PackageData};
use crate::vfs::VfsFile;
use crate::*;

#[derive(Default)]
pub struct NonResolvingAstDatabase {
    internal_module_refs: RefCell<HashMap<String, ModuleRef>>,
    internal_decls: RefCell<HashMap<FunctionId, Arc<Decl>>>,
    internal_packages: RefCell<HashMap<Package, Arc<PackageData>>>,
    internal_vfs_files: RefCell<HashMap<VfsPath, Arc<VfsFile>>>,
}

impl crate::database::AstDatabase for NonResolvingAstDatabase {
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

    fn decl_of(&self, name: FunctionId, vfs_file: Arc<VfsFile>) -> Option<Arc<Decl>> {
        vfs_file
            .dependencies
            .read()
            .unwrap()
            .get(&name)
            .cloned()
            .or_else(|| self.internal_decls.borrow().get(&name).cloned())
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

    fn add_path_dep(&self, vfs_file: Arc<VfsFile>, module: ModuleRef) {
        let mut names = vfs_file.dependencies.write().unwrap();
        let file = self.vfs_file(module);
        for (fn_id, decl) in self.items(file).iter() {
            names.insert(fn_id.clone(), decl.clone());
        }
    }

    fn intern_vfs_file(&self, vfs_file: VfsFile) -> Arc<VfsFile> {
        let vfs_file = Arc::new(vfs_file);
        let name = FunctionId::new(&vfs_file.name);
        self.internal_module_refs
            .borrow_mut()
            .insert(vfs_file.name.clone(), ModuleRef::Found(vfs_file.id.clone()));
        self.internal_vfs_files
            .borrow_mut()
            .insert(vfs_file.id.clone(), vfs_file.clone());
        for (partial_name, decl) in self.items(vfs_file.clone()).iter() {
            let global_name = FunctionId::create_path(name.clone(), partial_name.clone());
            println!("Global name: {:?}", global_name.as_str());
            self.internal_decls
                .borrow_mut()
                .insert(global_name, decl.clone());
        }
        vfs_file
    }

    fn intern_package(&self, package: PackageData) -> Package {
        let package = Arc::new(package);
        let mut internal_packages = self.internal_packages.borrow_mut();
        let id = Package(internal_packages.len());
        internal_packages.insert(id, package);
        id
    }

    fn intern_resolved_name(&self, module: FunctionId, decl: Decl) -> Arc<Decl> {
        let mut internal_decls = self.internal_decls.borrow_mut();
        let decl = Arc::new(decl);
        internal_decls.insert(module, decl.clone());
        decl
    }
}
