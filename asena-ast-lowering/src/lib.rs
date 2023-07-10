use std::sync::{Arc, Weak};

use asena_ast::{AsenaFile, Binary, Decl, Expr, GlobalName, Infix, Literal, Signed, Typed};
use asena_hir::database::HirBag;
use asena_hir::expr::{data::HirCallee, *};
use asena_hir::hir_type::HirTypeId;
use asena_hir::top_level::data::HirSignature;
use asena_hir::top_level::{HirBindingGroup, HirTopLevel, HirTopLevelId, HirTopLevelKind};
use asena_hir::value::*;
use asena_hir::{literal::*, NameId};
use asena_leaf::ast::Located;
use expr::ExprLowering;
use im::{hashmap, hashset, HashMap, HashSet};

pub mod decl;
pub mod expr;
pub mod types;

pub struct AstLowering<DB> {
    jar: Arc<DB>,
    me: Weak<AstLowering<DB>>,
}

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn new(jar: Arc<DB>) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            jar,
            me: me.clone(),
        })
    }

    pub fn lower_file(&self, file: AsenaFile) -> HashSet<HirTopLevelId> {
        let mut declarations = HashSet::new();
        let mut signatures = HashMap::new();

        for decl in file.declarations() {
            match decl {
                Decl::Error => {}
                Decl::Use(_) => {}
                Decl::Command(_) => todo!("lower command: transform into events"),
                Decl::Class(class_decl) => {
                    declarations.insert(self.lower_class(class_decl));
                }
                Decl::Instance(instance_decl) => {
                    declarations.insert(self.lower_instance(instance_decl));
                }
                Decl::Trait(trait_decl) => {
                    declarations.insert(self.lower_trait(trait_decl));
                }
                Decl::Enum(enum_decl) => {
                    declarations.insert(self.lower_enum(enum_decl));
                }
                Decl::Assign(assign) => {
                    let name = assign.name().to_fn_id();
                    let name = NameId::intern(self.jar.clone(), name.as_str());
                    let span = assign.location().into_owned();

                    let (_, group) = signatures.entry(name).or_insert((
                        span,
                        HirBindingGroup {
                            signature: HirSignature {
                                name,
                                parameters: hashmap![],
                                return_type: None,
                            },
                            declarations: hashset![],
                        },
                    ));

                    group.declarations.insert(todo!("block"));
                }
                Decl::Signature(signature) => {
                    let name = signature.name().to_fn_id();
                    let name = NameId::intern(self.jar.clone(), name.as_str());
                    let span = signature.location().into_owned();

                    if let Some(_existing) = signatures.get(&name) {
                        // TODO: handle error
                    }

                    let group = HirBindingGroup {
                        signature: HirSignature {
                            name,
                            parameters: self.compute_parameters(&signature),
                            return_type: match signature.return_type() {
                                Typed::Infer => None,
                                Typed::Explicit(type_expr) => Some(self.lower_type(type_expr)),
                            },
                        },
                        declarations: hashset![],
                    };

                    signatures.insert(name, (span, group));
                }
            };
        }

        for (span, group) in signatures.values().cloned() {
            let kind = HirTopLevelKind::from(group);
            let top_level = HirTopLevel::new(self.jar.clone(), kind, vec![], vec![], span);

            declarations.insert(top_level);
        }

        declarations
    }

    pub fn lower_value(&self, value: Expr) -> HirValueId {
        let location = value.location().into_owned();
        let mut lowering = ExprLowering::new(self.me.clone(), self.jar.clone());
        let value = HirValueBlock {
            value: {
                let id = lowering.make(value);
                let kind = HirValueExpr(id);

                HirValue::new(self.jar.clone(), kind.into(), location.clone())
            },
            instructions: lowering.instructions,
        };

        HirValue::new(self.jar.clone(), value.into(), location)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asena_ast::Expr;
    use asena_grammar::asena_expr;
    use asena_hir::{database::HirBag, hir_dbg, query::HirDatabase};
    use asena_leaf::ast::Node;

    #[test]
    fn it_works() {
        let db = Arc::new(HirDatabase::default());
        let ast_lowering = super::AstLowering::new(db.clone());

        let expr = asena_expr! { 1 + 1 };
        let id = ast_lowering.lower_value(Expr::new(expr));
        let value = db.clone().value_data(id);

        println!("{:#?}", hir_dbg!(db, value));
    }
}
