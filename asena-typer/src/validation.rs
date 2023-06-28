use asena_derive::*;

use asena_ast::command::*;
use asena_ast::walker::Reporter;
use asena_ast::*;
use asena_leaf::ast::{Lexeme, Virtual};

use crate::TypeError;

#[derive(Reporter)]
#[ast_step(
    CommandWalker,
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker,
    FileWalker
)]
pub struct AsenaTypeValidator<'a, R: Reporter> {
    #[ast_reporter]
    pub reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaTypeValidator<'a, R> {
    fn walk_expr_accessor(&mut self, value: &Accessor) {
        let mut accessor = value.clone();

        loop {
            let rhs = accessor.rhs();
            match rhs {
                Expr::Local(_) | Expr::QualifiedPath(_) => {}
                _ => {
                    self.report(&rhs, TypeError::UnexpectedAccessorInType);
                }
            }

            if let Expr::Accessor(rhs) = accessor.rhs() {
                accessor = rhs;
            } else {
                break;
            }
        }
    }

    fn walk_expr_infix(&mut self, value: &Infix) {
        self.report(value, TypeError::UnexpectedExprInType(Infix::tree_kind()))
    }

    fn walk_expr_array(&mut self, value: &Array) {
        self.report(value, TypeError::UnexpectedExprInType(Array::tree_kind()))
    }

    fn walk_expr_dsl(&mut self, value: &Dsl) {
        self.report(value, TypeError::UnexpectedExprInType(Dsl::tree_kind()))
    }

    fn walk_expr_lam(&mut self, value: &Lam) {
        self.report(value, TypeError::UnexpectedExprInType(Lam::tree_kind()))
    }

    fn walk_expr_let(&mut self, value: &Let) {
        self.report(value, TypeError::UnexpectedExprInType(Let::tree_kind()))
    }

    fn walk_expr_ann(&mut self, value: &Ann) {
        self.report(value, TypeError::UnexpectedExprInType(Ann::tree_kind()))
    }

    fn walk_expr_qual(&mut self, value: &Qual) {
        self.report(value, TypeError::UnsupportedQualifiersInType)
    }

    fn walk_expr_pi(&mut self, value: &Pi) {
        self.report(value, TypeError::UnexpectedExprInType(Pi::tree_kind()))
    }

    fn walk_expr_sigma(&mut self, value: &Sigma) {
        self.report(value, TypeError::UnsupportedSigmaInType)
    }

    fn walk_expr_help(&mut self, value: &Help) {
        self.report(value, TypeError::UnexpectedExprInType(Help::tree_kind()))
    }

    fn walk_expr_literal(&mut self, value: &Lexeme<Literal>) {
        let at = value.token.clone();
        self.diagnostic(at, TypeError::UnexpectedTokenInType(value.token.kind))
    }
}
