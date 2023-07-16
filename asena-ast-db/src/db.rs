use std::path::PathBuf;

use asena_ast::{AsenaFile, BindingId, GlobalName, QualifiedPath, Variant};
use asena_leaf::ast::{AstParam, GreenTree, Located, Node};
use asena_lexer::Lexer;
use asena_parser::Parser;
use asena_report::Diagnostic;
use asena_span::{Loc, Spanned};

use crate::build_system::BuildSystem;
use crate::def::{Def, DefData, DefWithId};
use crate::package::{HasDiagnostic, Package, PackageData};
use crate::scope::{ScopeRef, VariantResolution};
use crate::vfs::{VfsFile, VfsFileData};
use crate::*;

type Constructors = HashMap<FunctionId, Arc<Variant>>;

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase {
    fn build_system(&self) -> Arc<BuildSystem>;

    fn path_module(&self, path: PathBuf) -> ModuleRef;
    fn items(&self, module: VfsFile) -> Arc<HashMap<FunctionId, Arc<Decl>>>;
    fn constructors(&self, module: VfsFile) -> Arc<HashMap<FunctionId, Arc<Variant>>>;
    fn source(&self, module: VfsFile) -> Arc<String>;
    fn ast(&self, vfs_file: VfsFile) -> asena_ast::AsenaFile;
    fn cst(&self, vfs_file: VfsFile) -> GreenTree;
    fn package_of(&self, module: Loc) -> Package;
    fn vfs_file(&self, module: ModuleRef) -> VfsFile;

    fn module_ref(&self, module: Spanned<FunctionId>) -> ModuleRef;
    fn function_data(&self, name: QualifiedPath, vfs_file: VfsFile) -> Def;
    fn constructor_data(&self, name: BindingId, vfs_file: VfsFile) -> VariantResolution;
    fn location_file(&self, loc: Loc) -> ModuleRef;

    fn add_path_dep(&self, vfs_file: VfsFile, module: ModuleRef) -> ();
    fn mk_global_name(&self, module: FunctionId, decl: AstParam<Decl>) -> DefWithId;
    fn mk_vfs_file(&self, vfs_file: VfsFileData) -> VfsFile;

    fn global_scope(&self) -> ScopeRef;

    #[salsa::interned]
    fn intern_package(&self, package: PackageData) -> Package;

    #[salsa::interned]
    fn intern_vfs_file(&self, vfs_file: VfsFileData) -> VfsFile;

    #[salsa::interned]
    fn intern_def(&self, data: DefData) -> DefWithId;
}

fn path_module(db: &dyn AstDatabase, path: PathBuf) -> ModuleRef {
    let global_scope = db.global_scope();
    let global_scope = global_scope.read().unwrap();

    global_scope.paths.get(&path).cloned().unwrap_or_else(|| {
        Diagnostic::of(Loc::new(path.clone(), 0, 0), FileNotFoundError(path)).push(db);

        ModuleRef::NotFound
    })
}

fn global_scope(_: &dyn AstDatabase) -> ScopeRef {
    ScopeRef::default()
}

fn build_system(_: &dyn AstDatabase) -> Arc<BuildSystem> {
    Arc::new(BuildSystem::default())
}

fn package_of(db: &dyn AstDatabase, loc: Loc) -> Package {
    db.build_system()
        .file_package(&loc.file.clone().unwrap_or_default())
        .unwrap_or_else(|| panic!("Internal error: package not found: {loc:?}"))
    // TODO: handle
}

fn location_file(db: &dyn AstDatabase, loc: Loc) -> ModuleRef {
    let path = loc.file.clone().unwrap_or_default();

    let global_scope = db.global_scope();
    let global_scope = global_scope.read().unwrap();

    global_scope.paths.get(&path).cloned().unwrap_or_else(|| {
        Diagnostic::of(loc, FileNotFoundError(path)).push(db);

        ModuleRef::NotFound
    })
}

fn constructor_data(db: &dyn AstDatabase, name: BindingId, file: VfsFile) -> VariantResolution {
    db.lookup_intern_vfs_file(file)
        .read_scope()
        .find_type_constructor(&name)
        .or_else(|| {
            db.global_scope()
                .read()
                .unwrap()
                .find_type_constructor(&name)
        })
}

fn function_data(db: &dyn AstDatabase, name: QualifiedPath, file: VfsFile) -> Def {
    db.lookup_intern_vfs_file(file)
        .read_scope()
        .find_value(&name)
        .or_else(|| db.global_scope().read().unwrap().find_value(&name))
}

fn vfs_file(_db: &dyn AstDatabase, path: ModuleRef) -> VfsFile {
    match path {
        ModuleRef::NotFound => todo!("Not handling unresolved declarations: {path:?}"),
        ModuleRef::Found(path) => path,
    }
}

fn items(db: &dyn AstDatabase, vfs_file: VfsFile) -> Arc<HashMap<FunctionId, Arc<Decl>>> {
    let ast = db.ast(vfs_file);
    let mut decls = HashMap::new();
    for decl in ast.declarations() {
        if let Some(named_decl) = Decl::downcast_has_name(&decl) {
            decls.insert(named_decl.name().to_fn_id(), Arc::new(decl));
        }
    }
    Arc::new(decls)
}

fn source(db: &dyn AstDatabase, vfs_file: VfsFile) -> Arc<String> {
    let vfs_file = db.lookup_intern_vfs_file(vfs_file);

    let file = vfs_file
        .vfs()
        .read_file(&vfs_file.name)
        .expect("Internal error: VFS file not found");

    Arc::new(file)
}

fn cst(db: &dyn AstDatabase, vfs_file: VfsFile) -> GreenTree {
    let source = db.source(vfs_file);
    let data = db.lookup_intern_vfs_file(vfs_file);

    let lexer = Lexer::new(data.id.path, &source);
    let parser = Parser::from(lexer).run(asena_grammar::file);
    let tree = parser.build_tree();

    tree.data.into()
}

fn ast(db: &dyn AstDatabase, vfs_file: VfsFile) -> asena_ast::AsenaFile {
    let tree = db.cst(vfs_file);

    AsenaFile::new(tree)
}

fn constructors(db: &dyn AstDatabase, f: VfsFile) -> Arc<Constructors> {
    let mut variants = HashMap::new();

    let ast = db.ast(f);
    for decl in ast.declarations() {
        let Decl::Enum(enum_decl) = decl else {
            continue;
        };

        variants.extend(enum_decl.constructors());
    }

    Arc::new(variants)
}

fn add_path_dep(db: &dyn AstDatabase, vfs_file: VfsFile, module: ModuleRef) {
    let data = db.lookup_intern_vfs_file(vfs_file);
    let mut scope_data = data.scope.write().unwrap();
    scope_data.import(db, db.vfs_file(module), None);
}

fn mk_global_name(db: &dyn AstDatabase, module: FunctionId, decl: AstParam<Decl>) -> DefWithId {
    let global_scope = db.global_scope();
    let mut global_scope = global_scope.write().unwrap();

    let name = decl.name().unwrap(); // TODO: handle
    let def_with_id = DefWithId::new(db, name, decl.location().into_owned());

    global_scope.types.insert(module, def_with_id);

    def_with_id
}

fn mk_vfs_file(db: &dyn AstDatabase, vfs_file: VfsFileData) -> VfsFile {
    let scope = db.global_scope();
    let mut scope = scope.write().unwrap();

    let path = vfs_file.id.path.clone();
    let name = FunctionId::new(&vfs_file.name);
    let pkg = vfs_file.pkg;
    let id = db.intern_vfs_file(vfs_file);
    let module = ModuleRef::Found(id);

    db.build_system().add_file(path.clone(), module.clone());
    db.build_system().add_module(module.clone(), pkg);

    scope.modules.insert(name.to_string(), module.clone());
    scope.paths.insert(path, module);
    scope.import(db, id, Some(name));

    let pkg_data = db.lookup_intern_package(pkg);
    pkg_data.files.insert(id);

    id
}

fn module_ref(db: &dyn AstDatabase, module: Spanned<FunctionId>) -> ModuleRef {
    let global_scope = db.global_scope();
    let global_scope = global_scope.read().unwrap();

    global_scope
        .modules
        .get(&module.to_string())
        .cloned()
        .map(|module| {
            let pkg = db.build_system().module_package(&module).unwrap();

            db.build_system().add_module(module, pkg)
        })
        .unwrap_or_else(|| {
            Diagnostic::of(module.span, ModuleNotFoundError(module.value)).push(db);

            ModuleRef::NotFound
        })
}
