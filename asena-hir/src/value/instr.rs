use crate::Name;

use super::*;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
pub enum HirInstr {
    Null,
    Let(Name, HirValue),
    Variable(Name),
    Block(Block),
    ObjectClone(HirValue), // Object.clone
    ObjectDrop(HirValue),  // Object.drop
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub instructions: Vec<HirValue>,
    pub value: HirValue,
}
