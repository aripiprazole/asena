use crate::NameId;

use super::*;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
#[hir_debug]
pub enum HirMonad {
    PureUnit,
    Pure(HirValueId),
    Bind(NameId, HirValueId, HirValueId),
}
