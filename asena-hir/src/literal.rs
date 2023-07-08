use crate::NameId;

#[derive(Hash, Clone, Copy, Debug)]
pub enum HirIntSize {
    U1,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

#[derive(Hash, Clone, Copy, Debug)]
pub enum HirFloatSize {
    F32,
    F64,
}

#[derive(Hash, Clone, Copy, Debug)]
pub enum HirIntSign {
    Signed,
    Unsigned,
}

#[derive(Hash, Clone, Copy, Debug)]
pub struct HirDecimal {
    pub divided: usize,
    pub divisor: usize,
}

#[derive(Hash, Clone, Debug)]
pub struct HirString {
    pub value: String,
    pub name: Option<NameId>,
}

#[derive(Hash, Clone, Debug)]
pub enum HirLiteral {
    Int(usize, HirIntSize, HirIntSign),
    Decimal(HirDecimal, HirFloatSize),
    String(HirString),
}
