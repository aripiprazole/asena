use asena_derive::*;

use asena_ast::walker::Reporter;
use asena_ast::*;
use asena_leaf::ast::Walkable;
use itertools::Itertools;

use crate::validation::{AsenaConstraintValidator, AsenaTypeValidator};

#[derive(Default)]
pub struct ClassEnvironment {}

#[derive(Default)]
pub struct TypeEnvironment {}

#[derive(Reporter)]
#[ast_step(
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker,
    FileWalker
)]
pub struct AsenaTyper<'a, R: Reporter> {
    pub type_env: &'a mut TypeEnvironment,
    pub class_env: &'a mut ClassEnvironment,

    #[ast_reporter]
    pub reporter: &'a mut R,
}

impl<'a, R: Reporter> DeclWalker for AsenaTyper<'a, R> {
    fn walk_decl_signature(&mut self, value: &Signature) {
        // Check the return type of the signature.
        let return_type = value.return_type().walks(AsenaTypeValidator {
            is_constraint: false,
            reporter: self.reporter,
        });

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

        let explicit_parameters = parameters.iter().filter(|p| p.explicit()).collect_vec();
        let implicit_parameters = parameters.iter().filter(|p| !p.explicit()).collect_vec();
    }
}

impl<'a, R: Reporter> ExprWalker for AsenaTyper<'a, R> {}

impl<'a, R: Reporter> AsenaTyper<'a, R> {
    pub fn infer(&mut self, expr: Expr) {
        match expr {
            Expr::Error => todo!(),
            Expr::QualifiedPath(_) => todo!(),
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
            Expr::Local(_) => todo!(),
            Expr::Literal(_) => todo!(),
        }
    }
}
