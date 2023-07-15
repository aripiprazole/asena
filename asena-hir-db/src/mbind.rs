use asena_hir::value::HirValue;
use asena_leaf::ast::AstParam;

use crate::db::HirDatabase;

pub fn mbind(_db: &dyn HirDatabase, file: AstParam<HirValue>) -> HirValue {
    file.data
}
