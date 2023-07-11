use asena_ast::Stmt;
use asena_hir::{
    expr::data::{HirMatchCase, HirMatchKind},
    pattern::HirPattern,
    stmt::{HirStmt, HirStmtAsk, HirStmtId, HirStmtKind, HirStmtLet, HirStmtValue},
    value::runtime::HirValueInstrBlock,
};

use super::*;

type Instr = (HirStmtId, Option<HirValueId>);

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_stmt(&self, stmt: Stmt) -> Instr {
        let kind = match stmt {
            Stmt::Error => HirStmtKind::Error,
            Stmt::Ask(ref stmt) => {
                let pattern = self.lower_pattern(stmt.pattern());
                let value = self.lower_value(stmt.value());

                HirStmtKind::from(HirStmtAsk { value, pattern })
            }
            Stmt::Return(ref stmt) => {
                let value = match stmt.value() {
                    Some(value) => self.lower_value(value),
                    None => HirValue::pure_unit(self.jar()),
                };

                let kind = HirStmtKind::from(HirStmtValue(value));
                let stmt = HirStmt::new(self.jar(), kind, self.make_location(stmt));

                return (stmt, Some(value));
            }
            Stmt::IfStmt(ref stmt) => {
                let expr = self
                    .jar()
                    .intern_expr(HirExpr::from(HirExprKind::from(HirExprMatch {
                        scrutinee: self.lower_value(stmt.cond()),
                        cases: hashset![
                            HirMatchCase {
                                pattern: HirPattern::new_true(self.jar()),
                                value: self.lower_branch(stmt.then_branch()),
                            },
                            HirMatchCase {
                                pattern: HirPattern::new_false(self.jar()),
                                value: match stmt.else_branch() {
                                    Some(else_branch) => self.lower_branch(else_branch),
                                    None => HirBranch::Expr(HirValue::pure_unit(self.jar())),
                                },
                            }
                        ],
                        kind: HirMatchKind::If,
                    })));

                let value = HirValue::value(self.jar(), expr);

                HirStmtKind::from(HirStmtValue(value))
            }
            Stmt::LetStmt(ref stmt) => {
                let pattern = self.lower_pattern(stmt.pattern());
                let value = self.lower_value(stmt.value());

                HirStmtKind::from(HirStmtLet { pattern, value })
            }
            Stmt::ExprStmt(ref stmt) => {
                let value = self.lower_value(stmt.value());

                let kind = HirStmtKind::from(HirStmtValue(value));
                let stmt = HirStmt::new(self.jar(), kind, self.make_location(stmt));

                return (stmt, Some(value));
            }
        };

        let stmt = HirStmt::new(self.jar(), kind, self.make_location(&stmt));

        (stmt, None)
    }

    pub fn lower_block(&self, block: Vec<Stmt>) -> HirValueId {
        let mut stmts = vec![];
        let mut last = None;

        for stmt in block.iter() {
            let (stmt, value) = self.lower_stmt(stmt.clone());
            stmts.push(stmt);
            last = value;
        }

        let value = last.unwrap();
        let stmts = {
            let kind = HirValueKind::from(HirValueBlock {
                instructions: stmts,
                value,
            });

            HirValue::new(self.jar(), kind, self.make_location(&block))
        };

        let kind = HirValueKind::from(HirValueInstrBlock {
            instructions: vec![],
            value: stmts,
        });

        HirValue::new(self.jar(), kind, self.make_location(&block))
    }
}
