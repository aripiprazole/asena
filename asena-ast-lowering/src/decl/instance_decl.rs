use asena_ast::Instance;
use asena_hir::{database::HirBag, top_level::HirTopLevelId};

use crate::AstLowering;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_instance(&self, instance_decl: Instance) -> HirTopLevelId {
        todo!()
    }
}
