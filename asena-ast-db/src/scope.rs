use std::{borrow::Borrow, cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

use asena_ast::*;
use asena_leaf::ast::{Lexeme, Located};

use crate::def::{Def, DefWithId};
use crate::{
    db::AstDatabase,
    vfs::{VfsFile, VfsFileData},
    ModuleRef,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    #[default]
    Global,
    File(Arc<VfsFileData>),
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ScopeData {
    pub kind: ScopeKind,
    pub types: im::HashMap<FunctionId, DefWithId>,
    pub constructors: im::HashMap<FunctionId, DefWithId>,
    pub functions: im::HashMap<FunctionId, DefWithId>,
    pub variables: im::HashMap<FunctionId, usize>,

    pub modules: im::HashMap<String, ModuleRef>,
    pub paths: im::HashMap<PathBuf, ModuleRef>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum VariantResolution {
    None,
    Binding(Box<Lexeme<Local>>),
    Variant(DefWithId),
}

impl ScopeData {
    pub fn fork(&self) -> Rc<RefCell<ScopeData>> {
        Rc::new(RefCell::new(self.clone()))
    }

    pub fn create_enum<P>(&mut self, db: &dyn AstDatabase, decl: &Enum, prefix: P)
    where
        P: Into<Option<FunctionId>>,
    {
        let prefix = prefix.into();
        let name = FunctionId::optional_path(prefix.clone(), decl.name().to_fn_id());
        let enum_value = DefWithId::new(db, decl.name(), decl.location().into_owned());
        self.types.insert(name, enum_value);

        for (name, variant) in decl.constructors() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());
            let variant = DefWithId::new(db, variant.name(), variant.location().into_owned());

            self.constructors.insert(name.clone(), variant);
            self.functions.insert(name.clone(), variant);
        }
    }

    pub fn create_trait<P>(&mut self, db: &dyn AstDatabase, decl: &Trait, prefix: P)
    where
        P: Into<Option<FunctionId>>,
    {
        let prefix = prefix.into();
        let prefix = FunctionId::optional_path(prefix, decl.name().to_fn_id());
        let class_value = DefWithId::new(db, decl.name(), decl.location().into_owned());
        self.types.insert(prefix.clone(), class_value);

        for method in decl.default_methods() {
            let method_name = method.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);
            let def = DefWithId::new(db, method.name(), method.location().into_owned());

            self.functions.insert(name.clone(), def);
        }

        for field in decl.fields() {
            let method_name = field.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);
            let def = DefWithId::new(db, field.name(), field.location().into_owned());

            self.functions.insert(name.clone(), def);
        }
    }

    pub fn create_class<P>(&mut self, db: &dyn AstDatabase, decl: &Class, prefix: P)
    where
        P: Into<Option<FunctionId>>,
    {
        let prefix = prefix.into();
        let prefix = FunctionId::optional_path(prefix, decl.name().to_fn_id());
        let class_value = DefWithId::new(db, decl.name(), decl.location().into_owned());
        self.types.insert(prefix.clone(), class_value);

        for method in decl.methods() {
            let method_name = method.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);
            let def = DefWithId::new(db, method.name(), method.location().into_owned());

            self.functions.insert(name.clone(), def);
        }

        for field in decl.fields() {
            let method_name = field.name().to_fn_id();
            let name = FunctionId::optional_path(prefix.clone().into(), method_name);
            let def = DefWithId::new(db, field.name(), field.location().into_owned());

            self.functions.insert(name.clone(), def);
        }
    }

    pub fn find_value(&self, name: &impl GlobalName) -> Def {
        let name = name.to_fn_id();

        self.functions
            .get(&name)
            .cloned()
            .map(Def::WithId)
            .unwrap_or(Def::Unresolved)
    }

    pub fn find_type(&self, name: &impl GlobalName) -> Def {
        let name = name.to_fn_id();

        self.types
            .get(&name)
            .cloned()
            .map(Def::WithId)
            .unwrap_or(Def::Unresolved)
    }

    pub fn find_type_constructor(&self, name: &impl GlobalName) -> VariantResolution {
        match self.constructors.get(&name.to_fn_id()) {
            Some(variant) => VariantResolution::Variant(*variant),

            // if it is not a constructor, it is a variable binding: Vec.cons x xs
            //                                                                ^ ^^
            None if name.is_ident().is_some() => {
                VariantResolution::Binding(Box::new(name.is_ident().unwrap()))
            }

            // if it is a constructor and it is not found, report an error
            None => VariantResolution::None,
        }
    }

    pub fn import<'a, P>(&mut self, db: &dyn AstDatabase, file: VfsFile, prefix: P)
    where
        P: Into<Option<FunctionId>> + Clone + 'a,
    {
        let prefix: Option<_> = prefix.into();
        for (name, decl) in db.items(file).iter() {
            let name = FunctionId::optional_path(prefix.clone(), name.clone());

            match decl.borrow() {
                Decl::Signature(decl) => {
                    let def = DefWithId::new(db, decl.name(), decl.location().into_owned());
                    self.functions.insert(name.clone(), def);
                }
                Decl::Enum(ref decl) => {
                    self.create_enum(db, decl, prefix.clone());
                }
                Decl::Class(ref decl) => {
                    self.create_class(db, decl, prefix.clone());
                }
                Decl::Trait(ref decl) => {
                    self.create_trait(db, decl, prefix.clone());
                }
                Decl::Assign(_) | Decl::Instance(_) => {}
                Decl::Command(_) | Decl::Use(_) | Decl::Error => {}
            }
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
