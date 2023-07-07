use crate::{HirBaseDatabase, HirNode};

pub trait HirDebug: HirNode {
    fn fmt(&self, db: &dyn HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
