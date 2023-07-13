use super::*;

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Default, Node, Clone, Hash, PartialEq, Eq)]
pub struct QualifiedPath(GreenTree);

impl GlobalName for QualifiedPath {}

impl Located for QualifiedPath {
    fn location(&self) -> Cow<'_, Loc> {
        Cow::Owned(self.segments().location().into_owned())
    }
}

impl Debug for QualifiedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifiedPath")?;
        for segment in self.segments().iter() {
            write!(f, " [{:?}]", segment.0)?;
        }
        Ok(())
    }
}

impl Leaf for QualifiedPath {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            QualifiedPathTree => QualifiedPath::new(tree),
            _ => return None,
        })
    }
}

impl Listenable for QualifiedPath {
    type Listener<'a> = &'a mut dyn AsenaListener<()>;

    fn listen(&self, listener: &mut Self::Listener<'_>) {
        listener.enter_qualified_path(self.clone());
        for segment in self.segments().iter() {
            segment.listen(listener)
        }
        listener.exit_qualified_path(self.clone());
    }
}

impl Walkable for QualifiedPath {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        walker.visit_qualified_path(self.clone());
        for segment in self.segments().iter() {
            segment.walk(walker)
        }
    }
}
