use crate::*;

pub trait AstDatabase {
    fn module_ref(&self, path: &str) -> ModuleRef;
    fn vfs_file(&self, module: ModuleRef) -> Arc<VfsFile>;
    fn package_of(&self, module: ModuleRef) -> Arc<PackageData>;
    fn items(&self, module: Arc<VfsFile>) -> Arc<HashMap<FunctionId, Arc<Decl>>>;
    fn abstract_syntax_tree(&self, vfs_file: Arc<VfsFile>) -> Arc<asena_ast::AsenaFile>;
    fn add_path_dep(&self, vfs_file: Arc<VfsFile>, module: ModuleRef);
    fn decl_of(&self, name: FunctionId, vfs_file: Arc<VfsFile>) -> Option<Arc<Decl>>;

    fn intern_package(&self, package: PackageData) -> PackageId;
    fn intern_vfs_file(&self, vfs_file: VfsFile) -> Arc<VfsFile>;
    fn intern_resolved_name(&self, module: FunctionId, decl: Decl) -> Arc<Decl>;
}
