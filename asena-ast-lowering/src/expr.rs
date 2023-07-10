use asena_hir::stmt::HirStmtId;

use super::*;

use std::sync::{Arc, Weak};

pub struct ExprLowering<D> {
    pub db: Arc<D>,
    pub instructions: Vec<HirStmtId>,
    pub lowerrer: Weak<AstLowering<D>>,
}

impl<D: HirBag + 'static> ExprLowering<D> {
    pub fn new(lowerrer: Weak<AstLowering<D>>, db: Arc<D>) -> Self {
        Self {
            db,
            lowerrer,
            instructions: vec![],
        }
    }

    pub fn make(&mut self, expr: Expr) -> HirExprId {
        let kind = match expr {
            Expr::Error => HirExprKind::Error,
            Expr::SelfExpr(_) => HirExprKind::This,
            Expr::Unit(_) => HirExprKind::Unit,
            Expr::Group(ref group) => HirExprKind::from(HirExprGroup {
                value: self.lowerrer().lower_value(group.value()),
            }),
            Expr::Infix(ref infix) => {
                let lhs = self.lowerrer().lower_value(infix.lhs());
                let rhs = self.lowerrer().lower_value(infix.rhs());

                let callee = match infix.fn_id().as_str() {
                    "+" => HirCallee::Add,
                    "-" => HirCallee::Sub,
                    "*" => HirCallee::Mul,
                    "/" => HirCallee::Div,
                    fn_id => self.make_callee(infix, fn_id),
                };

                HirExprKind::from(HirExprCall {
                    callee,
                    arguments: vec![lhs, rhs],
                    as_dsl: None,
                })
            }
            Expr::App(ref app) => {
                let callee = self.lowerrer().lower_value(app.callee());
                let argument = self.lowerrer().lower_value(app.argument());

                HirExprKind::from(HirExprCall {
                    callee: HirCallee::Value(callee),
                    arguments: vec![argument],
                    as_dsl: None,
                })
            }
            Expr::Array(ref array) => {
                let items = array
                    .items()
                    .into_iter()
                    .map(|item| self.lowerrer().lower_value(item))
                    .collect();

                HirExprKind::from(HirExprArray { items })
            }
            Expr::Dsl(_) => todo!(),
            Expr::Lam(_) => todo!(),
            Expr::Let(_) => todo!("lowering let expressions is not yet implemented"),
            Expr::If(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::Ann(_) => todo!(),
            Expr::Qual(_) => todo!(),
            Expr::Pi(_) => todo!(),
            Expr::Sigma(_) => todo!(),
            Expr::Help(_) => todo!(),
            Expr::LocalExpr(_) => todo!(),
            Expr::LiteralExpr(ref expr) => {
                let literal = self.lowerrer().make_literal(expr.literal().data().clone());

                HirExprKind::from(HirExprLiteral(literal))
            }
        };

        HirExpr::new(self.db.clone(), kind, expr.location().into_owned())
    }

    fn make_callee(&self, infix: &Infix, fn_id: &str) -> HirCallee {
        let loc = infix.fn_id().location().into_owned();
        let name = self.db.clone().intern_name(fn_id.into());
        let reference = HirExprReference { name };
        let expr = HirExpr::new(self.db.clone(), reference.into(), loc.clone());
        let value = HirValueExpr(expr);
        let value = HirValue::new(self.db.clone(), value.into(), loc);

        HirCallee::Value(value)
    }

    fn lowerrer(&self) -> Arc<AstLowering<D>> {
        self.lowerrer.upgrade().unwrap()
    }
}
