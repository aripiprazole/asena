use asena_ast::*;
use asena_ast_db::db::AstDatabase;
use asena_hir::{
    expr::data::HirBranch,
    file::InternalAsenaFile,
    hir_type::HirType,
    interner::HirInterner,
    pattern::HirPattern,
    top_level::{HirTopLevel, HirTopLevelData, HirTopLevelKind},
    value::HirValue,
};
use asena_leaf::ast::{AstParam, Located};
use im::{HashMap, HashSet};

use crate::stmt::Instr;

#[salsa::query_group(AstLowerrerStorage)]
pub trait AstLowerrer: AstDatabase + HirInterner {
    fn hir_file(&self, file: AstParam<AsenaFile>) -> InternalAsenaFile;

    #[salsa::invoke(crate::decl::r#trait::lower_trait)]
    fn hir_trait(&self, decl: AstParam<Trait>) -> HirTopLevel;

    #[salsa::invoke(crate::decl::class::lower_class)]
    fn hir_class(&self, decl: AstParam<Class>) -> HirTopLevel;

    #[salsa::invoke(crate::decl::instance::lower_instance)]
    fn hir_instance(&self, decl: AstParam<Instance>) -> HirTopLevel;

    #[salsa::invoke(crate::decl::r#enum::lower_enum)]
    fn hir_enum(&self, decl: AstParam<Enum>) -> HirTopLevel;

    #[salsa::invoke(crate::types::lower_type)]
    fn hir_type(&self, expr: AstParam<Expr>) -> HirType;

    #[salsa::invoke(crate::stmt::lower_stmt)]
    fn hir_stmt(&self, stmt: AstParam<Stmt>) -> Instr;

    #[salsa::invoke(crate::stmt::lower_block)]
    fn hir_block(&self, stmts: AstParam<Vec<Stmt>>) -> HirValue;

    #[salsa::invoke(crate::pattern::lower_pattern)]
    fn hir_pattern(&self, pattern: AstParam<Pat>) -> HirPattern;

    #[salsa::invoke(crate::lower_value)]
    fn hir_value(&self, expr: AstParam<Expr>) -> HirValue;

    #[salsa::invoke(crate::lower_branch)]
    fn hir_branch(&self, branch: AstParam<Branch>) -> HirBranch;
}

fn hir_file(db: &dyn AstLowerrer, file: AstParam<AsenaFile>) -> InternalAsenaFile {
    let mut declarations = HashSet::new();
    let mut signatures = HashMap::new();

    for decl in file.declarations() {
        match decl {
            Decl::Error => {}
            Decl::Use(_) => {}
            Decl::Command(_) => {
                // TODO: handle commands
            }
            Decl::Assign(ref decl) => crate::make_assign(db, &mut signatures, decl),
            Decl::Signature(ref decl) => crate::make_signature(db, &mut signatures, decl),
            Decl::Class(class_decl) => {
                declarations.insert(db.hir_class(class_decl.into()));
            }
            Decl::Instance(instance_decl) => {
                declarations.insert(db.hir_instance(instance_decl.into()));
            }
            Decl::Trait(trait_decl) => {
                declarations.insert(db.hir_trait(trait_decl.into()));
            }
            Decl::Enum(enum_decl) => {
                declarations.insert(db.hir_enum(enum_decl.into()));
            }
        };
    }

    for (span, group) in signatures.values().cloned() {
        let top_level = db.intern_top_level(HirTopLevelData {
            kind: HirTopLevelKind::from(group),
            attributes: vec![],
            docs: vec![],
            span,
        });

        declarations.insert(top_level);
    }

    let module = db.location_file(file.location().into_owned());
    let file = db.vfs_file(module.clone());

    InternalAsenaFile {
        path: module,
        content: db.source(file),
        tree: db.cst(file),
        declarations,
    }
}
