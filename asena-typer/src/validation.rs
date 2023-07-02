use asena_ast::reporter::Reporter;
use asena_ast::*;
use asena_leaf::ast::{Lexeme, VirtualNode};

use crate::{Kind, Type, TypeError::*};

pub struct AsenaTypeValidator<'a> {
    pub is_constraint: bool,
    pub reporter: &'a mut Reporter,
}

impl AsenaVisitor<()> for AsenaTypeValidator<'_> {
    fn visit_accessor(&mut self, value: Accessor) {
        let mut accessor = value;

        loop {
            let rhs = accessor.rhs();
            match rhs {
                Expr::LocalExpr(_) => {}
                _ => {
                    self.reporter.report(&rhs, UnexpectedAccessorInTypeError);
                }
            }

            if let Expr::Accessor(rhs) = accessor.rhs() {
                accessor = rhs;
            } else {
                break;
            }
        }
    }

    fn visit_infix(&mut self, value: Infix) {
        self.reporter
            .report(&value, UnexpectedExprInTypeError(Infix::KIND))
    }

    fn visit_array(&mut self, value: Array) {
        self.reporter
            .report(&value, UnexpectedExprInTypeError(Array::KIND))
    }

    fn visit_dsl(&mut self, value: Dsl) {
        self.reporter
            .report(&value, UnexpectedExprInTypeError(Dsl::KIND))
    }

    fn visit_lam(&mut self, value: Lam) {
        self.reporter
            .report(&value, UnexpectedExprInTypeError(Lam::KIND))
    }

    fn visit_let(&mut self, value: Let) {
        self.reporter
            .report(&value, UnexpectedExprInTypeError(Let::KIND))
    }

    fn visit_ann(&mut self, value: Ann) {
        self.reporter
            .report(&value, UnexpectedExprInTypeError(Ann::KIND))
    }

    fn visit_qual(&mut self, value: Qual) {
        self.reporter
            .report(&value, UnsupportedQualifiersInTypeError)
    }

    fn visit_pi(&mut self, value: Pi) {
        if self.is_constraint {
            return;
        }

        self.reporter
            .report(&value, UnexpectedExprInTypeError(Pi::KIND))
    }

    fn visit_sigma(&mut self, value: Sigma) {
        self.reporter.report(&value, UnsupportedSigmaInTypeError)
    }

    fn visit_help(&mut self, value: Help) {
        self.reporter
            .report(&value, UnexpectedExprInTypeError(Help::KIND))
    }

    fn visit_literal(&mut self, value: Lexeme<Literal>) {
        let at = value.token.clone();
        self.reporter
            .diagnostic(at, UnexpectedTokenInTypeError(value.token.kind))
    }
}

pub struct AsenaConstraintValidator<'a> {
    pub reporter: &'a mut Reporter,
}

impl AsenaVisitor<()> for AsenaConstraintValidator<'_> {
    fn visit_ann(&mut self, value: Ann) {
        let name = value.lhs();
        match name {
            Expr::LocalExpr(_) => {}
            _ => {
                self.reporter.report(&name, ExpectedConstraintNameError);
            }
        }
    }

    fn visit_pi(&mut self, value: Pi) {
        if !is_constraint_type(value.parameter_type()) {
            self.reporter
                .report(&value.parameter_type(), ExpectedConstraintError)
        }

        if !is_constraint_type(value.return_type()) {
            self.reporter
                .report(&value.return_type(), ExpectedConstraintError)
        }
    }

    fn visit_accessor(&mut self, value: Accessor) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Accessor::KIND))
    }

    fn visit_infix(&mut self, value: Infix) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Infix::KIND))
    }

    fn visit_array(&mut self, value: Array) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Array::KIND))
    }

    fn visit_dsl(&mut self, value: Dsl) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Dsl::KIND))
    }

    fn visit_lam(&mut self, value: Lam) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Lam::KIND))
    }

    fn visit_let(&mut self, value: Let) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Let::KIND))
    }

    fn visit_qual(&mut self, value: Qual) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Qual::KIND))
    }

    fn visit_sigma(&mut self, value: Sigma) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Sigma::KIND))
    }

    fn visit_help(&mut self, value: Help) {
        self.reporter
            .report(&value, UnexpectedInConstraintError(Help::KIND))
    }

    fn visit_literal(&mut self, value: Lexeme<Literal>) {
        let at = value.token.clone();
        self.reporter
            .diagnostic(at, UnexpectedTokenInTypeError(value.token.kind))
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
            Expr::LocalExpr(local) if is_type_constructor(&local) => {
                Type::Constructor(local.name().to_fn_id(), Kind::Star)
            }
            Expr::LocalExpr(local) => Type::Variable(local.name().to_fn_id(), Kind::Star),
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

fn is_type_constructor(local: &LocalExpr) -> bool {
    let name = local.name();
    let str = name.as_str();
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
        Expr::LocalExpr(local) if local.name().as_str() == "Set" => true,
        _ => false,
    }
}
