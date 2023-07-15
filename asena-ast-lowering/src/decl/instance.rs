use asena_ast::{Instance, Typed};
use asena_hir::{
    hir_type::HirType,
    top_level::{HirTopLevel, HirTopLevelData, HirTopLevelInstance},
};
use asena_leaf::ast::AstParam;

use crate::{db::AstLowerrer, make_location};

use super::{compute_methods, compute_parameters};

pub fn lower_instance(db: &dyn AstLowerrer, decl: AstParam<Instance>) -> HirTopLevel {
    let span = make_location(db, &decl);
    let kind = HirTopLevelInstance {
        parameters: compute_parameters(db, &decl.data),
        signature: match decl.gadt_type() {
            Typed::Infer => HirType::error(db),
            Typed::Explicit(type_expr) => db.hir_type(type_expr.into()),
        },
        groups: compute_methods(db, decl.methods()),
    };

    db.intern_top_level(HirTopLevelData {
        kind: kind.into(),
        attributes: vec![],
        docs: vec![],
        span,
    })
}
