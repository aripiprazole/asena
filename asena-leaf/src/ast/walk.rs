pub trait Walkable<T> {
    fn walk(&self, walker: &T);
}

impl<W, T: Walkable<W>> Walkable<W> for Vec<T> {
    fn walk(&self, walker: &W) {
        for ele in self {
            ele.walk(walker)
        }
    }
}

impl<W, T: Walkable<W>> Walkable<W> for Option<T> {
    fn walk(&self, walker: &W) {
        match self {
            Some(value) => value.walk(walker),
            None => {}
        }
    }
}
