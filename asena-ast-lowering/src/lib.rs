use asena_ast::Expr;
use asena_hir::{
    expr::{HirExpr, HirExprGroup, HirExprId, HirExprKind},
    value::{HirExprValue, HirValue, HirValueId, HirValueKind},
};
use asena_hir_leaf::HirBaseDatabase;
use asena_leaf::ast::Located;

pub struct AstLowering<'a, D: HirBaseDatabase> {
    pub db: &'a D,
}

impl<'a, D: HirBaseDatabase> AstLowering<'a, D> {
    pub fn new(db: &'a D) -> Self {
        Self { db }
    }

    pub fn run_lower_value(&self, value: Expr) -> HirValueId {
        let location = value.location().into_owned();
        let value_id = self.run_lower_expr(value);
        let kind = HirValueKind::Expr(HirExprValue(value_id));

        HirValue::new(self.db, kind, location)
    }

    pub fn run_lower_expr(&self, expr: Expr) -> HirExprId {
        let kind = match expr {
            Expr::Error => HirExprKind::Error,
            Expr::SelfExpr(_) => HirExprKind::This,
            Expr::Unit(_) => HirExprKind::Unit,
            Expr::Group(ref group) => HirExprKind::Group(HirExprGroup {
                value: self.run_lower_value(group.value()),
            }),
            Expr::Infix(_) => todo!(),
            Expr::App(_) => todo!(),
            Expr::Array(_) => todo!(),
            Expr::Dsl(_) => todo!(),
            Expr::Lam(_) => todo!(),
            Expr::Let(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::Ann(_) => todo!(),
            Expr::Qual(_) => todo!(),
            Expr::Pi(_) => todo!(),
            Expr::Sigma(_) => todo!(),
            Expr::Help(_) => todo!(),
            Expr::LocalExpr(_) => todo!(),
            Expr::LiteralExpr(_) => todo!(),
        };

        HirExpr::new(self.db, kind, expr.location().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asena_ast::Expr;
    use asena_hir::value::HirValueKind;
    use asena_hir_leaf::HirBaseDatabase;

    #[test]
    fn test_lower_value() {
        let db = HirBaseDatabase::default();
        let lowering = AstLowering::new(&db);
        let value_id = lowering.run_lower_value(value);

        assert_eq!(
            db.value_data(value_id).kind,
            HirValueKind::Expr(HirExprValue(
                db.expr_data(db.expr_data(value_id).kind.into()).value
            ))
        );
    }
}
