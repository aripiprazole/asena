use std::sync::{Arc, Mutex, MutexGuard, Weak};

use asena_ast::reporter::Reporter;
use asena_ast::*;
use asena_hir::database::HirBag;
use asena_hir::expr::data::HirBranch;
use asena_hir::expr::{data::HirCallee, *};
use asena_hir::file::InternalAsenaFile;
use asena_hir::query::leaf::HirLoc;
use asena_hir::top_level::data::{HirDeclaration, HirSignature};
use asena_hir::top_level::{HirBindingGroup, HirTopLevel, HirTopLevelKind};
use asena_hir::value::*;
use asena_hir::{literal::*, NameId};
use asena_leaf::ast::{Located, Node};
use error::AstLoweringError::*;
use expr::ExprLowering;
use im::{hashset, HashMap, HashSet};
use itertools::Itertools;

pub mod decl;
pub mod error;
pub mod expr;
pub mod literal;
pub mod pattern;
pub mod stmt;
pub mod types;

type Signatures = HashMap<NameId, (HirLoc, HirBindingGroup)>;

pub struct AstLowering<'a, DB> {
    jar: Arc<DB>,
    file: Arc<InternalAsenaFile>,
    me: Weak<AstLowering<'a, DB>>,

    /// The reporter for this lowering.
    pub reporter: Arc<Mutex<Reporter>>,
    pub phantom: std::marker::PhantomData<&'a ()>,
}

impl<'ctx, DB: HirBag + 'static> AstLowering<'ctx, DB> {
    pub fn new(
        reporter: Arc<Mutex<Reporter>>,
        file: Arc<InternalAsenaFile>,
        jar: Arc<DB>,
    ) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            jar,
            file,
            reporter,
            me: me.clone(),
            phantom: std::marker::PhantomData,
        })
    }

    pub fn make_hir(&self) {
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
                Decl::Assign(ref decl) => self.make_assign(&mut signatures, decl),
                Decl::Signature(ref decl) => self.make_signature(&mut signatures, decl),
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
            };
        }

        for (span, group) in signatures.values().cloned() {
            let kind = HirTopLevelKind::from(group);
            let top_level = HirTopLevel::new(self.jar(), kind, vec![], vec![], span);

            declarations.insert(top_level);
        }

        *self.file.declarations.write().unwrap() = declarations;
    }

    fn make_signature(&self, signatures: &mut Signatures, decl: &Signature) {
        let name = decl.name().to_fn_id();
        let name = NameId::intern(self.jar(), name.as_str());
        let span = self.make_location(decl);

        if let Some((loc, _)) = signatures.get(&name) {
            self.reporter()
                .report(loc, DuplicatedSignatureDefinitionError);
        }

        let parameters = self.compute_parameters(decl);
        let declarations = match decl.body() {
            Some(body) => {
                let patterns = self.build_patterns(parameters.clone());

                hashset![HirDeclaration {
                    patterns,
                    value: self.lower_block(body),
                }]
            }
            None => hashset![],
        };
        let return_type = match decl.return_type() {
            Typed::Infer => None,
            Typed::Explicit(type_expr) => Some(self.lower_type(type_expr)),
        };

        let group = HirBindingGroup {
            signature: HirSignature {
                name,
                parameters,
                return_type,
            },
            declarations,
        };

        signatures.insert(name, (span, group));
    }

    fn make_assign(&self, signatures: &mut Signatures, decl: &Assign) {
        let name = decl.name().to_fn_id();
        let name = NameId::intern(self.jar(), name.as_str());
        let span = self.make_location(decl);

        let patterns = decl
            .patterns()
            .iter()
            .cloned()
            .map(|next| self.lower_pattern(next))
            .collect_vec();

        let (_, group) = signatures
            .entry(name)
            .or_insert_with(|| (span, Self::new_default_group(name)));

        group.declarations.insert(HirDeclaration {
            patterns,
            value: self.lower_value(decl.body()),
        });
    }

    fn new_default_group(name: NameId) -> HirBindingGroup {
        HirBindingGroup {
            signature: HirSignature {
                name,
                parameters: vec![],
                return_type: None,
            },
            declarations: hashset![],
        }
    }

    pub fn make_location(&self, node: &impl Located) -> HirLoc {
        let span = node.location().into_owned();
        let file = self.file.clone();

        HirLoc::new(file, span)
    }

    pub fn lower_value(&self, value: Expr) -> HirValueId {
        let location = self.make_location(&value);
        let mut lowering = ExprLowering::new(self.me.clone(), self.jar());
        let value = HirValueBlock {
            value: {
                let location = self.make_location(&value);
                let id = lowering.make(value);
                let kind = HirValueExpr(id);

                HirValue::new(self.jar(), kind.into(), location)
            },
            instructions: lowering.instructions,
        };

        HirValue::new(self.jar(), value.into(), location)
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

    pub fn reporter(&self) -> MutexGuard<Reporter> {
        self.reporter.lock().unwrap()
    }

    pub fn jar(&self) -> Arc<DB> {
        self.jar.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use asena_ast_db::{
        driver::Driver, implementation::AstDatabaseImpl, package::Package, vfs::VfsFile,
    };
    use asena_ast_resolver::decl::AstResolver;
    use asena_grammar::parse_asena_file;
    use asena_hir::{file::InternalAsenaFile, hir_dbg, query::HirDatabase};
    use asena_prec::{default_prec_table, InfixHandler, PrecReorder};

    use super::AstLowering;

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();
        let mut tree = parse_asena_file!("../Test.ase");

        let db = Driver(Arc::new(AstDatabaseImpl::default()));
        let local_pkg = Package::new(&db, "Local", "0.0.0", Arc::new(Default::default()));
        let file = VfsFile::new(&db, "Test", "./Test.ase".into(), local_pkg);

        let global_scope = db.global_scope();

        let internal_file = Arc::new(InternalAsenaFile {
            path: file.id.clone(),
            content: file.vfs().read_file(&file.id.path).unwrap(),
            tree: tree.clone().into(),
            declarations: Default::default(),
        });

        global_scope.borrow_mut().import(&db, file.clone(), None);

        tree.reporting(|reporter| {
            db.abstract_syntax_tree(file.clone())
                .walk_on(InfixHandler {
                    prec_table: &mut prec_table,
                    reporter,
                })
                .walk_on(PrecReorder {
                    prec_table: &prec_table,
                    reporter,
                })
                .walk_on(AstResolver::new(db, file, reporter))
        });

        let reporter = Arc::new(Mutex::new(tree.reporter));
        let jar = Arc::new(HirDatabase::default());
        let lowerrer = AstLowering::new(reporter.clone(), internal_file.clone(), jar.clone());
        lowerrer.make_hir();
        reporter.lock().unwrap().dump();

        println!("{:#?}", hir_dbg!(jar, internal_file));
    }
}
