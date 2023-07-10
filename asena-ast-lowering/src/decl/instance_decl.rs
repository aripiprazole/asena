use asena_ast::{Instance, Typed};
use asena_hir::{
    database::HirBag,
    hir_type::HirType,
    top_level::{HirTopLevel, HirTopLevelId, HirTopLevelInstance},
};

use crate::AstLowering;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_instance(&self, instance_decl: Instance) -> HirTopLevelId {
        let location = self.make_location(&instance_decl);
        let kind = HirTopLevelInstance {
            parameters: self.compute_parameters(&instance_decl),
            signature: match instance_decl.gadt_type() {
                Typed::Infer => HirType::error(self.jar.clone()),
                Typed::Explicit(type_expr) => self.lower_type(type_expr),
            },
            groups: self.compute_methods(instance_decl.methods()),
        };

        HirTopLevel::new(self.jar.clone(), kind.into(), vec![], vec![], location)
    }
}
