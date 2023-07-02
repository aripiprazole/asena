pub trait Visitable {
    type Visitor<'a, T: Default + 'a>;

    fn accept<T: Default>(&self, visitor: Self::Visitor<'_, T>) -> T;
}
