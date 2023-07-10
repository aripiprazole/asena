use asena_ast::Stmt;
use asena_hir::stmt::{HirStmt, HirStmtId, HirStmtKind, HirStmtValue};

use super::*;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_stmt(&self, stmt: Stmt) -> (HirStmtId, Option<HirValueId>) {
        let kind = match stmt {
            Stmt::Error => HirStmtKind::Error,
            Stmt::Ask(_) => todo!(),
            Stmt::Return(_) => todo!(),
            Stmt::IfStmt(_) => todo!(),
            Stmt::LetStmt(_) => todo!(),
            Stmt::ExprStmt(ref stmt) => {
                let value = self.lower_value(stmt.value());

                let kind = HirStmtKind::from(HirStmtValue(value));
                let stmt = HirStmt::new(self.jar.clone(), kind, stmt.location().into_owned());

                return (stmt, Some(value));
            }
        };

        let stmt = HirStmt::new(self.jar.clone(), kind, stmt.location().into_owned());
        (stmt, None)
    }

    pub fn lower_block(&self, block: Vec<Stmt>) -> HirValueId {
        let mut instructions = vec![];
        let mut last = None;

        for stmt in block.iter() {
            let (stmt, value) = self.lower_stmt(stmt.clone());
            instructions.push(stmt);
            last = value;
        }

        let value = last.unwrap();
        let kind = HirValueKind::from(HirValueBlock {
            instructions,
            value,
        });

        HirValue::new(self.jar.clone(), kind, block.location().into_owned())
    }
}
