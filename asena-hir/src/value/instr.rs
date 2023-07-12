use std::fmt::Formatter;

use crate::NameId;

use super::*;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirValue)]
#[hir_debug]
pub enum HirInstr {
    Null,
    Let(NameId, HirValueId),
    Variable(NameId),
    Block(Block),
    ObjectClone(HirValueId), // Object.clone
    ObjectDrop(HirValueId),  // Object.drop
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub instructions: Vec<HirValueId>,
    pub value: HirValueId,
}

impl HirDebug for Block {
    type Database = dyn crate::database::HirBag;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.instructions.is_empty() {
            self.value.fmt(db, f)
        } else {
            write!(f, "HirValueInstrBlock(")?;
            let mut s = f.debug_list();
            for instruction in self.instructions.iter() {
                s.entry(&hir_dbg!(db.clone(), instruction));
            }
            s.entry(&hir_dbg!(db.clone(), &self.value));
            s.finish()?;
            write!(f, ")")
        }
    }
}
