use std::{cell::RefCell, rc::Rc, sync::Arc};

use asena_ast::{reporter::Reporter, *};
use asena_ast_db::{
    driver::Driver,
    scope::{ScopeData, Value, VariantResolution},
    vfs::*,
};
use asena_leaf::ast::Listenable;
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

pub struct ScopeResolver<'gctx, 'a> {
    pub db: Driver,
    pub local_scope: Rc<RefCell<ScopeData>>,
    pub frames: Vec<Rc<RefCell<ScopeData>>>,
    pub resolver: &'gctx mut AstResolver<'a>,
}

#[derive(Default, Error, Debug, Clone, PartialEq, Eq)]
pub enum ResolutionError {
    #[error("Not resolved")]
    #[default]
    NotResolved,

    #[error("Unresolved import: `{0}`")]
    UnresolvedImportError(FunctionId),

    #[error("Could not find the declared name: `{0}`")]
    UnresolvedNameError(FunctionId),

    #[error("Could not find the type constructor: `{0}`")]
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

impl<'gctx, 'a> ScopeResolver<'gctx, 'a> {
    pub fn new(name: BindingId, resolver: &'gctx mut AstResolver<'a>) -> Self {
        let global_scope = resolver.db.global_scope();
        let local_scope = {
            let named_scope = global_scope.borrow().fork();
            let mut scope_mut = named_scope.borrow_mut();
            scope_mut.variables.insert(name.to_fn_id(), 0);
            named_scope.clone()
        };

        Self {
            db: resolver.db.clone(),
            local_scope: local_scope.clone(),
            frames: vec![local_scope],
            resolver,
        }
    }

    pub fn last_scope(&mut self) -> Rc<RefCell<ScopeData>> {
        self.frames
            .last()
            .cloned()
            .unwrap_or_else(|| self.db.global_scope())
    }
}

impl AsenaListener for ScopeResolver<'_, '_> {
    fn enter_pi(&mut self, pi: asena_ast::Pi) {
        let scope = self.last_scope().borrow().fork();
        if let Some(name) = pi.parameter_name() {
            let name = name.to_fn_id();
            let value = pi.parameter_type();
            let mut scope = scope.borrow_mut();
            scope.functions.insert(name, Value::Expr(Arc::new(value)));
        }

        self.frames.push(scope);
    }

    fn exit_pi(&mut self, _: asena_ast::Pi) {
        self.frames.pop();
    }

    fn enter_case(&mut self, _: Case) {
        let scope = self.last_scope().borrow().fork();
        self.frames.push(scope);
    }

    fn exit_case(&mut self, _: Case) {
        self.frames.pop();
    }

    fn enter_lam(&mut self, _: Lam) {
        let scope = self.last_scope().borrow().fork();
        self.frames.push(scope);
    }

    fn exit_lam(&mut self, _: Lam) {
        self.frames.pop();
    }

    fn enter_lam_parameter(&mut self, value: LamParameter) {
        let scope = self.last_scope();
        let mut scope = scope.borrow_mut();

        let value = Arc::new(value);
        scope
            .functions
            .insert(value.name().to_fn_id(), Value::LamParam(value));
    }

    fn enter_local_expr(&mut self, value: LocalExpr) {
        let scope = self.last_scope();
        let scope = scope.borrow();
        match scope.functions.get(&value.to_fn_id()) {
            Some(_) => {}
            None => {
                self.resolver
                    .reporter
                    .report(&value.segments(), UnresolvedNameError(value.to_fn_id()));
            }
        }
    }

    fn enter_qualified_path(&mut self, value: asena_ast::QualifiedPath) {
        self.resolver.visit_qualified_path(value);
    }

    fn enter_global_pat(&mut self, value: asena_ast::GlobalPat) {
        let name = value.name();
        let file = self.resolver.curr_vf.clone();

        match self.db.constructor_data(value.name(), file) {
            VariantResolution::Variant(_) => {}
            VariantResolution::Binding(name) => {
                let scope = self.last_scope();
                let mut scope = scope.borrow_mut();
                let name = name.to_fn_id();
                let value = Arc::new(value.into());
                scope.functions.insert(name, Value::Pat(value));
            }
            VariantResolution::None => {
                let fn_id = name.to_fn_id();
                self.resolver
                    .reporter
                    .report(&name, UnresolvedNameError(fn_id));
            }
        }
    }

    fn enter_constructor_pat(&mut self, value: asena_ast::ConstructorPat) {
        let name = value.name();
        let file = self.resolver.curr_vf.clone();

        match self.db.constructor_data(value.name(), file) {
            VariantResolution::Binding(_) if !value.arguments().is_empty() => {
                let fn_id = name.to_fn_id();
                self.resolver
                    .reporter
                    .report(&name, UnresolvedNameError(fn_id));
            }
            VariantResolution::Variant(_) | VariantResolution::Binding(_) => {}
            VariantResolution::None => {
                let fn_id = name.to_fn_id();
                self.resolver
                    .reporter
                    .report(&name, UnresolvedNameError(fn_id));
            }
        }
    }
}

impl<'a> AsenaVisitor<()> for AstResolver<'a> {
    fn visit_use(&mut self, value: asena_ast::Use) {
        let module_ref = self.db.module_ref(value.to_fn_id().as_str());

        self.db.add_path_dep(self.curr_vf.clone(), module_ref);
    }

    fn visit_signature(&mut self, signature: Signature) {
        let name = signature.name();
        let mut resolver = ScopeResolver::new(name, self);

        for (name, parameter) in Parameter::compute_parameters(signature.parameters()) {
            let mut scope = resolver.local_scope.borrow_mut();
            let value = Arc::new(parameter);
            scope.functions.insert(name, Value::Param(value));
        }

        let mut resolver: &mut dyn AsenaListener<()> = &mut resolver;

        signature.return_type().listen(&mut resolver);
        signature.body().listen(&mut resolver);
    }

    fn visit_assign(&mut self, assign: Assign) {
        let name = assign.name();
        let resolver = &mut ScopeResolver::new(name, self);

        for pat in assign.patterns() {
            let mut resolver: &mut dyn AsenaListener<()> = resolver;

            pat.listen(&mut resolver);
        }

        let mut resolver: &mut dyn AsenaListener<()> = resolver;

        assign.body().listen(&mut resolver);
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
