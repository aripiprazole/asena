use std::{borrow::Borrow, sync::Arc};

use asena_ast::{Decl, Expr, FunctionId, Signature, Variant};

use crate::{database::AstDatabase, vfs::VfsFile};

#[derive(Default, Debug, Clone)]
pub enum ScopeKind {
    #[default]
    Global,
    File(Arc<VfsFile>),
}

#[derive(Debug, Clone)]
pub enum Function {
    Signature(Arc<Signature>),
    Constructor(Arc<Variant>),
    Expr(Arc<Expr>),
}

#[derive(Default, Debug, Clone)]
pub struct ScopeData {
    pub kind: ScopeKind,
    pub declarations: im::HashMap<FunctionId, Arc<Decl>>,
    pub constructors: im::HashMap<FunctionId, Arc<Variant>>,
    pub functions: im::HashMap<FunctionId, Function>,
    pub variables: im::HashMap<FunctionId, usize>,
}

impl ScopeData {
    pub fn rename_all(
        &mut self,
        db: &dyn AstDatabase,
        vfs_file: Arc<VfsFile>,
        prefix: Option<FunctionId>,
    ) {
        for (name, constructor) in db.constructors(vfs_file.clone()).iter() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());

            self.constructors.insert(name.clone(), constructor.clone());
        }

        for (name, decl) in db.items(vfs_file).iter() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());

            match decl.borrow() {
                Decl::Signature(signature) => {
                    let function = Function::Signature(Arc::new(signature.clone()));
                    self.functions.insert(name.clone(), function);
                }
                Decl::Assign(_)
                | Decl::Class(_)
                | Decl::Instance(_)
                | Decl::Enum(_)
                | Decl::Trait(_) => {
                    self.declarations.insert(name.clone(), decl.clone());
                }
                Decl::Command(_) | Decl::Use(_) | Decl::Error => {}
            }
        }
    }
}
