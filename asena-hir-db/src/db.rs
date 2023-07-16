use asena_ast_db::{db::AstDatabase, package::Package, vfs::VfsFile};
use asena_ast_lowering::db::AstLowerrer;
use asena_ast_resolver::db::AstResolverDatabase;
use asena_hir::{
    interner::HirInterner,
    top_level::{HirBindingGroup, HirTopLevel, HirTopLevelKind},
    value::HirValue,
};
use asena_leaf::ast::AstParam;
use asena_prec::PrecDatabase;
use if_chain::if_chain;
use im::HashSet;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase:
    PrecDatabase + HirInterner + AstDatabase + AstLowerrer + AstResolverDatabase
{
    #[salsa::invoke(crate::loceval::loceval)]
    fn hir_loceval(&self, file: VfsFile) -> VfsFile;

    #[salsa::invoke(crate::mbind::mbind)]
    fn hir_mbind(&self, file: AstParam<HirValue>) -> HirValue;

    #[salsa::invoke(crate::rc::rc)]
    fn hir_rc(&self, declaration: AstParam<HirBindingGroup>) -> HirBindingGroup;

    fn hir_file_defs(&self, file: VfsFile) -> HashSet<HirTopLevel>;

    fn hir_defs(&self, pkg: Package) -> HashSet<HirTopLevel>;

    fn hir_find_fn(&self, pkg: Package, name: String) -> Option<HirBindingGroup>;
}

fn hir_find_fn(db: &dyn HirDatabase, pkg: Package, name: String) -> Option<HirBindingGroup> {
    let defs = db.hir_defs(pkg);
    defs.into_iter().find_map(|def| {
        if_chain! {
            let data = db.lookup_intern_top_level(def);
            if let HirTopLevelKind::BindingGroup(group) = data.kind;
            let local_name = db.lookup_intern_name(group.signature.name);
            if local_name == name;
            then {
                return Some(group);
            }
        }

        None
    })
}

fn hir_file_defs(db: &dyn HirDatabase, file: VfsFile) -> HashSet<HirTopLevel> {
    let ast = db.ast(file);
    let ast = db.infix_commands(ast.into());
    let ast = db.ordered_prec(ast.into());
    let ast = db.ast_resolved_file(ast.into());
    let hir = db.hir_file(ast.into());

    hir.declarations.into_iter().collect()
}

fn hir_defs(db: &dyn HirDatabase, pkg: Package) -> HashSet<HirTopLevel> {
    let mut defs = HashSet::default();

    for file in pkg.files(db).iter() {
        let file = db.hir_loceval(*file);
        let groups = db.hir_file_defs(file);

        defs.extend(groups);
    }

    defs
}
