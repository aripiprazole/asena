use asena_ast::{Class, Field, GlobalName, Typed};
use asena_hir::{
    database::HirBag,
    hir_type::HirTypeId,
    top_level::{data::HirSignature, HirTopLevel, HirTopLevelId, HirTopLevelStruct},
    NameId,
};
use asena_leaf::ast::Located;
use im::HashMap;

use crate::AstLowering;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_class(&self, class_decl: Class) -> HirTopLevelId {
        let location = class_decl.location().into_owned();
        let name = NameId::intern(self.jar.clone(), class_decl.name().to_fn_id().as_str());
        let kind = HirTopLevelStruct {
            signature: HirSignature {
                name,
                parameters: self.compute_parameters(&class_decl),
                return_type: None, // class can not be gadt
            },
            fields: self.lower_fields(class_decl.fields()),
            groups: self.compute_methods(class_decl.methods()),
        };

        HirTopLevel::new(self.jar.clone(), kind.into(), vec![], vec![], location)
    }

    fn lower_fields(&self, fields: Vec<Field>) -> HashMap<NameId, HirTypeId> {
        let mut map = HashMap::new();
        for field in fields {
            let name = NameId::intern(self.jar.clone(), field.name().to_fn_id().as_str());
            match field.field_type() {
                Typed::Infer => {
                    // TODO: handle error
                    // a field cannot be infer
                }
                Typed::Explicit(type_expr) => {
                    let type_id = self.lower_type(type_expr);
                    map.insert(name, type_id);
                }
            };
        }
        map
    }
}
