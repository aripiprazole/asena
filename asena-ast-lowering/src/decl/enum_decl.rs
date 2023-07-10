use asena_ast::{Enum, GlobalName, Typed, Variant};
use asena_hir::database::HirBag;
use asena_hir::hir_type::HirType;
use asena_hir::top_level::{
    data::{HirSignature, HirVariant},
    HirTopLevelEnum,
};
use asena_hir::top_level::{HirTopLevel, HirTopLevelId};
use asena_hir::NameId;
use im::HashMap;
use itertools::Itertools;

use crate::AstLowering;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_enum(&self, enum_decl: Enum) -> HirTopLevelId {
        let location = self.make_location(&enum_decl);
        let name = NameId::intern(self.jar.clone(), enum_decl.name().to_fn_id().as_str());
        let kind = HirTopLevelEnum {
            signature: HirSignature {
                name,
                parameters: self.compute_parameters(&enum_decl),
                return_type: match enum_decl.gadt_type() {
                    Typed::Infer => None,
                    Typed::Explicit(type_expr) => Some(self.lower_type(type_expr)),
                },
            },
            variants: self.lower_variants(&enum_decl),
            groups: self.compute_methods(enum_decl.methods()),
        };

        HirTopLevel::new(self.jar.clone(), kind.into(), vec![], vec![], location)
    }

    fn lower_variants(&self, enum_decl: &Enum) -> HashMap<NameId, HirVariant> {
        let mut map = HashMap::new();

        let variants = enum_decl.variants();
        let enum_name = enum_decl.name();
        let enum_name = NameId::intern(self.jar.clone(), enum_name.to_fn_id().as_str());

        for variant in variants {
            let name = NameId::intern(self.jar.clone(), variant.name().to_fn_id().as_str());
            let variant_type = match variant {
                Variant::Error => HirType::error(self.jar.clone()),
                Variant::TypeVariant(type_variant) => match type_variant.value() {
                    Typed::Infer => HirType::constructor(self.jar.clone(), enum_name),
                    Typed::Explicit(variant_type) => self.lower_type(variant_type),
                },
                Variant::ConstructorVariant(variant) => {
                    let parameters = variant
                        .parameters()
                        .iter()
                        .cloned()
                        .filter_map(|parameter| match parameter {
                            Typed::Infer => {
                                // TODO: handle illegal declaration
                                None
                            }
                            Typed::Explicit(type_expr) => Some(self.lower_type(type_expr)),
                        })
                        .collect_vec();
                    let enum_value_type = HirType::constructor(self.jar.clone(), enum_name);

                    HirType::pi(self.jar.clone(), parameters.as_slice(), enum_value_type)
                }
            };

            map.insert(name, HirVariant { name, variant_type });
        }

        map
    }
}
