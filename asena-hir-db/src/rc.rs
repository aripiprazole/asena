use asena_hir::top_level::HirBindingGroup;
use asena_leaf::ast::AstParam;

use crate::db::HirDatabase;

pub fn rc(_db: &dyn HirDatabase, declaration: AstParam<HirBindingGroup>) -> HirBindingGroup {
    declaration.data
}
