use asena_ast::{DefaultMethod, Field, GlobalName, Trait, Typed};
use asena_hir::database::HirBag;
use asena_hir::top_level::HirTopLevelId;
use asena_hir::top_level::{
    data::{HirDeclaration, HirSignature},
    HirBindingGroup, HirTopLevel, HirTopLevelTrait,
};
use asena_hir::NameId;
use im::{hashset, HashMap};

use crate::error::AstLoweringError::*;
use crate::AstLowering;

type Methods = HashMap<NameId, HirBindingGroup>;

impl<DB: HirBag + 'static> AstLowering<'_, DB> {
    pub fn lower_trait(&self, trait_decl: Trait) -> HirTopLevelId {
        let location = self.make_location(&trait_decl);
        let name = NameId::intern(self.jar(), trait_decl.name().to_fn_id().as_str());

        let methods = self.compute_abstract_fields(trait_decl.fields());
        let kind = HirTopLevelTrait {
            signature: HirSignature {
                name,
                parameters: self.compute_parameters(&trait_decl),
                return_type: None,
            },
            groups: self.defaults(methods, trait_decl.default_methods()),
        };

        HirTopLevel::new(self.jar(), kind.into(), vec![], vec![], location)
    }

    fn compute_abstract_fields(&self, fields: Vec<Field>) -> Methods {
        let mut methods = HashMap::new();
        for field in fields {
            let name = NameId::intern(self.jar(), field.name().to_fn_id().as_str());
            if methods.get(&name).is_some() {
                self.reporter()
                    .report(&field, DuplicatedAbstractFieldDefinitionError)
            }
            let return_type = match field.field_type() {
                Typed::Infer => None,
                Typed::Explicit(type_expr) => Some(self.lower_type(type_expr)),
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

    fn defaults(&self, mut methods: Methods, default_methods: Vec<DefaultMethod>) -> Methods {
        for method in default_methods {
            let name = NameId::intern(self.jar(), method.name().to_fn_id().as_str());
            let parameters = self.compute_parameters(&method);
            let group = methods.entry(name).or_insert(HirBindingGroup {
                signature: HirSignature {
                    name,
                    parameters: parameters.clone(),
                    return_type: None,
                },
                declarations: hashset![],
            });

            group.declarations.insert(HirDeclaration {
                patterns: self.build_patterns(parameters),
                value: self.lower_block(method.body()),
            });
        }

        methods
    }
}
