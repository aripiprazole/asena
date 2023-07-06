use std::{borrow::Borrow, cell::RefCell, rc::Rc, sync::Arc};

use asena_ast::*;
use asena_leaf::ast::Lexeme;

use crate::{database::AstDatabase, driver::HasDB, vfs::VfsFile};

#[derive(Default, Debug, Clone)]
pub enum ScopeKind {
    #[default]
    Global,
    File(Arc<VfsFile>),
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Synthetic,
    Sign(Arc<Signature>),
    Cons(Arc<Variant>),
    Pat(Arc<Pat>),
    Param(Arc<Parameter>),
    LamParam(Arc<LamParameter>),
    Expr(Arc<Expr>),
}

#[derive(Debug, Clone)]
pub enum TypeValue {
    Decl(Arc<Decl>),
    Synthetic,
    None,
}

#[derive(Default, Debug, Clone)]
pub struct ScopeData {
    pub kind: ScopeKind,
    pub types: im::HashMap<FunctionId, TypeValue>,
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

    pub fn create_enum(&mut self, enum_decl: &Enum, prefix: Option<FunctionId>) {
        let name = FunctionId::optional_path(prefix.clone(), enum_decl.name().to_fn_id());
        let enum_value = TypeValue::Decl(Arc::new(enum_decl.clone().into()));
        self.types.insert(name, enum_value);
        for (name, variant) in enum_decl.constructors() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());
            let variant = Arc::new(variant);

            self.constructors.insert(name.clone(), variant.clone());
            self.functions.insert(name.clone(), Value::Cons(variant));
        }
    }

    pub fn create_trait(&mut self, trait_decl: &Trait, prefix: Option<FunctionId>) {
        let prefix = FunctionId::optional_path(prefix, trait_decl.name().to_fn_id());
        let class_value = TypeValue::Decl(Arc::new(trait_decl.clone().into()));
        self.types.insert(prefix.clone(), class_value);
        for method in trait_decl.default_methods() {
            let method_name = method.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);

            self.functions.insert(name.clone(), Value::Synthetic);
        }
        for field in trait_decl.fields() {
            let method_name = field.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);

            self.functions.insert(name.clone(), Value::Synthetic);
        }
    }

    pub fn create_class(&mut self, class_decl: &Class, prefix: Option<FunctionId>) {
        let prefix = FunctionId::optional_path(prefix, class_decl.name().to_fn_id());
        let class_value = TypeValue::Decl(Arc::new(class_decl.clone().into()));
        self.types.insert(prefix.clone(), class_value);
        for method in class_decl.methods() {
            let method_name = method.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);

            self.functions.insert(name.clone(), Value::Synthetic);
        }
        for field in class_decl.fields() {
            let method_name = field.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);

            self.functions.insert(name.clone(), Value::Synthetic);
        }
    }

    pub fn find_value(&self, name: &impl GlobalName) -> Value {
        let name = name.to_fn_id();

        self.functions.get(&name).cloned().unwrap_or(Value::None)
    }

    pub fn find_type(&self, name: &impl GlobalName) -> TypeValue {
        let name = name.to_fn_id();

        self.types.get(&name).cloned().unwrap_or(TypeValue::None)
    }

    pub fn find_type_constructor(&self, name: &impl GlobalName) -> VariantResolution {
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

    pub fn import<'a, B: HasDB<'a>, P>(&mut self, db: B, file: Arc<VfsFile>, prefix: P)
    where
        P: Into<Option<FunctionId>>,
    {
        let prefix: Option<_> = prefix.into();
        let db: &dyn AstDatabase = db.db();
        for (name, decl) in db.items(file).iter() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());

            match decl.borrow() {
                Decl::Signature(signature) => {
                    let function = Value::Sign(Arc::new(signature.clone()));
                    self.functions.insert(name.clone(), function);
                }
                Decl::Enum(enum_decl) => {
                    self.create_enum(enum_decl, prefix.clone());
                }
                Decl::Class(class_decl) => {
                    self.create_class(class_decl, prefix.clone());
                }
                Decl::Trait(trait_decl) => {
                    self.create_trait(trait_decl, prefix.clone());
                }
                Decl::Assign(_) | Decl::Instance(_) => {
                    self.types
                        .insert(name.clone(), TypeValue::Decl(decl.clone()));
                }
                Decl::Command(_) | Decl::Use(_) | Decl::Error => {}
            }
        }
    }
}

impl Value {
    pub fn or_else(&self, other: impl Fn() -> Value) -> Value {
        match self {
            Value::None => other(),
            _ => self.clone(),
        }
    }
}

impl VariantResolution {
    pub fn or_else(&self, other: impl Fn() -> VariantResolution) -> VariantResolution {
        match self {
            VariantResolution::None => other(),
            _ => self.clone(),
        }
    }
}

impl TypeValue {
    pub fn or_else(&self, other: impl Fn() -> TypeValue) -> TypeValue {
        match self {
            TypeValue::None => other(),
            _ => self.clone(),
        }
    }
}
