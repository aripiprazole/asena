use std::{borrow::Borrow, cell::RefCell, rc::Rc, sync::Arc};

use asena_ast::{Decl, Enum, Expr, FunctionId, GlobalName, Local, Signature, Variant};
use asena_leaf::ast::Lexeme;

use crate::{database::AstDatabase, vfs::VfsFile};

#[derive(Default, Debug, Clone)]
pub enum ScopeKind {
    #[default]
    Global,
    File(Arc<VfsFile>),
}

#[derive(Debug, Clone)]
pub enum Value {
    Sign(Arc<Signature>),
    Cons(Arc<Variant>),
    Expr(Arc<Expr>),
}

#[derive(Default, Debug, Clone)]
pub struct ScopeData {
    pub kind: ScopeKind,
    pub declarations: im::HashMap<FunctionId, Arc<Decl>>,
    pub constructors: im::HashMap<FunctionId, Arc<Variant>>,
    pub functions: im::HashMap<FunctionId, Value>,
    pub variables: im::HashMap<FunctionId, usize>,
}

#[derive(Clone)]
pub enum VariantResolution {
    None,
    Binding(Lexeme<Local>),
    Variant(Arc<Variant>),
}

impl ScopeData {
    pub fn fork(&self) -> Rc<RefCell<ScopeData>> {
        Rc::new(RefCell::new(self.clone()))
    }

    pub fn create_enum(&mut self, enum_decl: Enum, prefix: Option<FunctionId>) {
        for (name, variant) in enum_decl.constructors() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());
            let variant = Arc::new(variant);

            self.constructors.insert(name.clone(), variant.clone());
            self.functions.insert(name.clone(), Value::Cons(variant));

            println!("Declared enum constructor: {}", name);
        }
    }

    pub fn find_type_constructor<T>(&self, name: &T) -> VariantResolution
    where
        T: GlobalName,
    {
        match self.constructors.get(&name.to_fn_id()) {
            Some(variant) => VariantResolution::Variant(variant.clone()),

            // if it is not a constructor, it is a variable binding: Vec.cons x xs
            //                                                                ^ ^^
            None if name.is_ident().is_some() => {
                VariantResolution::Binding(name.is_ident().unwrap())
            }

            // if it is a constructor and it is not found, report an error
            None => VariantResolution::None,
        }
    }

    pub fn import<P>(&mut self, db: &dyn AstDatabase, file: Arc<VfsFile>, prefix: P)
    where
        P: Into<Option<FunctionId>>,
    {
        let prefix = prefix.into();
        for (name, decl) in db.items(file).iter() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());

            match decl.borrow() {
                Decl::Signature(signature) => {
                    let function = Value::Sign(Arc::new(signature.clone()));
                    self.functions.insert(name.clone(), function);
                }
                Decl::Enum(enum_decl) => {
                    self.create_enum(enum_decl.clone(), Some(name.clone()));
                }
                Decl::Assign(_) | Decl::Class(_) | Decl::Instance(_) | Decl::Trait(_) => {
                    self.declarations.insert(name.clone(), decl.clone());
                }
                Decl::Command(_) | Decl::Use(_) | Decl::Error => {}
            }
        }
    }
}

impl VariantResolution {
    pub fn or_else<F>(&self, other: F) -> VariantResolution
    where
        F: Fn() -> VariantResolution,
    {
        match self {
            VariantResolution::None => other(),
            _ => self.clone(),
        }
    }
}
