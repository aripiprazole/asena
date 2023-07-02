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
pub trait Walkable: Sized {
    type Walker<'a>;

    fn walk(&self, walker: &mut Self::Walker<'_>);
}

impl<T: Walkable> Walkable for Vec<T> {
    type Walker<'a> = T::Walker<'a>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        for ele in self {
            ele.walk(walker)
        }
    }
}

impl<T: Walkable> Walkable for Option<T> {
    type Walker<'a> = T::Walker<'a>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        match self {
            Some(value) => value.walk(walker),
            None => {}
        }
    }
}
