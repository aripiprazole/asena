use asena_ast::{Class, Field, GlobalName, Typed};
use asena_hir::{
    database::HirBag,
    hir_type::HirTypeId,
    top_level::{data::HirSignature, HirTopLevel, HirTopLevelId, HirTopLevelStruct},
    NameId,
};
use im::HashMap;

use crate::error::AstLoweringError::*;
use crate::AstLowering;

impl<DB: HirBag + 'static> AstLowering<'_, DB> {
    pub fn lower_class(&self, class_decl: Class) -> HirTopLevelId {
        let location = self.make_location(&class_decl);
        let name = NameId::intern(self.jar(), class_decl.name().to_fn_id().as_str());
        let kind = HirTopLevelStruct {
            signature: HirSignature {
                name,
                parameters: self.compute_parameters(&class_decl),
                return_type: None, // class can not be gadt
            },
            fields: self.lower_fields(class_decl.fields()),
            groups: self.compute_methods(class_decl.methods()),
        };

        HirTopLevel::new(self.jar(), kind.into(), vec![], vec![], location)
    }

    fn lower_fields(&self, fields: Vec<Field>) -> HashMap<NameId, HirTypeId> {
        let mut map = HashMap::new();
        for field in fields {
            let name = NameId::intern(self.jar(), field.name().to_fn_id().as_str());
            match field.field_type() {
                // a field cannot be infer
                Typed::Infer => self.reporter().report(&field, FieldTypeCanNotBeInferError),
                Typed::Explicit(type_expr) => {
                    let type_id = self.lower_type(type_expr);
                    map.insert(name, type_id);
                }
            };
        }
        map
    }
}
