use std::{borrow::Borrow, ops::Deref, sync::Arc};

use crate::database::AstDatabase;

#[derive(Clone)]
pub struct Driver(pub Arc<dyn AstDatabase>);

pub trait HasDB<'a> {
    fn db(self) -> &'a dyn AstDatabase;
}

impl<'a> HasDB<'a> for &'a Driver {
    fn db(self) -> &'a dyn AstDatabase {
        self.0.borrow()
    }
}

impl Deref for Driver {
    type Target = dyn AstDatabase;

    fn deref(&self) -> &Self::Target {
        self.0.borrow()
    }
}
