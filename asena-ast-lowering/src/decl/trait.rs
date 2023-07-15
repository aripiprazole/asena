use asena_ast::{DefaultMethod, Field, GlobalName, Trait, Typed};
use asena_ast_db::package::HasDiagnostic;
use asena_hir::top_level::{
    data::{HirDeclaration, HirSignature},
    HirBindingGroup, HirTopLevel, HirTopLevelData, HirTopLevelTrait,
};
use asena_hir::Name;
use asena_leaf::ast::AstParam;
use asena_report::WithError;
use im::{hashset, HashMap};

use crate::make_location;
use crate::pattern::build_patterns;
use crate::{db::AstLowerrer, error::AstLoweringError::*};

use super::compute_parameters;

type Methods = HashMap<Name, HirBindingGroup>;

pub fn lower_trait(db: &dyn AstLowerrer, decl: AstParam<Trait>) -> HirTopLevel {
    let span = make_location(db, &decl);
    let name = db.intern_name(decl.name().to_fn_id().to_string());

    let methods = compute_abstract_fields(db, decl.fields());
    let kind = HirTopLevelTrait {
        signature: HirSignature {
            name,
            parameters: compute_parameters(db, &decl.data),
            return_type: None,
        },
        groups: defaults(db, methods, decl.default_methods()),
    };

    db.intern_top_level(HirTopLevelData {
        kind: kind.into(),
        attributes: vec![],
        docs: vec![],
        span,
    })
}

fn compute_abstract_fields(db: &dyn AstLowerrer, fields: Vec<Field>) -> Methods {
    let mut methods = HashMap::new();

    for field in fields {
        let name = db.intern_name(field.name().to_fn_id().to_string());
        if methods.get(&name).is_some() {
            field
                .clone()
                .fail(DuplicatedAbstractFieldDefinitionError)
                .push(db);
        }

        let return_type = match field.field_type() {
            Typed::Infer => None,
            Typed::Explicit(type_expr) => Some(db.hir_type(type_expr.into())),
        };

        let method = HirBindingGroup {
            signature: HirSignature {
                name,
                parameters: vec![],
                return_type,
            },
            declarations: hashset![],
        };

        methods.insert(name, method);
    }

    methods
}

fn defaults(db: &dyn AstLowerrer, mut methods: Methods, defaults: Vec<DefaultMethod>) -> Methods {
    for method in defaults {
        let name = db.intern_name(method.name().to_fn_id().to_string());
        let parameters = compute_parameters(db, &method);
        let group = methods.entry(name).or_insert(HirBindingGroup {
            signature: HirSignature {
                name,
                parameters: parameters.clone(),
                return_type: None,
            },
            declarations: hashset![],
        });

        group.declarations.insert(HirDeclaration {
            patterns: build_patterns(db, parameters),
            value: db.hir_block(method.body().into()),
        });
    }

    methods
}
