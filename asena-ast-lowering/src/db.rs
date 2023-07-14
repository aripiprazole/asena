use std::sync::Arc;

use asena_ast::{db::ReporterDatabase, reporter::Reporter, *};
use asena_hir::{
    expr::data::HirBranch, hir_type::HirType, interner::HirInterner, pattern::HirPattern,
    top_level::HirTopLevel, value::HirValue,
};

use crate::stmt::Instr;

#[salsa::query_group(AstLowerrerStorage)]
pub trait AstLowerrer: HirInterner + ReporterDatabase {
    #[salsa::invoke(crate::decl::r#trait::lower_trait)]
    fn lower_trait(&self, decl: Trait) -> HirTopLevel;

    #[salsa::invoke(crate::decl::class::lower_class)]
    fn lower_class(&self, decl: Class) -> HirTopLevel;

    #[salsa::invoke(crate::decl::instance::lower_instance)]
    fn lower_instance(&self, decl: Instance) -> HirTopLevel;

    #[salsa::invoke(crate::decl::r#enum::lower_enum)]
    fn lower_enum(&self, decl: Enum) -> HirTopLevel;

    #[salsa::invoke(crate::types::lower_type)]
    fn lower_type(&self, expr: Expr) -> HirType;

    #[salsa::invoke(crate::stmt::lower_stmt)]
    fn lower_stmt(&self, stmt: Stmt) -> Instr;

    #[salsa::invoke(crate::stmt::lower_block)]
    fn lower_block(&self, stmts: Vec<Stmt>) -> HirValue;

    #[salsa::invoke(crate::pattern::lower_pattern)]
    fn lower_pattern(&self, pattern: Pat) -> HirPattern;

    #[salsa::invoke(crate::lower_value)]
    fn lower_value(&self, expr: Expr) -> HirValue;

    #[salsa::invoke(crate::lower_branch)]
    fn lower_branch(&self, branch: Branch) -> HirBranch;
}
