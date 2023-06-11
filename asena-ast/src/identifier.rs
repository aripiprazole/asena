use asena_derive::Leaf;

use asena_leaf::green::GreenTree;
use asena_leaf::spec::Node;

use asena_span::{Loc, Spanned};

//>>>Identifiers
/// Identifier's key to a function (everything on the language), this can be abstracted in another
/// identifiers. Serves as a key on a graph, or the abstract syntax tree representation.
#[derive(Clone)]
pub struct FunctionId(pub String);

/// Identifier's key to a type constructor.
#[derive(Clone)]
pub struct ConstructorId(pub Vec<Spanned<FunctionId>>);

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Clone)]
pub struct Local(pub Spanned<FunctionId>);

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Leaf, Clone)]
pub struct QualifiedPath(GreenTree);

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
        Self(Spanned::new(span, FunctionId::new(id)))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.value().as_str()
    }
}

impl QualifiedPath {
    pub fn segments(&self) -> Vec<Node<Spanned<Local>>> {
        self.filter_terminal()
    }
}
