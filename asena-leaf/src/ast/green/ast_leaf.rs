use std::hash::Hash;

use dashmap::DashMap;

use super::*;

#[derive(Debug, Clone)]
pub struct AstLeaf {
    pub(crate) data: Arc<Spanned<Tree>>,

    pub(crate) synthetic: bool,

    /// A hash map of the named children.
    pub(crate) children: HashMap<LeafKey, Arc<Spanned<Child>>>,

    /// A hash map of the named children.
    pub(crate) keys: Arc<DashMap<&'static str, Arc<dyn Any + Send + Sync>>>,

    /// Lazy names' hash map, they have to exist, to make the tree mutable.
    ///
    /// E.g: I can't set the `lhs` node for `binary` tree, if the tree is immutable, so the
    /// lazy names should be used to compute that things.
    /// ```rs
    /// binary.lhs()
    /// ```
    pub(crate) names: Arc<DashMap<LeafKey, Arc<dyn Any + Send + Sync>>>,
}

impl Eq for AstLeaf {}

impl PartialEq for AstLeaf {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
            && Arc::ptr_eq(&self.names, &other.names)
            && Arc::ptr_eq(&self.keys, &other.keys)
            && self.synthetic == other.synthetic
    }
}

impl Hash for AstLeaf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.synthetic.hash(state);
        self.children.iter().collect::<Vec<_>>().hash(state);
        self.children.iter().collect::<Vec<_>>().hash(state);
        self.children.iter().collect::<Vec<_>>().hash(state);
    }
}
