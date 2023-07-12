use asena_hir::hir_type::{
    data::{HirTypeArgument, HirTypeFunction},
    HirType, HirTypeApp, HirTypeKind, HirTypeName,
};
use if_chain::if_chain;

use super::*;

impl<DB: HirBag + 'static> AstLowering<'_, DB> {
    pub fn lower_type(&self, expr: Expr) -> HirTypeId {
        let kind = match expr {
            Expr::SelfExpr(_) => HirTypeKind::This,
            Expr::Unit(_) => HirTypeKind::Unit,
            Expr::Error => HirTypeKind::Error,
            // TODO: handle dependent types
            Expr::Infix(_) => HirTypeKind::Error,
            Expr::Array(_) => HirTypeKind::Error,
            Expr::Dsl(_) => HirTypeKind::Error,
            Expr::Lam(_) => HirTypeKind::Error,
            Expr::Let(_) => HirTypeKind::Error,
            Expr::If(_) => HirTypeKind::Error,
            Expr::Match(_) => HirTypeKind::Error,
            Expr::Ann(_) => HirTypeKind::Error,
            Expr::Qual(_) => HirTypeKind::Error,
            Expr::Sigma(_) => HirTypeKind::Error,
            Expr::Help(_) => HirTypeKind::Error,
            Expr::LiteralExpr(_) => HirTypeKind::Error,
            //
            Expr::Group(ref group) => return self.lower_type(group.value()),
            Expr::Pi(ref pi) => {
                let lhs = self.lower_type(pi.parameter_type());
                let rhs = self.lower_type(pi.return_type());
                let parameter = match pi.parameter_name() {
                    Some(name) => {
                        let name = NameId::intern(self.jar.clone(), name.as_str());
                        HirTypeArgument::Named(name, lhs)
                    }
                    None => HirTypeArgument::Type(lhs),
                };

                HirTypeKind::from(HirTypeApp {
                    callee: HirTypeFunction::Pi,
                    arguments: vec![parameter, HirTypeArgument::Type(rhs)],
                })
            }
            Expr::App(ref app) => {
                let callee = self.lower_type(app.callee());
                let argument = self.lower_type(app.argument());

                HirTypeKind::from(HirTypeApp {
                    callee: HirTypeFunction::Type(callee),
                    arguments: vec![HirTypeArgument::Type(argument)],
                })
            }
            Expr::LocalExpr(ref local) => {
                let str = local.clone().to_fn_id();
                let name = NameId::intern(self.jar.clone(), str.as_str());
                let mut is_constructor = false;

                if_chain! {
                    if let Some(c) = str.as_str().chars().next();
                    if c.is_uppercase();
                    then {
                        is_constructor = true;
                    }
                }

                HirTypeKind::from(HirTypeName {
                    is_constructor,
                    name,
                })
            }
        };

        HirType::new(self.jar.clone(), kind, self.make_location(&expr))
    }
}
