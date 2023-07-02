use asena_derive::*;

use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use crate::*;

/// A variant type is a GADT-style declared constructor variant for an enum. It does hold a name and
/// a type.
///
/// # Examples
///
/// ```asena
/// Just : a -> Maybe a
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct TypeVariant(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl TypeVariant {
    #[ast_leaf]
    pub fn name(&self) -> QualifiedPath {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Typed {
        self.filter().first()
    }
}

/// A variant constructor is a constructor for an enum. It does hold a name and a list of types.
///
/// # Examples
///
/// ```asena
/// Just(a)
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct ConstructorVariant(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl ConstructorVariant {
    #[ast_leaf]
    pub fn name(&self) -> QualifiedPath {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Typed> {
        self.filter().first()
    }
}

ast_enum! {
    #[ast_walker(AsenaVisitor)]
    pub enum Variant {
        TypeVariant        <- VariantType,
        ConstructorVariant <- VariantConstructor,
    }
}
