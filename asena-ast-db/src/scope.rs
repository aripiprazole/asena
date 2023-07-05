use std::{borrow::Borrow, cell::RefCell, rc::Rc, sync::Arc};

use asena_ast::{
    Decl, Enum, Expr, FunctionId, GlobalName, LamParameter, Local, Parameter, Pat, Signature,
    Variant,
};
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
    None,
    Sign(Arc<Signature>),
    Cons(Arc<Variant>),
    Pat(Arc<Pat>),
    Param(Arc<Parameter>),
    LamParam(Arc<LamParameter>),
    Expr(Arc<Expr>),
}

#[derive(Default, Debug, Clone)]
pub struct ScopeData {
    pub kind: ScopeKind,
    pub declarations: std::collections::HashMap<FunctionId, Arc<Decl>>,
    pub constructors: std::collections::HashMap<FunctionId, Arc<Variant>>,
    pub functions: std::collections::HashMap<FunctionId, Value>,
    pub variables: std::collections::HashMap<FunctionId, usize>,
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

    pub fn find_value<T>(&self, name: &T) -> Value
    where
        T: GlobalName,
    {
        let name = name.to_fn_id();

        self.functions.get(&name).cloned().unwrap_or(Value::None)
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

impl Value {
    pub fn or_else<F>(&self, other: F) -> Value
    where
        F: Fn() -> Value,
    {
        match self {
            Value::None => other(),
            _ => self.clone(),
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
