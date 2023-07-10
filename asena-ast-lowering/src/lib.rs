use std::sync::{Arc, Weak};

use asena_ast::{AsenaFile, Binary, Decl, Expr, GlobalName, Infix, Literal, Signed, Typed};
use asena_hir::database::HirBag;
use asena_hir::expr::{data::HirCallee, *};
use asena_hir::hir_type::HirTypeId;
use asena_hir::top_level::data::{HirDeclaration, HirSignature};
use asena_hir::top_level::{HirBindingGroup, HirTopLevel, HirTopLevelId, HirTopLevelKind};
use asena_hir::value::*;
use asena_hir::{literal::*, NameId};
use asena_leaf::ast::Located;
use expr::ExprLowering;
use im::{hashset, HashMap, HashSet};
use itertools::Itertools;

pub mod decl;
pub mod expr;
pub mod pattern;
pub mod stmt;
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

                    let patterns = assign
                        .patterns()
                        .iter()
                        .cloned()
                        .map(|next| self.lower_pattern(next))
                        .collect_vec();

                    let (_, group) = signatures.entry(name).or_insert((
                        span,
                        HirBindingGroup {
                            signature: HirSignature {
                                name,
                                parameters: vec![],
                                return_type: None,
                            },
                            declarations: hashset![],
                        },
                    ));

                    group.declarations.insert(HirDeclaration {
                        patterns,
                        value: self.lower_value(assign.body()),
                    });
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

    pub fn make_literal(&self, literal: Literal) -> HirLiteral {
        match literal {
            Literal::Error => HirLiteral::Error,
            Literal::True => HirLiteral::Int(1, HirISize::U1, HirIntSign::Unsigned),
            Literal::False => HirLiteral::Int(0, HirISize::U1, HirIntSign::Unsigned),
            Literal::String(value) => HirLiteral::String(HirString { value, name: None }),
            Literal::Nat(_) => todo!("lowering nat literals is not yet implemented"),
            Literal::Int8(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U8, HirIntSign::Signed)
            }
            Literal::Int8(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U8, HirIntSign::Unsigned)
            }
            Literal::Int16(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U16, HirIntSign::Signed)
            }
            Literal::Int16(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U16, HirIntSign::Unsigned)
            }
            Literal::Int32(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U32, HirIntSign::Signed)
            }
            Literal::Int32(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U32, HirIntSign::Unsigned)
            }
            Literal::Int64(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U64, HirIntSign::Signed)
            }
            Literal::Int64(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U64, HirIntSign::Unsigned)
            }
            Literal::Int128(value, Signed::Signed) => {
                HirLiteral::Int(value as _, HirISize::U128, HirIntSign::Signed)
            }
            Literal::Int128(value, Signed::Unsigned) => {
                HirLiteral::Int(value as _, HirISize::U128, HirIntSign::Unsigned)
            }
            Literal::Float32(value) => {
                let s = value.clone().to_string();

                let mut split = s.split('.');
                let integer = split.next().unwrap().parse::<usize>().unwrap();
                let decimal = split.next().unwrap_or("0").parse::<usize>().unwrap();

                HirLiteral::Decimal(HirFSize::F64, HirDecimal { integer, decimal })
            }
            Literal::Float64(value) => {
                let s = value.clone().to_string();

                let mut split = s.split('.');
                let integer = split.next().unwrap().parse::<usize>().unwrap();
                let decimal = split.next().unwrap_or("0").parse::<usize>().unwrap();

                HirLiteral::Decimal(HirFSize::F64, HirDecimal { integer, decimal })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asena_ast::AsenaFile;
    use asena_grammar::parse_asena_file;
    use asena_hir::{hir_dbg, query::HirDatabase};
    use asena_leaf::ast::Node;

    #[test]
    fn it_works() {
        let db = Arc::new(HirDatabase::default());
        let ast_lowering = super::AstLowering::new(db.clone());

        let tree = parse_asena_file!("../Test.ase");
        let data = ast_lowering.lower_file(AsenaFile::new(tree));

        println!("{:#?}", hir_dbg!(db, data));
    }
}
