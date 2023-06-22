/// Represents a tree walkable node. It can be accessed with an walker using the proc-macros.
///
/// # Example
/// ```rust,norun
/// pub struct Group;
///
/// #[ast_walkable(ExprWalker)]
/// impl Group {
///   #[ast_leaf]
///   fn value() { ... }
/// }
/// ```
///
/// It should generate the walk for the `value` node, using the `ExprWalker` walker
/// as the walker.
pub trait Walkable<T> {
    fn walk(&self, walker: &mut T);

    fn run(&self, mut walker: T) {
        self.walk(&mut walker)
    }

    /// Walks the current node and returns itself.
    fn walks(self, mut walker: T) -> Self
    where
        Self: Sized,
    {
        self.walk(&mut walker);
        self
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
