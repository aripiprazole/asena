use asena_ast::{traits::global_decl::GlobalDecl, GlobalName, Method, Parameter, Typed};
use asena_ast_db::package::HasDiagnostic;
use asena_hir::top_level::{
    data::{HirDeclaration, HirParameterData, HirParameterKind, HirSignature},
    HirBindingGroup,
};
use asena_report::WithError;
use im::hashset;

use crate::{db::AstLowerrer, error::AstLoweringError::*};

pub mod class;
pub mod r#enum;
pub mod instance;
pub mod r#trait;

pub fn compute_parameters(db: &dyn AstLowerrer, decl: &impl GlobalDecl) -> Vec<HirParameterKind> {
    let mut parameters = Vec::new();
    for (name, parameter) in Parameter::compute_parameters(decl.parameters()) {
        let name = db.intern_name(name.to_string());
        let data = HirParameterData {
            name,
            parameter_type: match parameter.parameter_type() {
                Typed::Infer => None,
                Typed::Explicit(expr) => Some(db.hir_type(expr.into())),
            },
        };

        match true {
            _ if parameter.is_self() && parameter.explicit() => {
                parameters.push(HirParameterKind::This);
            }
            // a self parameter cannot be implicit
            _ if parameter.is_self() && !parameter.explicit() => {
                parameter.fail(SelfParameterBayMeExplicitError).push(db)
            }
            // This is the inverse, for explicit being the default case, if the parameter is
            // with some error and explicit is buggy, then it will be explicit.
            _ if !parameter.explicit() => {
                parameters.push(HirParameterKind::Implicit(data));
            }
            _ => {
                parameters.push(HirParameterKind::Explicit(data));
            }
        }
    }
    parameters
}

pub fn compute_methods(db: &dyn AstLowerrer, methods: Vec<Method>) -> im::HashSet<HirBindingGroup> {
    let mut groups = hashset![];
    for method in methods {
        let name = db.intern_name(method.name().to_fn_id().to_string());
        let parameters = compute_parameters(db, &method);
        let return_type = match method.return_type() {
            Typed::Infer => None,
            Typed::Explicit(expr) => Some(db.hir_type(expr.into())),
        };

        let group = HirBindingGroup {
            signature: HirSignature {
                name,
                parameters,
                return_type,
            },
            declarations: hashset![HirDeclaration {
                patterns: vec![],
                value: db.hir_block(method.body().into()),
            }],
        };
        groups.insert(group);
    }
    groups
}
