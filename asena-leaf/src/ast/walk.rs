pub trait Walkable<T> {
    fn walk(&self, walker: &mut T);

    fn run(&self, mut walker: T) {
        self.walk(&mut walker)
    }
}

impl<W, T: Walkable<W>> Walkable<W> for Vec<T> {
    fn walk(&self, walker: &mut W) {
        for ele in self {
            ele.walk(walker)
        }
    }
}

impl<W, T: Walkable<W>> Walkable<W> for Option<T> {
    fn walk(&self, walker: &mut W) {
        match self {
            Some(value) => value.walk(walker),
            None => {}
        }
    }
}
