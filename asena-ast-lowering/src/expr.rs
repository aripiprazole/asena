use super::*;

use std::sync::{Arc, Weak};

pub struct ExprLowering<D> {
    pub db: Arc<D>,
    pub instructions: Vec<HirValueId>,
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
                let literal = self.make_literal(expr.literal().data().clone());

                HirExprKind::from(HirExprLiteral(literal))
            }
        };

        HirExpr::new(self.db.clone(), kind, expr.location().into_owned())
    }

    fn make_literal(&self, literal: Literal) -> HirLiteral {
        match literal {
            Literal::Error => HirLiteral::Error,
            Literal::True => HirLiteral::Int(1, HirISize::U1, HirIntSign::Unsigned),
            Literal::False => HirLiteral::Int(0, HirISize::U1, HirIntSign::Unsigned),
            Literal::String(value) => HirLiteral::String(HirString { value, name: None }),
            Literal::Nat(_) => todo!("lowering nat literals is not yet implemented"),
            Literal::Int8(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U8, HirIntSign::Signed)
            }
            Literal::Int8(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U8, HirIntSign::Unsigned)
            }
            Literal::Int16(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U16, HirIntSign::Signed)
            }
            Literal::Int16(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U16, HirIntSign::Unsigned)
            }
            Literal::Int32(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U32, HirIntSign::Signed)
            }
            Literal::Int32(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U32, HirIntSign::Unsigned)
            }
            Literal::Int64(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U64, HirIntSign::Signed)
            }
            Literal::Int64(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U64, HirIntSign::Unsigned)
            }
            Literal::Int128(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U128, HirIntSign::Signed)
            }
            Literal::Int128(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U128, HirIntSign::Unsigned)
            }
            Literal::Float32(value) => {
                let s = value.clone().to_string();

                let mut split = s.split('.');
                let integer = split.next().unwrap().parse::<usize>().unwrap();
                let decimal = split.next().unwrap_or("0").parse::<usize>().unwrap();

                HirLiteral::Decimal(HirFSize::F64, HirDecimal { integer, decimal })
            }
            Literal::Float64(value) => {
                let s = value.clone().to_string();

                let mut split = s.split('.');
                let integer = split.next().unwrap().parse::<usize>().unwrap();
                let decimal = split.next().unwrap_or("0").parse::<usize>().unwrap();

                HirLiteral::Decimal(HirFSize::F64, HirDecimal { integer, decimal })
            }
        }
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
