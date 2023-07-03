use crate::{Class, Decl, Enum, QualifiedPath, Signature, Trait};

pub trait HasName {
    fn name(&self) -> QualifiedPath;
}

impl HasName for Enum {
    fn name(&self) -> QualifiedPath {
        self.find_name().as_leaf()
    }
}

impl HasName for Class {
    fn name(&self) -> QualifiedPath {
        self.find_name().as_leaf()
    }
}

impl HasName for Signature {
    fn name(&self) -> QualifiedPath {
        self.find_name().as_leaf()
    }
}

impl HasName for Trait {
    fn name(&self) -> QualifiedPath {
        self.find_name().as_leaf()
    }
}

impl Decl {
    pub fn downcast_has_name(decl: &Decl) -> Option<&dyn HasName> {
        match decl {
            Decl::Error => None,
            Decl::Use(_) => None,
            Decl::Assign(_) => None,
            Decl::Command(_) => None,
            Decl::Instance(_) => None,
            Decl::Signature(signature) => Some(signature),
            Decl::Class(class) => Some(class),
            Decl::Trait(trait_decl) => Some(trait_decl),
            Decl::Enum(enum_decl) => Some(enum_decl),
        }
    }
}
