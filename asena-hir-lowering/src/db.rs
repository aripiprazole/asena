use std::sync::Arc;

use asena_ast_db::package::Package;
use asena_hir_db::db::HirDatabase;
use inkwell::context::Context;

use crate::{cg::CgLowering, LlirErr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LlirPackage;

#[salsa::query_group(LlirStorage)]
pub trait LlirDatabase: HirDatabase {
    fn llir_package(&self, pkg: Package) -> eyre::Result<Arc<LlirPackage>, LlirErr>;
}

fn llir_package(db: &dyn LlirDatabase, pkg: Package) -> Result<Arc<LlirPackage>, LlirErr> {
    // Discovery step
    let mut defs = db.hir_defs(pkg);

    let data = db.lookup_intern_package(pkg);
    for data in data.dependencies {
        let local_defs = db.hir_defs(data);
        defs.extend(local_defs);
    }

    let ctx = Context::create();
    let _cg = CgLowering::new(db, pkg, &ctx);

    // let main = db
    //     .hir_find_fn(pkg, "main".into())
    //     .ok_or(LlirErr::MainNotFound(data.name))?;

    todo!()
}
