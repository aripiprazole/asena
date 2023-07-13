use crate::attr::*;
use crate::expr::*;
use crate::hir_type::*;
use crate::pattern::*;
use crate::stmt::*;
use crate::top_level::*;
use crate::value::*;

#[salsa::query_group(InternerDb)]
pub trait Interner {
    #[salsa::input]
    fn intern_attr(&self, data: HirAttrData) -> HirAttr;

    #[salsa::input]
    fn intern_expr(&self, data: HirExprData) -> HirExpr;

    #[salsa::input]
    fn intern_pattern(&self, data: HirPatternData) -> HirPattern;

    #[salsa::input]
    fn intern_stmt(&self, data: HirStmtData) -> HirStmt;

    #[salsa::input]
    fn intern_type(&self, data: HirTypeData) -> HirType;

    #[salsa::input]
    fn intern_value(&self, data: HirValueData) -> HirValue;

    #[salsa::input]
    fn intern_top_level(&self, data: HirTopLevelData) -> HirTopLevel;
}
