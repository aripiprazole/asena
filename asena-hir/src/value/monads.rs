use crate::NameId;

use super::*;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
pub enum HirMonad {
    PureUnit,
    Pure(HirValue),
    Bind(NameId, HirValue, HirValue),
}
