use std::{cell::RefCell, rc::Rc, sync::Arc};

use asena_ast::{
    reporter::Reporter, AsenaListener, AsenaVisitor, BindingId, Expr, FunctionId, GlobalName,
};
use asena_ast_db::{
    driver::Driver,
    scope::{ScopeData, VariantResolution},
    vfs::*,
};
use asena_report::InternalError;
use im::HashMap;
use thiserror::Error;

use crate::ResolutionError::*;

pub struct AstResolver<'a> {
    pub db: Driver,
    pub curr_vf: Arc<VfsFile>,
    pub canonical_paths: HashMap<FunctionId, VfsPath>,
    pub reporter: &'a mut Reporter,
}

pub struct ScopeResolver<'gctx> {
    pub db: Driver,
    pub frames: Vec<Rc<RefCell<ScopeData>>>,
    pub resolver: &'gctx mut AstResolver<'gctx>,
    pub file: &'gctx ScopeData,
    pub reporter: &'gctx mut Reporter,
}

#[derive(Default, Error, Debug, Clone, PartialEq, Eq)]
pub enum ResolutionError {
    #[error("Not resolved")]
    #[default]
    NotResolved,

    #[error("Unresolved import: {0}")]
    UnresolvedImportError(FunctionId),

    #[error("Could not find the declared name {0}")]
    UnresolvedNameError(FunctionId),

    #[error("Could not find the type constructor {0}")]
    UnresolvedConstructorError(FunctionId),
}

pub enum Resolution {
    OkExpr(Arc<Expr>),
    OkBinding(Arc<BindingId>),
    Err(ResolutionError),
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::Err(NotResolved)
    }
}

impl ResolutionError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl InternalError for ResolutionError {
    fn code(&self) -> u16 {
        self.discriminant() as u16
    }

    fn kind(&self) -> asena_report::DiagnosticKind {
        asena_report::DiagnosticKind::Error
    }
}

impl ScopeResolver<'_> {
    pub fn last_scope(&mut self) -> Rc<RefCell<ScopeData>> {
        self.frames
            .last()
            .cloned()
            .unwrap_or_else(|| self.db.global_scope())
    }
}

impl AsenaListener for ScopeResolver<'_> {
    fn enter_pi(&mut self, _value: asena_ast::Pi) {
        let scope = self.last_scope().borrow().fork();
        self.frames.push(scope);
    }

    fn exit_pi(&mut self, _value: asena_ast::Pi) {
        self.frames.pop();
    }
}

impl AsenaVisitor<()> for AstResolver<'_> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.module_ref(value.to_fn_id().as_str());

        self.db.add_path_dep(self.curr_vf.clone(), module_ref);
    }

    fn visit_qualified_path(&mut self, value: asena_ast::QualifiedPath) {
        let path = value.to_fn_id();
        match self.db.function_data(path, self.curr_vf.clone()) {
            Some(_) => {}
            None => {
                let fn_id = value.to_fn_id();
                self.reporter.report(&value, UnresolvedNameError(fn_id));
            }
        }
    }

    fn visit_global_pat(&mut self, value: asena_ast::GlobalPat) {
        let path = value.name();

        // Global pattern finding logic
        match self.db.constructor_data(value.name(), self.curr_vf.clone()) {
            VariantResolution::Variant(_) => {}

            // if it is not a constructor, it is a variable binding: Vec.cons x xs
            //                                                                ^ ^^
            VariantResolution::Binding(_) => {}

            // if it is a constructor and it is not found, report an error
            VariantResolution::None => {
                let fn_id = path.to_fn_id();
                self.reporter.report(&path, UnresolvedNameError(fn_id));
            }
        }
    }

    fn visit_constructor_pat(&mut self, value: asena_ast::ConstructorPat) {
        let name = value.name();

        match self.db.constructor_data(value.name(), self.curr_vf.clone()) {
            VariantResolution::Variant(_) | VariantResolution::Binding(_) => {}
            VariantResolution::None => {
                let fn_id = name.to_fn_id();
                self.reporter.report(&name, UnresolvedNameError(fn_id));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asena_ast_db::{driver::Driver, implementation::*, package::*, vfs::*};
    use asena_grammar::parse_asena_file;
    use asena_prec::{default_prec_table, InfixHandler, PrecReorder};

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();

        let mut db = Driver(Arc::new(NonResolvingAstDatabase::default()));
        let local_pkg = Package::new(&db, "Local", "0.0.0", Arc::new(Default::default()));
        let file = VfsFile::new(&db, "Test", "./Test.ase".into(), local_pkg);
        VfsFile::new(&db, "Nat", "./Nat.ase".into(), local_pkg);
        VfsFile::new(&db, "IO", "./IO.ase".into(), local_pkg);

        let mut asena_file = parse_asena_file!("../Test.ase");

        db.global_scope()
            .borrow_mut()
            .import(Arc::get_mut(&mut db).unwrap(), file.clone(), None);

        db.abstract_syntax_tree(file.clone())
            .arc_walks(InfixHandler {
                prec_table: &mut prec_table,
                reporter: &mut asena_file.reporter,
            })
            .arc_walks(PrecReorder {
                prec_table: &prec_table,
                reporter: &mut asena_file.reporter,
            })
            .arc_walks(super::AstResolver {
                db,
                curr_vf: file,
                canonical_paths: Default::default(),
                reporter: &mut asena_file.reporter,
            });

        asena_file.reporter.dump();
    }
}
