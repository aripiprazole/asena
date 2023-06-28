use std::borrow::Cow;
use std::fmt::{Debug, Display};

use asena_derive::*;

use asena_leaf::ast::{GreenTree, Leaf, Lexeme, Located, Node, Terminal, Walkable};
use asena_leaf::node::TreeKind::*;

use asena_leaf::token::{Token, TokenKind};
use asena_span::{Loc, Spanned};

//>>>Identifiers
/// Identifier's key to a function (everything on the language), this can be abstracted in another
/// identifiers. Serves as a key on a graph, or the abstract syntax tree representation.
#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct FunctionId(pub String);

impl Located for FunctionId {
    fn location(&self) -> std::borrow::Cow<'_, Loc> {
        Cow::Owned((0..0).into())
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

impl From<&str> for FunctionId {
    fn from(value: &str) -> Self {
        FunctionId::new(value)
    }
}

impl Terminal for FunctionId {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let text = token.text.clone();

        Some(FunctionId(text))
    }
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Debug for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", self.0)
    }
}

impl<W> Walkable<W> for FunctionId {
    fn walk(&self, _walker: &mut W) {}
}

/// Identifier's key to a type constructor.
#[derive(Default, Clone)]
pub struct ConstructorId(pub Vec<Spanned<FunctionId>>);

impl ConstructorId {
    /// Creates a new [ConstructorId] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(vec![Spanned::new(span, FunctionId::new(id))])
    }
}

impl Terminal for ConstructorId {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        let text = token.text.clone();
        let span = token.span;

        Some(ConstructorId::new(span, &text))
    }
}

impl Debug for ConstructorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstructorId {:#?}", self.0)
    }
}

impl<W> Walkable<W> for ConstructorId {
    fn walk(&self, _walker: &mut W) {}
}

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Default, Clone)]
pub struct Local(pub String, pub Loc);

impl Local {
    /// Creates a new [Local] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(id.into(), span)
        // Self(Spanned::new(span, id.into()))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_fn_id(&self) -> FunctionId {
        FunctionId::new(&self.0)
    }
}

impl Located for Local {
    fn location(&self) -> std::borrow::Cow<'_, Loc> {
        Cow::Borrowed(&self.1)
    }
}

impl Terminal for Local {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        Some(match token.kind {
            TokenKind::Identifier => {
                let text = token.text.clone();
                let span = token.span;

                Local::new(span, &text)
            }
            _ => return None,
        })
    }
}

impl Debug for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Local {:#?}", self.0)
    }
}

impl<W> Walkable<W> for Local {
    fn walk(&self, _walker: &mut W) {}
}

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Default, Node, Clone)]
pub struct QualifiedPath(GreenTree);

#[ast_of]
impl QualifiedPath {
    #[ast_leaf]
    pub fn segments(&self) -> Vec<Lexeme<FunctionId>> {
        self.filter_terminal()
    }

    pub fn to_fn_id(&self) -> FunctionId {
        let mut paths = Vec::new();
        for lexeme in self.segments().iter() {
            paths.push(lexeme.0.clone())
        }

        FunctionId::new(&paths.join("."))
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

impl<W> Walkable<W> for QualifiedPath {
    fn walk(&self, walker: &mut W) {
        for ele in self.segments().iter() {
            ele.walk(walker)
        }
    }
}
