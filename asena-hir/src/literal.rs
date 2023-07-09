use asena_hir_derive::hir_debug;

use crate::NameId;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_debug]
pub enum HirISize {
    U1,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_debug]
pub enum HirFSize {
    F32,
    F64,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_debug]
pub enum HirIntSign {
    Signed,
    Unsigned,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
#[hir_debug]
pub struct HirDecimal {
    pub integer: usize,
    pub decimal: usize,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_debug]
pub struct HirString {
    pub value: String,
    pub name: Option<NameId>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_debug]
pub enum HirLiteral {
    #[default]
    Error,
    Int(usize, HirISize, HirIntSign),
    Decimal(HirFSize, HirDecimal),
    String(HirString),
}
