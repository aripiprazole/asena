use asena_ast::{DefaultMethod, Field, GlobalName, Trait, Typed};
use asena_hir::database::HirBag;
use asena_hir::top_level::HirTopLevelId;
use asena_hir::top_level::{
    data::{HirDeclaration, HirSignature},
    HirBindingGroup, HirTopLevel, HirTopLevelTrait,
};
use asena_hir::NameId;
use asena_leaf::ast::Located;
use im::{hashmap, hashset, HashMap};

use crate::AstLowering;

type Methods = HashMap<NameId, HirBindingGroup>;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_trait(&self, trait_decl: Trait) -> HirTopLevelId {
        let location = trait_decl.location().into_owned();
        let name = NameId::intern(self.jar.clone(), trait_decl.name().to_fn_id().as_str());

        let methods = self.compute_abstract_fields(trait_decl.fields());
        let kind = HirTopLevelTrait {
            signature: HirSignature {
                name,
                parameters: self.compute_parameters(&trait_decl),
                return_type: None,
            },
            groups: self.defaults(methods, trait_decl.default_methods()),
        };

        HirTopLevel::new(self.jar.clone(), kind.into(), vec![], vec![], location)
    }

    fn compute_abstract_fields(&self, fields: Vec<Field>) -> Methods {
        let mut methods = HashMap::new();
        for field in fields {
            let name = NameId::intern(self.jar.clone(), field.name().to_fn_id().as_str());
            if let Some(_existing) = methods.get(&name) {
                // TODO: handle error
            }
            let return_type = match field.field_type() {
                Typed::Infer => None,
                Typed::Explicit(type_expr) => Some(self.lower_type(type_expr)),
            };
            let method = HirBindingGroup {
                signature: HirSignature {
                    name,
                    parameters: hashmap![],
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
            let name = NameId::intern(self.jar.clone(), method.name().to_fn_id().as_str());
            let group = methods.entry(name).or_insert(HirBindingGroup {
                signature: HirSignature {
                    name,
                    parameters: self.compute_parameters(&method),
                    return_type: None,
                },
                declarations: hashset![],
            });

            // TODO: transforms the parameters into a list of patterns
            let patterns = vec![];

            group.declarations.insert(HirDeclaration {
                patterns,
                value: todo!("blocks"),
            });
        }

        methods
    }
}
