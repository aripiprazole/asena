use asena_ast::{Ask, ExprStmt, IfStmt, LetStmt, Return, Stmt};
use asena_hir::{
    expr::data::{HirMatchCase, HirMatchKind},
    pattern::HirPattern,
    stmt::{HirStmt, HirStmtAsk, HirStmtData, HirStmtKind, HirStmtLet, HirStmtValue},
    value::instr::{Block, HirInstr},
};

use crate::db::AstLowerrer;

use super::*;

pub type Instr = (HirStmt, Option<HirValue>);

pub fn lower_stmt(db: &dyn AstLowerrer, stmt: Stmt) -> Instr {
    let kind = match stmt {
        Stmt::Error => HirStmtKind::Error,
        Stmt::Ask(ref stmt) => make_ask(db, stmt),
        Stmt::IfStmt(ref stmt) => make_if(db, stmt),
        Stmt::LetStmt(ref stmt) => make_let(db, stmt),
        Stmt::Return(ref stmt) => return make_return(db, stmt),
        Stmt::ExprStmt(ref stmt) => return make_value(db, stmt),
    };

    let stmt = db.intern_stmt(HirStmtData {
        kind,
        span: make_location(db, &stmt),
    });

    (stmt, None)
}

pub fn lower_block(db: &dyn AstLowerrer, block: Vec<Stmt>) -> HirValue {
    let mut stmts = vec![];
    let mut last = None;

    for stmt in block.iter() {
        let (stmt, value) = db.hir_stmt(stmt.clone());
        stmts.push(stmt);
        last = value;
    }

    let value = last.unwrap_or_else(|| HirValue::unit(db));
    let stmts = {
        let kind = HirValueKind::from(HirValueBlock {
            instructions: stmts,
            value,
        });

        db.intern_value(HirValueData {
            kind,
            span: make_location(db, &block),
        })
    };

    let kind = HirValueKind::from(HirInstr::Block(Block {
        instructions: vec![],
        value: stmts,
    }));

    db.intern_value(HirValueData {
        kind,
        span: make_location(db, &block),
    })
}

fn make_value(db: &dyn AstLowerrer, stmt: &ExprStmt) -> Instr {
    let value = db.hir_value(stmt.value());

    let kind = HirStmtKind::from(HirStmtValue(value));
    let stmt = db.intern_stmt(HirStmtData {
        kind,
        span: make_location(db, stmt),
    });

    (stmt, Some(value))
}

fn make_let(db: &dyn AstLowerrer, stmt: &LetStmt) -> HirStmtKind {
    let pattern = db.hir_pattern(stmt.pattern());
    let value = db.hir_value(stmt.value());

    HirStmtKind::from(HirStmtLet { value, pattern })
}

fn make_ask(db: &dyn AstLowerrer, stmt: &Ask) -> HirStmtKind {
    let pattern = db.hir_pattern(stmt.pattern());
    let value = db.hir_value(stmt.value());

    HirStmtKind::from(HirStmtAsk { value, pattern })
}

fn make_return(db: &dyn AstLowerrer, stmt: &Return) -> Instr {
    let value = match stmt.value() {
        Some(value) => db.hir_value(value),
        None => HirValue::unit(db),
    };

    let kind = HirStmtKind::from(HirStmtValue(value));
    let stmt = db.intern_stmt(HirStmtData {
        kind,
        span: make_location(db, stmt),
    });

    (stmt, Some(value))
}

fn make_if(db: &dyn AstLowerrer, stmt: &IfStmt) -> HirStmtKind {
    let expr = db.intern_expr(HirExprData::from(HirExprKind::from(HirExprMatch {
        scrutinee: db.hir_value(stmt.cond()),
        cases: hashset![
            HirMatchCase {
                pattern: HirPattern::new_true(db),
                value: db.hir_branch(stmt.then_branch()),
            },
            HirMatchCase {
                pattern: HirPattern::new_false(db),
                value: match stmt.else_branch() {
                    Some(else_branch) => db.hir_branch(else_branch),
                    None => HirBranch::Expr(HirValue::unit(db)),
                },
            }
        ],
        kind: HirMatchKind::If,
    })));

    let value = HirValue::of_expr(db, expr);

    HirStmtKind::from(HirStmtValue(value))
}
