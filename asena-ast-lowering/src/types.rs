use asena_hir::hir_type::{data::*, *};
use if_chain::if_chain;

use crate::db::AstLowerrer;

use super::*;

pub fn lower_type(db: &dyn AstLowerrer, expr: AstParam<Expr>) -> HirType {
    let kind = match expr.data {
        Expr::SelfExpr(_) => HirTypeKind::This,
        Expr::Unit(_) => HirTypeKind::Unit,
        Expr::Error => HirTypeKind::Error,

        // unsupported types yet
        Expr::Infix(_) => raise_type_expr_error(db, &expr),
        Expr::Array(_) => raise_type_expr_error(db, &expr),
        Expr::Dsl(_) => raise_type_expr_error(db, &expr),
        Expr::Lam(_) => raise_type_expr_error(db, &expr),
        Expr::Let(_) => raise_type_expr_error(db, &expr),
        Expr::If(_) => raise_type_expr_error(db, &expr),
        Expr::Match(_) => raise_type_expr_error(db, &expr),
        Expr::Ann(_) => raise_type_expr_error(db, &expr),
        Expr::Qual(_) => raise_type_expr_error(db, &expr),
        Expr::Sigma(_) => raise_type_expr_error(db, &expr),
        Expr::Help(_) => raise_type_expr_error(db, &expr),
        Expr::LiteralExpr(_) => raise_type_literal_error(db, &expr),

        //
        Expr::Group(ref group) => return db.hir_type(group.value().into()),
        Expr::Pi(ref pi) => {
            let lhs = db.hir_type(pi.parameter_type().into());
            let rhs = db.hir_type(pi.return_type().into());
            let parameter = match pi.parameter_name() {
                Some(name) => {
                    let name = db.intern_name(name.to_fn_id().to_string());

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
            let callee = db.hir_type(app.callee().into());
            let argument = db.hir_type(app.argument().into());

            HirTypeKind::from(HirTypeApp {
                callee: HirTypeFunction::Type(callee),
                arguments: vec![HirTypeArgument::Type(argument)],
            })
        }
        Expr::LocalExpr(ref local) => {
            let str = local.clone().to_fn_id().to_string();
            let name = db.intern_name(str.clone());
            let mut is_constructor = false;

            if_chain! {
                if let Some(c) = str.chars().next();
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

    db.intern_type(HirTypeData {
        kind,
        span: make_location(db, &expr),
    })
}

fn raise_type_literal_error(db: &dyn AstLowerrer, expr: &Expr) -> HirTypeKind {
    expr.clone().fail(UnsupportedTypeLiteralsError).push(db);

    HirTypeKind::Error
}

fn raise_type_expr_error(db: &dyn AstLowerrer, expr: &Expr) -> HirTypeKind {
    expr.clone().fail(UnsupportedTypeExprsError).push(db);

    HirTypeKind::Error
}
