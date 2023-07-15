#![feature(trait_upcasting)]

use asena_ast::*;

pub mod commands;
pub mod db;

pub use commands::*;
pub use db::*;

pub struct PrecReorder<'db> {
    pub db: &'db dyn PrecDatabase,
}

impl<'db> AsenaVisitor<()> for PrecReorder<'db> {
    fn visit_qual(&mut self, value: Qual) {
        self.impl_reorder_prec(&value);
    }
}

impl<'db> PrecReorder<'db> {
    /// Reorder the precedence of the binary expression.
    fn impl_reorder_prec(&mut self, binary: &impl Binary) -> Option<()> {
        let lhs = binary.lhs();
        let fn_id = binary.fn_id();
        let rhs = binary.rhs().as_binary()?;

        let prec_table = self.db.prec_table();
        let prec_table = prec_table.read().unwrap();

        let op1 = prec_table.get(&fn_id)?;
        let op2 = prec_table.get(&rhs.fn_id())?;

        if op1.order > op2.order {
            let new_lhs = binary.as_new_ast::<VirtualBinary>();
            new_lhs.set_lhs(lhs);
            new_lhs.set_fn_id(fn_id);
            new_lhs.set_rhs(rhs.find_lhs().as_new_node().as_leaf());

            binary.set_lhs(new_lhs);
            binary.set_fn_id(rhs.fn_id());
            binary.set_rhs(rhs.rhs());
        }

        Some(())
    }
}
