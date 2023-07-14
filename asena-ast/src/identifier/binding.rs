use super::*;

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`. But as the original reference.
#[derive(Default, Node, Clone, Hash, PartialEq, Eq)]
pub struct BindingId(GreenTree);

impl GlobalName for BindingId {}

impl Located for BindingId {
    fn location(&self) -> Cow<'_, Loc> {
        Cow::Owned(self.segments().location().into_owned())
    }
}

impl AstName for BindingId {
    fn into_spanned(self) -> Spanned<FunctionId> {
        Spanned::new(self.location().into_owned(), self.to_fn_id())
    }
}

impl Debug for BindingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifiedBindingId")?;
        for segment in self.segments().iter() {
            write!(f, " [{:?}]", segment.0)?;
        }
        Ok(())
    }
}

impl Leaf for BindingId {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            QualifiedPathTree => BindingId::new(tree),
            _ => return None,
        })
    }
}

impl Listenable for BindingId {
    type Listener<'a> = &'a mut dyn AsenaListener<()>;

    fn listen(&self, listener: &mut Self::Listener<'_>) {
        listener.enter_qualified_binding_id(self.clone());
        for segment in self.segments().iter() {
            segment.listen(listener)
        }
        listener.exit_qualified_binding_id(self.clone());
    }
}

impl Walkable for BindingId {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        walker.visit_qualified_binding_id(self.clone());
        for segment in self.segments().iter() {
            segment.walk(walker)
        }
    }
}
