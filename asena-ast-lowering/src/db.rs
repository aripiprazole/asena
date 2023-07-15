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
use asena_leaf::ast::Located;
use im::{HashMap, HashSet};

use crate::stmt::Instr;

#[salsa::query_group(AstLowerrerStorage)]
pub trait AstLowerrer: AstDatabase + HirInterner {
    fn hir_file(&self, file: AsenaFile) -> InternalAsenaFile;

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

fn hir_file(db: &dyn AstLowerrer, file: AsenaFile) -> InternalAsenaFile {
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
                declarations.insert(db.lower_class(class_decl));
            }
            Decl::Instance(instance_decl) => {
                declarations.insert(db.lower_instance(instance_decl));
            }
            Decl::Trait(trait_decl) => {
                declarations.insert(db.lower_trait(trait_decl));
            }
            Decl::Enum(enum_decl) => {
                declarations.insert(db.lower_enum(enum_decl));
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
