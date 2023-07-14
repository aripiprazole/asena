use crate::attr::*;
use crate::expr::*;
use crate::hir_type::*;
use crate::pattern::*;
use crate::stmt::*;
use crate::top_level::*;
use crate::value::*;
use crate::Name;

#[salsa::query_group(HirStorage)]
pub trait HirInterner {
    #[salsa::interned]
    fn intern_name(&self, data: String) -> Name;

    #[salsa::interned]
    fn intern_attr(&self, data: HirAttrData) -> HirAttr;

    #[salsa::interned]
    fn intern_expr(&self, data: HirExprData) -> HirExpr;

    #[salsa::interned]
    fn intern_pattern(&self, data: HirPatternData) -> HirPattern;

    #[salsa::interned]
    fn intern_stmt(&self, data: HirStmtData) -> HirStmt;

    #[salsa::interned]
    fn intern_type(&self, data: HirTypeData) -> HirType;

    #[salsa::interned]
    fn intern_value(&self, data: HirValueData) -> HirValue;

    #[salsa::interned]
    fn intern_top_level(&self, data: HirTopLevelData) -> HirTopLevel;
}
