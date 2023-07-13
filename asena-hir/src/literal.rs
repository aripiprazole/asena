use std::fmt::Debug;

use crate::Name;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
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
pub enum HirFSize {
    F32,
    F64,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HirISign {
    Signed,
    Unsigned,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct HirDecimal {
    pub integer: usize,
    pub decimal: usize,
}

impl Debug for HirDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.integer, self.decimal)
    }
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirString {
    pub value: String,
    pub name: Option<Name>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub enum HirLiteral {
    #[default]
    Error,
    Int(usize, HirISize, HirISign),
    Decimal(HirFSize, HirDecimal),
    String(HirString),
}
