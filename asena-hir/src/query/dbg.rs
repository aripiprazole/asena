use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

pub trait HirDebug {
    type Database: ?Sized;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<A: HirDebug> HirDebug for &A {
    type Database = A::Database;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        (*self).fmt(db, f)
    }
}

impl<A: HirDebug> HirDebug for Arc<A> {
    type Database = A::Database;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(db, f)
    }
}

impl<A: HirDebug> HirDebug for Option<A> {
    type Database = A::Database;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(ref value) => {
                write!(f, "Some(")?;
                value.fmt(db, f)?;
                write!(f, ")")
            }
            None => write!(f, "None"),
        }
    }
}

impl<A, B> HirDebug for im::HashMap<A, B>
where
    A: HirDebug<Database = dyn HirBag>,
    B: HirDebug<Database = dyn HirBag>,
{
    type Database = A::Database;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_map();

        for (key, value) in self.iter() {
            dbg.entry(&hir_dbg!(db.clone(), key), &hir_dbg!(db.clone(), value));
        }

        dbg.finish()
    }
}

impl<A: HirDebug<Database = dyn HirBag>> HirDebug for HashSet<A> {
    type Database = A::Database;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_list();

        for value in self.iter() {
            dbg.entry(&hir_dbg!(db.clone(), value));
        }

        dbg.finish()
    }
}

impl<A: HirDebug<Database = dyn HirBag>> HirDebug for Vec<A> {
    type Database = A::Database;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_list();

        for value in self.iter() {
            dbg.entry(&hir_dbg!(db.clone(), value));
        }

        dbg.finish()
    }
}

pub struct HirDebugger<'node> {
    db: Arc<dyn HirBag>,
    node: &'node dyn HirDebug<Database = dyn HirBag>,
}

impl<'node> HirDebugger<'node> {
    pub fn new(db: Arc<dyn HirBag>, node: &'node dyn HirDebug<Database = dyn HirBag>) -> Self {
        Self { db, node }
    }
}

impl Debug for HirDebugger<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.node.fmt(self.db.clone(), f)
    }
}

#[macro_export]
macro_rules! hir_dbg {
    ($db:expr, $e:expr) => {
        $crate::query::dbg::HirDebugger::new($db, &$e)
    };
}

#[macro_export]
macro_rules! impl_hir_dbg {
    ($db:ty, $($ty:ty),*) => {
        $(
            impl $crate::query::dbg::HirDebug for $ty {
                type Database = $db;

                fn fmt(&self, _: std::sync::Arc<Self::Database>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        )*
    };
}

pub use hir_dbg;
use im::HashSet;

use crate::database::HirBag;
