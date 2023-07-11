use std::sync::{Arc, Weak};

use asena_ast::{AsenaFile, Binary, Branch, Decl, Expr, GlobalName, Infix, Literal, Signed, Typed};
use asena_hir::database::HirBag;
use asena_hir::expr::data::HirBranch;
use asena_hir::expr::{data::HirCallee, *};
use asena_hir::file::InternalAsenaFile;
use asena_hir::hir_type::HirTypeId;
use asena_hir::query::leaf::HirLoc;
use asena_hir::top_level::data::{HirDeclaration, HirSignature};
use asena_hir::top_level::{HirBindingGroup, HirTopLevel, HirTopLevelKind};
use asena_hir::value::*;
use asena_hir::{literal::*, NameId};
use asena_leaf::ast::{Located, Node};
use expr::ExprLowering;
use im::{hashset, HashMap, HashSet};
use itertools::Itertools;

pub mod decl;
pub mod expr;
pub mod literal;
pub mod pattern;
pub mod stmt;
pub mod types;

pub struct AstLowering<DB> {
    jar: Arc<DB>,
    file: Arc<InternalAsenaFile>,
    me: Weak<AstLowering<DB>>,
}

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn new(file: Arc<InternalAsenaFile>, jar: Arc<DB>) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            jar,
            file,
            me: me.clone(),
        })
    }

    pub fn run_lowering(&self) {
        let file = AsenaFile::new(self.file.tree.clone());

        let mut declarations = HashSet::new();
        let mut signatures = HashMap::new();

        for decl in file.declarations() {
            match decl {
                Decl::Error => {}
                Decl::Use(_) => {}
                Decl::Command(_) => {
                    // TODO: handle commands
                }
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
                    let span = self.make_location(&assign);

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
                    let span = self.make_location(&signature);

                    if let Some(_existing) = signatures.get(&name) {
                        // TODO: handle error
                    }

                    let parameters = self.compute_parameters(&signature);

                    let group = HirBindingGroup {
                        signature: HirSignature {
                            name,
                            parameters: parameters.clone(),
                            return_type: match signature.return_type() {
                                Typed::Infer => None,
                                Typed::Explicit(type_expr) => Some(self.lower_type(type_expr)),
                            },
                        },
                        declarations: match signature.body() {
                            Some(body) => {
                                let patterns = self.build_patterns(parameters);

                                hashset![HirDeclaration {
                                    patterns,
                                    value: self.lower_block(body),
                                }]
                            }
                            None => hashset![],
                        },
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

        *self.file.declarations.write().unwrap() = declarations;
    }

    pub fn make_location(&self, node: &impl Located) -> HirLoc {
        let span = node.location().into_owned();
        let file = self.file.clone();

        HirLoc::new(file, span)
    }

    pub fn lower_value(&self, value: Expr) -> HirValueId {
        let location = self.make_location(&value);
        let mut lowering = ExprLowering::new(self.me.clone(), self.jar.clone());
        let value = HirValueBlock {
            value: {
                let location = self.make_location(&value);
                let id = lowering.make(value);
                let kind = HirValueExpr(id);

                HirValue::new(self.jar.clone(), kind.into(), location)
            },
            instructions: lowering.instructions,
        };

        HirValue::new(self.jar.clone(), value.into(), location)
    }

    pub fn lower_branch(&self, branch: Branch) -> HirBranch {
        match branch {
            Branch::Error => HirBranch::Error,
            Branch::ExprBranch(ref branch) => {
                let value = self.lower_value(branch.value());

                HirBranch::Expr(value)
            }
            Branch::BlockBranch(ref branch) => HirBranch::Block(self.lower_block(branch.stmts())),
        }
    }

    pub fn jar(&self) -> Arc<DB> {
        self.jar.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asena_ast_db::{
        driver::Driver, implementation::AstDatabaseImpl, package::Package, vfs::VfsFile,
    };
    use asena_grammar::parse_asena_file;
    use asena_hir::{file::InternalAsenaFile, hir_dbg, query::HirDatabase};

    use super::AstLowering;

    #[test]
    fn it_works() {
        let tree = parse_asena_file!("../Test.ase");

        let db = Driver(Arc::new(AstDatabaseImpl::default()));
        let local_pkg = Package::new(&db, "Local", "0.0.0", Arc::new(Default::default()));
        let file = VfsFile::new(&db, "Test", "./Test.ase".into(), local_pkg);

        let internal_file = Arc::new(InternalAsenaFile {
            path: file.id.clone(),
            content: file.vfs().read_file(&file.id.path).unwrap(),
            tree: tree.into(),
            declarations: Default::default(),
        });

        let jar = Arc::new(HirDatabase::default());
        let ast_lowering = AstLowering::new(internal_file.clone(), jar.clone());
        ast_lowering.run_lowering();

        println!("{:#?}", hir_dbg!(jar, internal_file));
    }
}
