use asena_derive::*;

use asena_ast::command::*;
use asena_ast::walker::Reporter;
use asena_ast::*;
use asena_leaf::ast::{Lexeme, VirtualNode};

use crate::{Kind, Type, TypeError};

#[derive(Reporter)]
#[ast_step(
    BranchWalker,
    CommandWalker,
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker,
    FileWalker
)]
pub struct AsenaTypeValidator<'a, R: Reporter> {
    pub is_constraint: bool,
    #[ast_reporter]
    pub reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaTypeValidator<'a, R> {
    fn walk_expr_accessor(&mut self, value: &Accessor) {
        let mut accessor = value.clone();

        loop {
            let rhs = accessor.rhs();
            match rhs {
                Expr::Local(_) => {}
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
        if self.is_constraint {
            return;
        }

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

#[derive(Reporter)]
#[ast_step(
    BranchWalker,
    CommandWalker,
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker,
    FileWalker
)]
pub struct AsenaConstraintValidator<'a, R: Reporter> {
    #[ast_reporter]
    pub reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaConstraintValidator<'a, R> {
    fn walk_expr_ann(&mut self, value: &Ann) {
        let name = value.lhs();
        match name {
            Expr::Local(_) => {}
            _ => {
                self.report(&name, TypeError::ExpectedConstraintName);
            }
        }
    }

    fn walk_expr_pi(&mut self, value: &Pi) {
        if !is_constraint_type(value.parameter_type()) {
            self.report(&value.parameter_type(), TypeError::ExpectedConstraint)
        }

        if !is_constraint_type(value.return_type()) {
            self.report(&value.return_type(), TypeError::ExpectedConstraint)
        }
    }

    fn walk_expr_accessor(&mut self, value: &Accessor) {
        let kind = Accessor::tree_kind();
        self.report(value, TypeError::UnexpectedInConstraint(kind))
    }

    fn walk_expr_infix(&mut self, value: &Infix) {
        self.report(value, TypeError::UnexpectedInConstraint(Infix::tree_kind()))
    }

    fn walk_expr_array(&mut self, value: &Array) {
        self.report(value, TypeError::UnexpectedInConstraint(Array::tree_kind()))
    }

    fn walk_expr_dsl(&mut self, value: &Dsl) {
        self.report(value, TypeError::UnexpectedInConstraint(Dsl::tree_kind()))
    }

    fn walk_expr_lam(&mut self, value: &Lam) {
        self.report(value, TypeError::UnexpectedInConstraint(Lam::tree_kind()))
    }

    fn walk_expr_let(&mut self, value: &Let) {
        self.report(value, TypeError::UnexpectedInConstraint(Let::tree_kind()))
    }

    fn walk_expr_qual(&mut self, value: &Qual) {
        self.report(value, TypeError::UnexpectedInConstraint(Qual::tree_kind()))
    }

    fn walk_expr_sigma(&mut self, value: &Sigma) {
        self.report(value, TypeError::UnexpectedInConstraint(Sigma::tree_kind()))
    }

    fn walk_expr_help(&mut self, value: &Help) {
        self.report(value, TypeError::UnexpectedInConstraint(Help::tree_kind()))
    }

    fn walk_expr_literal(&mut self, value: &Lexeme<Literal>) {
        let at = value.token.clone();
        self.diagnostic(at, TypeError::UnexpectedTokenInType(value.token.kind))
    }
}

impl From<Typed> for Type {
    fn from(value: Typed) -> Self {
        match value {
            Typed::Infer => Self::Hole(None),
            Typed::Explicit(expr) => expr.into(),
        }
    }
}

impl From<Expr> for Type {
    fn from(value: Expr) -> Self {
        match value {
            Expr::Group(group) => group.value().into(),
            Expr::Local(local) if is_type_constructor(&local) => {
                Type::Constructor(local.to_fn_id(), Kind::Star)
            }
            Expr::Local(local) => Type::Variable(local.to_fn_id(), Kind::Star),
            Expr::App(app) => {
                let callee = Type::from(app.callee());
                let argument = Type::from(app.argument());

                Type::App(callee.into(), argument.into())
            }
            Expr::Pi(pi) => {
                let parameter_type = Type::from(pi.parameter_type());
                let return_type = Type::from(pi.return_type());

                Type::Arrow(parameter_type.into(), return_type.into())
            }
            _ => Type::Error,
        }
    }
}

fn is_type_constructor(local: &Lexeme<Local>) -> bool {
    let str = local.as_str();
    if let Some(x) = str.chars().next() {
        x.is_uppercase()
    } else {
        false
    }
}

fn is_constraint_type(expr: Expr) -> bool {
    match expr {
        Expr::Pi(pi) => {
            is_constraint_type(pi.parameter_type()) && is_constraint_type(pi.return_type())
        }
        Expr::Local(local) if local.as_str() == "Set" => true,
        _ => false,
    }
}
