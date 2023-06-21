use super::GreenTree;

#[derive(Clone)]
pub enum Value<T> {
    Ref(GreenTree),
    Value(T),
}

impl<T> Default for Value<T> {
    fn default() -> Self {
        Value::Ref(Default::default())
    }
}
