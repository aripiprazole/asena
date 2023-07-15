use asena_ast_db::package::Package;
use inkwell::{basic_block::BasicBlock, builder::Builder, context::Context};

use crate::db::LlirDatabase;

pub struct CgLowering<'db, 'ctx> {
    pub db: &'db dyn LlirDatabase,
    pub pkg: Package,

    pub ctx: &'ctx Context,
    pub builder: Builder<'ctx>,

    pub bb: Option<BasicBlock<'ctx>>,
}

impl<'db, 'ctx> CgLowering<'db, 'ctx> {
    pub fn new(db: &'db dyn LlirDatabase, pkg: Package, ctx: &'ctx Context) -> Self {
        Self {
            db,
            pkg,
            ctx,
            builder: ctx.create_builder(),
            bb: None,
        }
    }

    pub fn bb(&self) -> BasicBlock<'ctx> {
        self.bb.unwrap()
    }
}
