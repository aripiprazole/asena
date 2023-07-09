use std::sync::Arc;

use crate::query::leaf::HirBaseDatabase;

use crate::*;

pub trait HirBag: HirInterner {
    fn name_data(self: Arc<Self>, id: NameId) -> Arc<String>;
    fn expr_data(self: Arc<Self>, id: expr::HirExprId) -> Arc<expr::HirExpr>;
    fn value_data(self: Arc<Self>, id: value::HirValueId) -> Arc<value::HirValue>;
    fn pattern_data(self: Arc<Self>, id: pattern::HirPatternId) -> Arc<pattern::HirPattern>;
    fn stmt_data(self: Arc<Self>, id: stmt::HirStmtId) -> Arc<stmt::HirStmt>;
    fn top_level_data(self: Arc<Self>, id: top_level::HirTopLevelId)
        -> Arc<top_level::HirTopLevel>;
    fn attr_data(self: Arc<Self>, id: attr::HirAttrId) -> Arc<attr::HirAttr>;
    fn type_data(self: Arc<Self>, id: hir_type::HirTypeId) -> Arc<hir_type::HirType>;
}

pub trait HirInterner: HirBaseDatabase {
    fn intern_name(self: Arc<Self>, data: String) -> NameId;
    fn intern_expr(self: Arc<Self>, data: expr::HirExpr) -> expr::HirExprId;
    fn intern_value(self: Arc<Self>, data: value::HirValue) -> value::HirValueId;
    fn intern_pattern(self: Arc<Self>, data: pattern::HirPattern) -> pattern::HirPatternId;
    fn intern_stmt(self: Arc<Self>, data: stmt::HirStmt) -> stmt::HirStmtId;
    fn intern_top_level(self: Arc<Self>, data: top_level::HirTopLevel) -> top_level::HirTopLevelId;
    fn intern_attr(self: Arc<Self>, data: attr::HirAttr) -> attr::HirAttrId;
    fn intern_type(self: Arc<Self>, data: hir_type::HirType) -> hir_type::HirTypeId;
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
#[doc(hidden)]
fn _assert_dyn() {
    fn assert_dyn<T: ?Sized>() {}

    assert_dyn::<dyn HirBag>();
    assert_dyn::<dyn HirInterner>();
}
