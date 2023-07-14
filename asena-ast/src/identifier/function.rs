use super::*;

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

impl AstName for Lexeme<FunctionId> {
    fn into_spanned(self) -> Spanned<FunctionId> {
        Spanned::new(self.location().into_owned(), self.data().clone())
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
            TokenKind::SelfKeyword => Self(token.text.clone()),
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

impl LexemeListenable for FunctionId {
    type Listener<'a> = &'a mut dyn AsenaListener<()>;

    fn lexeme_listen(value: Lexeme<Self>, listener: &mut Self::Listener<'_>) {
        listener.visit_function_id(value);
    }
}
