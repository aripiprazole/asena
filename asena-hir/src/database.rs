use asena_hir_leaf::HirBaseDatabase;

use crate::*;

pub trait HirBag: HirInterner {
    fn expr_data(&self, id: expr::HirExprId) -> expr::HirExpr;
    fn value_data(&self, id: value::HirValueId) -> value::HirValue;
    fn pattern_data(&self, id: pattern::HirPatternId) -> pattern::HirPattern;
    fn stmt_data(&self, id: stmt::HirStmtId) -> stmt::HirStmt;
    fn top_level_data(&self, id: top_level::HirTopLevelId) -> top_level::HirTopLevel;
    fn attr_data(&self, id: attr::HirAttrId) -> attr::HirAttr;
    fn type_data(&self, id: hir_type::HirTypeId) -> hir_type::HirType;
}

pub trait HirInterner: HirBaseDatabase {
    fn intern_expr(&self, data: expr::HirExpr) -> expr::HirExprId;
    fn intern_value(&self, data: value::HirValue) -> value::HirValueId;
    fn intern_pattern(&self, data: pattern::HirPattern) -> pattern::HirPatternId;
    fn intern_stmt(&self, data: stmt::HirStmt) -> stmt::HirStmtId;
    fn intern_top_level(&self, data: top_level::HirTopLevel) -> top_level::HirTopLevelId;
    fn intern_attr(&self, data: attr::HirAttr) -> attr::HirAttrId;
    fn intern_type(&self, data: hir_type::HirType) -> hir_type::HirTypeId;
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
#[doc(hidden)]
fn _assert_dyn() {
    fn assert_dyn<T: ?Sized>() {}

    assert_dyn::<dyn HirBag>();
    assert_dyn::<dyn HirInterner>();
}
