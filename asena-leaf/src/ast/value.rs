use std::rc::Rc;

use super::GreenTree;

#[derive(Clone)]
pub enum Value<T> {
    Ref(GreenTree),
    Value(Rc<T>),
}

impl<T> Default for Value<T> {
    fn default() -> Self {
        Value::Ref(Default::default())
    }
}
