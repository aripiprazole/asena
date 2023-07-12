use asena_ast::{Ask, ExprStmt, IfStmt, LetStmt, Return, Stmt};
use asena_hir::{
    expr::data::{HirMatchCase, HirMatchKind},
    pattern::HirPattern,
    stmt::{HirStmt, HirStmtAsk, HirStmtId, HirStmtKind, HirStmtLet, HirStmtValue},
    value::instr::{Block, HirInstr},
};

use super::*;

type Instr = (HirStmtId, Option<HirValueId>);

impl<DB: HirBag + 'static> AstLowering<'_, DB> {
    pub fn lower_stmt(&self, stmt: Stmt) -> Instr {
        let kind = match stmt {
            Stmt::Error => HirStmtKind::Error,
            Stmt::Ask(ref stmt) => self.make_ask(stmt),
            Stmt::Return(ref stmt) => return self.make_return(stmt),
            Stmt::IfStmt(ref stmt) => self.make_if(stmt),
            Stmt::LetStmt(ref stmt) => self.make_let(stmt),
            Stmt::ExprStmt(ref stmt) => return self.make_value(stmt),
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

        let value = last.unwrap_or_else(|| HirValue::pure_unit(self.jar()));
        let stmts = {
            let kind = HirValueKind::from(HirValueBlock {
                instructions: stmts,
                value,
            });

            HirValue::new(self.jar(), kind, self.make_location(&block))
        };

        let kind = HirValueKind::from(HirInstr::Block(Block {
            instructions: vec![],
            value: stmts,
        }));

        HirValue::new(self.jar(), kind, self.make_location(&block))
    }

    fn make_value(&self, stmt: &ExprStmt) -> Instr {
        let value = self.lower_value(stmt.value());

        let kind = HirStmtKind::from(HirStmtValue(value));
        let stmt = HirStmt::new(self.jar(), kind, self.make_location(stmt));

        (stmt, Some(value))
    }

    fn make_let(&self, stmt: &LetStmt) -> HirStmtKind {
        let pattern = self.lower_pattern(stmt.pattern());
        let value = self.lower_value(stmt.value());

        HirStmtKind::from(HirStmtLet { value, pattern })
    }

    fn make_ask(&self, stmt: &Ask) -> HirStmtKind {
        let pattern = self.lower_pattern(stmt.pattern());
        let value = self.lower_value(stmt.value());

        HirStmtKind::from(HirStmtAsk { value, pattern })
    }

    fn make_return(&self, stmt: &Return) -> Instr {
        let value = match stmt.value() {
            Some(value) => self.lower_value(value),
            None => HirValue::pure_unit(self.jar()),
        };

        let kind = HirStmtKind::from(HirStmtValue(value));
        let stmt = HirStmt::new(self.jar(), kind, self.make_location(stmt));

        (stmt, Some(value))
    }

    fn make_if(&self, stmt: &IfStmt) -> HirStmtKind {
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
}
