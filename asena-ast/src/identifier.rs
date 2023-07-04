use std::borrow::Cow;
use std::fmt::{Debug, Display};

use asena_derive::*;

use asena_leaf::ast::{GreenTree, Leaf, Lexeme, LexemeWalkable, Located, Node, Terminal, Walkable};
use asena_leaf::node::TreeKind::*;
use asena_leaf::token::{kind::TokenKind, Token};

use asena_span::{Loc, Span, Spanned};

use crate::AsenaVisitor;

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

    /// Creates a new [FunctionId] by appending a path to the current identifier
    pub fn create_path<I: Into<FunctionId>>(a: I, b: I) -> Self {
        Self(format!("{}.{}", a.into().as_str(), b.into().as_str()))
    }

    pub fn optional_path<I: Clone + Into<FunctionId>>(a: Option<I>, b: I) -> Self {
        a.map(|a| Self::create_path(a.into(), b.clone().into()))
            .unwrap_or(b.into())
    }
}

impl From<&str> for FunctionId {
    fn from(value: &str) -> Self {
        FunctionId::new(value)
    }
}

impl Terminal for FunctionId {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        Some(match token.kind {
            TokenKind::Identifier => Self(token.text.clone()),
            TokenKind::Symbol => Self(token.text.clone()),
            TokenKind::Dot => Self(token.text.clone()),
            TokenKind::DoubleArrow => Self(token.text.clone()),
            TokenKind::LeftArrow => Self(token.text.clone()),
            TokenKind::RightArrow => Self(token.text.clone()),
            _ => return None,
        })
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

impl LexemeWalkable for FunctionId {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn lexeme_walk(value: Lexeme<Self>, walker: &mut Self::Walker<'_>) {
        walker.visit_function_id(value)
    }
}

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Default, Clone)]
pub struct Local(pub String, pub Loc);

impl Local {
    /// Creates a new [Local] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(id.into(), span)
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Checks if the local's identifier is equal to the given string
    ///
    /// # Example
    ///
    /// ```
    /// use asena_ast::Local;
    ///
    /// let local = Local::new(0..0, "foo");
    /// assert!(local.is_ident("foo"));
    /// ```
    pub fn is_ident(&self, id: &str) -> bool {
        self.0 == id
    }

    pub fn to_fn_id(&self) -> FunctionId {
        FunctionId::new(&self.0)
    }
}

impl Located for Local {
    fn location(&self) -> Cow<'_, Loc> {
        Cow::Borrowed(&self.1)
    }
}

impl Terminal for Local {
    fn terminal(token: Spanned<Token>) -> Option<Self> {
        Some(match token.kind {
            TokenKind::SelfKeyword => {
                let text = token.text.clone();
                let span = token.span;

                Local::new(span, &text)
            }
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

impl LexemeWalkable for Local {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn lexeme_walk(value: Lexeme<Self>, walker: &mut Self::Walker<'_>) {
        walker.visit_local(value)
    }
}

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Default, Node, Clone)]
pub struct QualifiedPath(GreenTree);

#[ast_of]
impl QualifiedPath {
    #[ast_leaf]
    pub fn segments(&self) -> Vec<Lexeme<Local>> {
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

impl Located for QualifiedPath {
    fn location(&self) -> Cow<'_, Loc> {
        if self.segments().is_empty() {
            return Cow::Owned(Loc::Synthetic);
        }

        Cow::Owned(
            self.segments().first().unwrap().location().on(self
                .segments()
                .last()
                .unwrap()
                .location()
                .into_owned()),
        )
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

impl Walkable for QualifiedPath {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        walker.visit_qualified_path(self.clone());
        for segment in self.segments().iter() {
            segment.walk(walker)
        }
    }
}

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`. But as the original reference.
#[derive(Default, Node, Clone)]
pub struct QualifiedId(GreenTree);

#[ast_of]
impl QualifiedId {
    #[ast_leaf]
    pub fn segments(&self) -> Vec<Lexeme<Local>> {
        self.filter_terminal()
    }

    pub fn is_ident(&self) -> Option<Lexeme<Local>> {
        if self.segments().len() != 1 {
            return None;
        }

        self.segments().first().cloned()
    }

    pub fn to_fn_id(&self) -> FunctionId {
        let mut paths = Vec::new();
        for lexeme in self.segments().iter() {
            paths.push(lexeme.0.clone())
        }

        FunctionId::new(&paths.join("."))
    }
}

impl Located for QualifiedId {
    fn location(&self) -> Cow<'_, Loc> {
        if self.segments().is_empty() {
            return Cow::Owned(Loc::Synthetic);
        }

        Cow::Owned(
            self.segments().first().unwrap().location().on(self
                .segments()
                .last()
                .unwrap()
                .location()
                .into_owned()),
        )
    }
}

impl Debug for QualifiedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifiedId")?;
        for segment in self.segments().iter() {
            write!(f, " [{:?}]", segment.0)?;
        }
        Ok(())
    }
}

impl Leaf for QualifiedId {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            QualifiedPathTree => QualifiedId::new(tree),
            _ => return None,
        })
    }
}

impl Walkable for QualifiedId {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        walker.visit_qualified_id(self.clone());
        for segment in self.segments().iter() {
            segment.walk(walker)
        }
    }
}
