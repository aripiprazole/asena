use asena_ast::{Binary, Expr, Infix, Literal, Signed};
use asena_hir::{
    database::HirBag,
    expr::{
        data::HirCallee, HirExpr, HirExprArray, HirExprCall, HirExprGroup, HirExprId, HirExprKind,
        HirExprLiteral, HirExprReference,
    },
    literal::{HirDecimal, HirFSize, HirISize, HirIntSign, HirLiteral, HirString},
    value::{HirValue, HirValueExpr, HirValueId, HirValueKind},
};
use asena_leaf::ast::Located;

pub struct AstLowering<'a, D> {
    pub db: &'a D,
}

impl<'a, D: HirBag> AstLowering<'a, D> {
    pub fn new(db: &'a D) -> Self {
        Self { db }
    }

    pub fn run_lower_value(&self, value: Expr) -> HirValueId {
        let location = value.location().into_owned();
        let value_id = self.run_lower_expr(value);
        let kind = HirValueKind::HirValueExpr(HirValueExpr(value_id));

        HirValue::new(self.db, kind, location)
    }

    pub fn run_lower_expr(&self, expr: Expr) -> HirExprId {
        let kind = match expr {
            Expr::Error => HirExprKind::Error,
            Expr::SelfExpr(_) => HirExprKind::This,
            Expr::Unit(_) => HirExprKind::Unit,
            Expr::Group(ref group) => HirExprKind::from(HirExprGroup {
                value: self.run_lower_value(group.value()),
            }),
            Expr::Infix(ref infix) => {
                let lhs = self.run_lower_value(infix.lhs());
                let rhs = self.run_lower_value(infix.rhs());

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
                let callee = self.run_lower_value(app.callee());
                let argument = self.run_lower_value(app.argument());

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
                    .map(|item| self.run_lower_value(item))
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

        HirExpr::new(self.db, kind, expr.location().into_owned())
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
        let name = self.db.intern_name(fn_id.into());
        let reference = HirExprReference { name };
        let expr = HirExpr::new(self.db, reference.into(), loc.clone());
        let value = HirValueExpr(expr);
        let value = HirValue::new(self.db, value.into(), loc);

        HirCallee::Value(value)
    }
}

#[cfg(test)]
mod tests {
    use asena_ast::Expr;
    use asena_grammar::asena_expr;
    use asena_hir::database::HirBag;
    use asena_hir_leaf::hir_dbg;
    use asena_leaf::ast::Node;

    #[test]
    fn it_works() {
        let db = asena_hir_db::HirDatabase::default();
        let ast_lowering = super::AstLowering::new(&db);

        let expr = asena_expr! { 1 + 1 };
        let id = ast_lowering.run_lower_value(Expr::new(expr));
        let value = db.value_data(id);

        println!("{:#?}", hir_dbg!(db, value));
    }
}
