use std::{fmt::Debug, sync::Arc};

use asena_hir_derive::hir_debug;

use crate::{query::HirDebug, NameId};

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

impl HirDebug for HirDecimal {
    type Database = dyn crate::database::HirBag;

    fn fmt(&self, _: Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.integer, self.decimal)
    }
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_debug]
pub struct HirString {
    pub value: String,
    pub name: Option<NameId>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub enum HirLiteral {
    #[default]
    Error,
    Int(usize, HirISize, HirISign),
    Decimal(HirFSize, HirDecimal),
    String(HirString),
}

impl HirDebug for HirLiteral {
    type Database = dyn crate::database::HirBag;

    fn fmt(&self, _: Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HirLiteral::Error => write!(f, "Error"),
            HirLiteral::Int(value, size, sign) => write!(f, "Int({value}, {size:?}, {sign:?})"),
            HirLiteral::Decimal(size, decimal) => write!(f, "Decimal({size:?}, {decimal:?})"),
            HirLiteral::String(string) => write!(f, "String({})", string.value),
        }
    }
}
