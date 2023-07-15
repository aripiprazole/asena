use asena_ast::{Class, Field, GlobalName, Typed};
use asena_ast_db::package::HasDiagnostic;
use asena_hir::{
    hir_type::HirType,
    top_level::{data::HirSignature, HirTopLevel, HirTopLevelData, HirTopLevelStruct},
    Name,
};
use asena_report::WithError;
use im::HashMap;

use crate::{db::AstLowerrer, error::AstLoweringError::*, make_location};

use super::{compute_methods, compute_parameters};

pub fn lower_class(db: &dyn AstLowerrer, decl: Class) -> HirTopLevel {
    let span = make_location(db, &decl);
    let name = db.intern_name(decl.name().to_fn_id().to_string());
    let kind = HirTopLevelStruct {
        signature: HirSignature {
            name,
            parameters: compute_parameters(db, &decl),
            return_type: None, // class can not be gadt
        },
        fields: lower_fields(db, decl.fields()),
        groups: compute_methods(db, decl.methods()),
    };

    db.intern_top_level(HirTopLevelData {
        kind: kind.into(),
        attributes: vec![],
        docs: vec![],
        span,
    })
}

pub fn lower_fields(db: &dyn AstLowerrer, fields: Vec<Field>) -> HashMap<Name, HirType> {
    let mut map = HashMap::new();
    for field in fields {
        let name = db.intern_name(field.name().to_fn_id().to_string());
        match field.field_type() {
            // a field cannot be infer
            Typed::Infer => field.fail(FieldTypeCanNotBeInferError).push(db),
            Typed::Explicit(type_expr) => {
                let type_id = db.lower_type(type_expr);
                map.insert(name, type_id);
            }
        };
    }
    map
}
