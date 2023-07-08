use std::fmt::Debug;

use crate::HirBaseDatabase;

pub trait HirDebug {
    fn fmt(&self, db: &dyn HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

pub struct HirDebugger<'ctx> {
    db: &'ctx dyn HirBaseDatabase,
    node: &'ctx dyn HirDebug,
}

impl<'ctx> HirDebugger<'ctx> {
    pub fn new(db: &'ctx dyn HirBaseDatabase, node: &'ctx dyn HirDebug) -> Self {
        Self { db, node }
    }
}

impl Debug for HirDebugger<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.node.fmt(self.db, f)
    }
}

#[macro_export]
macro_rules! hir_dbg {
    ($db:expr, $e:expr) => {
        $crate::HirDebugger::new(&$db, &$e)
    };
}
