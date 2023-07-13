use super::*;

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Default, Clone, Hash, PartialEq, Eq)]
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

impl LexemeListenable for Local {
    type Listener<'a> = &'a mut dyn AsenaListener<()>;

    fn lexeme_listen(value: Lexeme<Self>, listener: &mut Self::Listener<'_>) {
        listener.visit_local(value)
    }
}
