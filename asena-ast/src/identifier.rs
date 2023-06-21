use std::fmt::Debug;

use asena_derive::Leaf;

use asena_leaf::ast::{Cursor, GreenTree};

use asena_span::{Loc, Spanned};

//>>>Identifiers
/// Identifier's key to a function (everything on the language), this can be abstracted in another
/// identifiers. Serves as a key on a graph, or the abstract syntax tree representation.
#[derive(Clone)]
pub struct FunctionId(pub String);

impl Debug for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", self.0)
    }
}

/// Identifier's key to a type constructor.
#[derive(Clone)]
pub struct ConstructorId(pub Vec<Spanned<FunctionId>>);

impl Debug for ConstructorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstructorId {:#?}", self.0)
    }
}

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Clone)]
pub struct Local(pub Spanned<String>);

impl Debug for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalId {:#?}", self.0)
    }
}

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Leaf, Clone)]
pub struct QualifiedPath(GreenTree);

impl Debug for QualifiedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifiedPath ")?;
        for segment in self.segments().as_leaf() {
            write!(f, " ({:?})", segment.0)?;
        }
        Ok(())
    }
}

impl FunctionId {
    /// Creates a new [FunctionId] by a string
    pub fn new(id: &str) -> Self {
        Self(id.into())
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ConstructorId {
    /// Creates a new [ConstructorId] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(vec![Spanned::new(span, FunctionId::new(id))])
    }
}

impl Local {
    /// Creates a new [Local] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(Spanned::new(span, id.into()))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.value().as_str()
    }
}

impl QualifiedPath {
    pub fn segments(&self) -> Cursor<Vec<Local>> {
        self.filter_terminal()
    }
}
