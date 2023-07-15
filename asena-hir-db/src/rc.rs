use asena_ast_db::vfs::VfsFile;

use crate::db::HirDatabase;

pub fn rc(db: &dyn HirDatabase, file: VfsFile) -> VfsFile {
    let ast = db.ast(file);
    let _hir = db.hir_file(ast);

    file
}
