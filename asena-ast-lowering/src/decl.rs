use asena_ast::{traits::global_decl::GlobalDecl, GlobalName, Method, Parameter, Typed};
use asena_hir::{
    database::HirBag,
    top_level::{
        data::{HirParameterData, HirParameterKind, HirSignature},
        HirBindingGroup,
    },
    NameId,
};
use im::{hashset, HashMap};

use crate::AstLowering;

pub mod class_decl;
pub mod enum_decl;
pub mod instance_decl;
pub mod trait_decl;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn compute_parameters(&self, decl: &impl GlobalDecl) -> HashMap<NameId, HirParameterKind> {
        let mut parameters = HashMap::new();
        for (name, parameter) in Parameter::compute_parameters(decl.parameters()) {
            let name = NameId::intern(self.jar.clone(), name.as_str());
            let data = HirParameterData {
                name,
                parameter_type: match parameter.parameter_type() {
                    Typed::Infer => None,
                    Typed::Explicit(expr) => Some(self.lower_type(expr)),
                },
            };

            match true {
                _ if parameter.is_self() && parameter.explicit() => {
                    parameters.insert(name, HirParameterKind::This);
                }
                _ if parameter.is_self() && !parameter.explicit() => {
                    // TODO: handle error
                    // a self parameter cannot be implicit
                }
                // This is the inverse, for explicit being the default case, if the parameter is
                // with some error and explicit is buggy, then it will be explicit.
                _ if !parameter.explicit() => {
                    parameters.insert(name, HirParameterKind::Implicit(data));
                }
                _ => {
                    parameters.insert(name, HirParameterKind::Explicit(data));
                }
            }
        }
        parameters
    }

    pub fn compute_methods(&self, methods: Vec<Method>) -> im::HashSet<HirBindingGroup> {
        let mut groups = hashset![];
        for method in methods {
            let name = NameId::intern(self.jar.clone(), method.name().to_fn_id().as_str());
            let parameters = self.compute_parameters(&method);
            let return_type = match method.return_type() {
                Typed::Infer => None,
                Typed::Explicit(expr) => Some(self.lower_type(expr)),
            };

            let group = HirBindingGroup {
                signature: HirSignature {
                    name,
                    parameters,
                    return_type,
                },
                declarations: hashset![],
            };
            groups.insert(group);
        }
        groups
    }
}
