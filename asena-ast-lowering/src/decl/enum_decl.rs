use asena_ast::{Enum, GlobalName, Typed, Variant};
use asena_hir::database::HirBag;
use asena_hir::top_level::{
    data::{HirSignature, HirVariant},
    HirTopLevelEnum,
};
use asena_hir::top_level::{HirTopLevel, HirTopLevelId};
use asena_hir::NameId;
use asena_leaf::ast::Located;
use im::HashMap;

use crate::AstLowering;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_enum(&self, enum_decl: Enum) -> HirTopLevelId {
        let location = enum_decl.location().into_owned();
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
            variants: self.lower_variants(enum_decl.variants()),
            groups: self.compute_methods(enum_decl.methods()),
        };

        HirTopLevel::new(self.jar.clone(), kind.into(), vec![], vec![], location)
    }

    fn lower_variants(&self, variants: Vec<Variant>) -> HashMap<NameId, HirVariant> {
        let mut map = HashMap::new();

        for variant in variants {
            let name = NameId::intern(self.jar.clone(), variant.name().to_fn_id().as_str());
            let variant_type = match variant {
                Variant::Error => todo!(),
                Variant::TypeVariant(type_variant) => match type_variant.value() {
                    Typed::Infer => todo!(),
                    Typed::Explicit(variant_type) => self.lower_type(variant_type),
                },
                Variant::ConstructorVariant(_constructor_variant) => {
                    todo!("Transforms enum variant into GADT")
                }
            };

            map.insert(name, HirVariant { name, variant_type });
        }

        map
    }
}
