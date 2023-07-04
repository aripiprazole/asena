use std::cell::RefCell;
use std::rc::Rc;

use asena_ast::{BindingId, Variant};

use crate::package::{Package, PackageData};
use crate::scope::{ScopeData, Value, VariantResolution};
use crate::vfs::VfsFile;
use crate::*;

pub trait AstDatabase {
    fn global_scope(&self) -> Rc<RefCell<ScopeData>>;
    fn module_ref(&self, path: &str) -> ModuleRef;
    fn package_data(&self, file: Package) -> Interned<PackageData>;
    fn vfs_file(&self, module: ModuleRef) -> Arc<VfsFile>;
    fn package_of(&self, module: ModuleRef) -> Arc<PackageData>;
    fn items(&self, module: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Decl>>>;
    fn constructors(&self, module: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Variant>>>;
    fn abstract_syntax_tree(&self, vfs_file: Arc<VfsFile>) -> Arc<asena_ast::AsenaFile>;
    fn add_path_dep(&self, vfs_file: Arc<VfsFile>, module: ModuleRef);
    fn function_data(&self, name: FunctionId, vfs_file: Arc<VfsFile>) -> Option<Value>;
    fn constructor_data(&self, name: BindingId, vfs_file: Arc<VfsFile>) -> VariantResolution;

    fn intern_package(&self, package: PackageData) -> Package;
    fn intern_vfs_file(&self, vfs_file: VfsFile) -> Arc<VfsFile>;
    fn intern_resolved_name(&self, module: FunctionId, decl: Decl) -> Arc<Decl>;
}
