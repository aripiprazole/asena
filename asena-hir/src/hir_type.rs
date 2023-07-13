use asena_hir_derive::*;

use crate::NameId;

use self::data::HirTypeFunction;

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
pub struct HirTypeName {
    pub name: NameId,
    pub is_constructor: bool,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
pub struct HirTypeApp {
    pub callee: HirTypeFunction,
    pub arguments: Vec<data::HirTypeArgument>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirType)]
pub enum HirTypeKind {
    #[default]
    Error,
    Unit,
    This,
    HirTypeName(HirTypeName),
    HirTypeApp(HirTypeApp),
}

#[hir_struct]
pub struct HirType {
    pub kind: HirTypeKind,
}

pub mod data {
    use super::*;

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirTypeFunction {
        #[default]
        Error,
        Pi,
        Type(HirType),
    }

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirTypeArgument {
        #[default]
        Error,
        Type(HirType),
        Named(NameId, HirType),
    }
}
