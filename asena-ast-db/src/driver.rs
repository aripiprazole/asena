use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::database::AstDatabase;

pub struct Driver(pub Arc<dyn AstDatabase>);

impl DerefMut for Driver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for Driver {
    type Target = Arc<dyn AstDatabase>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
