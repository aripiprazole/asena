use asena_ast::{Enum, GlobalName, Typed, Variant};
use asena_ast_db::package::HasDiagnostic;
use asena_hir::hir_type::HirType;
use asena_hir::top_level::{
    data::{HirSignature, HirVariant},
    HirTopLevelEnum,
};
use asena_hir::top_level::{HirTopLevel, HirTopLevelData};
use asena_hir::Name;
use asena_report::WithError;
use im::HashMap;
use itertools::Itertools;

use crate::db::AstLowerrer;
use crate::error::AstLoweringError::*;
use crate::make_location;

use super::{compute_methods, compute_parameters};

pub fn lower_enum(db: &dyn AstLowerrer, decl: Enum) -> HirTopLevel {
    let span = make_location(db, &decl);
    let name = db.intern_name(decl.name().to_fn_id().to_string());
    let kind = HirTopLevelEnum {
        signature: HirSignature {
            name,
            parameters: compute_parameters(db, &decl),
            return_type: match decl.gadt_type() {
                Typed::Infer => None,
                Typed::Explicit(type_expr) => Some(db.lower_type(type_expr)),
            },
        },
        variants: lower_variants(db, &decl),
        groups: compute_methods(db, decl.methods()),
    };

    db.intern_top_level(HirTopLevelData {
        kind: kind.into(),
        attributes: vec![],
        docs: vec![],
        span,
    })
}

pub fn lower_variants(db: &dyn AstLowerrer, decl: &Enum) -> HashMap<Name, HirVariant> {
    let mut map = HashMap::new();

    let variants = decl.variants();
    let enum_name = db.intern_name(decl.name().to_fn_id().to_string());

    for variant in variants {
        let name = db.intern_name(variant.name().to_fn_id().to_string());
        let variant_type = match variant {
            Variant::Error => HirType::error(db),
            Variant::TypeVariant(type_variant) => match type_variant.value() {
                Typed::Infer => HirType::constructor(db, enum_name),
                Typed::Explicit(variant_type) => db.lower_type(variant_type),
            },
            Variant::ConstructorVariant(variant) => {
                let parameters = variant
                    .parameters()
                    .iter()
                    .cloned()
                    .filter_map(|parameter| match parameter {
                        Typed::Infer => {
                            parameter.fail(VariantParameterCanNotBeInferError).push(db);

                            None
                        }
                        Typed::Explicit(type_expr) => Some(db.lower_type(type_expr)),
                    })
                    .collect_vec();
                let enum_value_type = HirType::constructor(db, enum_name);

                HirType::pi(db, parameters.as_slice(), enum_value_type)
            }
        };

        map.insert(name, HirVariant { name, variant_type });
    }

    map
}
