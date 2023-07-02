use asena_ast::reporter::Reporter;
use asena_ast::*;
use itertools::Itertools;

use crate::{validation::*, Scheme, Type};

#[derive(Default, Debug)]
pub struct ClassEnvironment {}

#[derive(Default, Debug)]
pub struct TypeEnvironment {
    pub schemes: im::HashMap<FunctionId, Scheme>,
}

pub struct Typer<'a> {
    pub type_env: &'a mut TypeEnvironment,
    pub class_env: &'a mut ClassEnvironment,
    pub reporter: &'a mut Reporter,
}

impl AsenaVisitor<()> for Typer<'_> {
    fn visit_signature(&mut self, value: Signature) {
        let name = value.name().to_fn_id();

        // Check the return type of the signature.
        let return_type = Type::from(value.return_type().walks(AsenaTypeValidator {
            is_constraint: false,
            reporter: self.reporter,
        }));

        let parameters = value
            .parameters()
            .into_iter()
            .map(|param| {
                if param.explicit() {
                    param.walks(AsenaTypeValidator {
                        is_constraint: false,
                        reporter: self.reporter,
                    })
                } else {
                    param.walks(AsenaConstraintValidator {
                        reporter: self.reporter,
                    })
                }
            })
            .collect_vec();

        let implicit_parameters = parameters
            .iter()
            .filter(|p| !p.explicit())
            .cloned()
            .filter_map(|x| match x.parameter_type() {
                Typed::Infer => None,
                Typed::Explicit(Expr::LocalExpr(name)) => Some(name.name().to_fn_id()),
                Typed::Explicit(_) => None,
            })
            .collect_vec();

        // FIXME: this transforms a -> b -> c in (a -> b) -> c
        let mono = parameters
            .iter()
            .filter(|p| p.explicit())
            .map(|param| Type::from(param.parameter_type()))
            .rev()
            .fold(return_type, |acc, next| {
                Type::Arrow(next.into(), acc.into())
            });

        self.type_env.schemes.insert(
            name,
            Scheme {
                variables: implicit_parameters,
                mono,
            },
        );
    }
}

impl<'a> Typer<'a> {
    ///
    /// Γ, σ ∈ Γ   τ = inst(σ)
    /// ────────────────────── [E-VAR]
    /// e : τ
    ///
    /// Γ, e : τ, S     S Γ e' ⊢ τ', S'
    /// τ''' = newvar   S'' = mgu(S' τ, τ' -> τ''')
    /// ─────────────────────────────────────────── [E-APP]
    /// Γ ⊢ (e e' : S'' τ'''), S'', S', S
    ///
    /// τ = newvar   Γ, x: τ ⊢ e : τ', S
    /// ──────────────────────────────── [E-ABS]
    /// Γ ⊢ (λx. e : τ -> τ'), S
    ///
    /// Γ ⊢ e : τ, S   S Γ x : gen(S Γ, τ) ⊢ e' : τ', S'
    /// ──────────────────────────────────────────────── [E-LET]
    /// Γ ⊢ (let x = e in e' : τ'), S', S
    ///
    pub fn infer(&mut self, expr: Expr) -> Type {
        match expr {
            Expr::Error => todo!(),
            Expr::Group(_) => todo!(),
            Expr::Infix(_) => todo!(),
            Expr::Accessor(_) => todo!(),
            Expr::App(_) => todo!(),
            Expr::Array(_) => todo!(),
            Expr::Dsl(_) => todo!(),
            Expr::Lam(_) => todo!(),
            Expr::Let(_) => todo!(),
            Expr::Ann(_) => todo!(),
            Expr::Qual(_) => todo!(),
            Expr::Pi(_) => todo!(),
            Expr::Sigma(_) => todo!(),
            Expr::Help(_) => todo!(),
            Expr::LocalExpr(_) => todo!(),
            Expr::LiteralExpr(_) => todo!(),
            Expr::Unit(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Match(_) => todo!(),
        }
    }
}
