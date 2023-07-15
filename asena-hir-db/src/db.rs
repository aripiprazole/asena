use asena_ast_db::{db::AstDatabase, vfs::VfsFile};
use asena_ast_lowering::db::AstLowerrer;
use asena_hir::interner::HirInterner;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: HirInterner + AstDatabase + AstLowerrer {
    #[salsa::invoke(crate::loceval::loceval)]
    fn hir_loceval(&self, file: VfsFile) -> VfsFile;

    #[salsa::invoke(crate::mbind::mbind)]
    fn hir_mbind(&self, file: VfsFile) -> VfsFile;

    #[salsa::invoke(crate::rc::rc)]
    fn hir_rc(&self, file: VfsFile) -> VfsFile;
}
