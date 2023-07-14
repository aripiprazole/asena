use super::*;

impl GreenTreeKind {
    /// Checks if the tree matches the given kind.
    pub fn matches(&self, nth: usize, kind: TokenKind) -> bool {
        match self {
            Self::Leaf(leaf) => leaf.data.matches(nth, kind),
            _ => false,
        }
    }

    /// Returns the [Spanned] value of the tree, if it's not an error node, then it should
    /// return the default value.
    pub fn or_empty(self) -> Arc<Spanned<Tree>> {
        match self {
            Self::Leaf(leaf) => leaf.data,
            _ => Default::default(),
        }
    }

    /// Returns the [TreeKind] of the tree, and if it's not a [AstLeaf], it will return an
    /// Error kind.
    pub fn kind(&self) -> TreeKind {
        match self {
            Self::Leaf(leaf) => leaf.data.kind,
            _ => TreeKind::Error,
        }
    }

    /// Returns the value of the given name, if it exists, otherwise it will return the default
    /// value.
    pub fn spanned(&self) -> Spanned<()> {
        match self {
            Self::Leaf(leaf) => leaf.data.replace(()),
            Self::Token(lexeme) => lexeme.token.replace(()),
            _ => Spanned::default(),
        }
    }

    /// Returns if the value is the only element in the tree.
    pub fn is_single(&self) -> bool {
        match self {
            Self::Leaf(leaf) => leaf.data.is_single(),
            Self::Token(..) => true,
            _ => false,
        }
    }

    /// Returns the tree children, if it's not an error node.
    pub fn children(&mut self) -> Option<&mut Vec<Spanned<Child>>> {
        match self {
            Self::Leaf(leaf) => Some(&mut Arc::make_mut(&mut leaf.data).children),
            _ => None,
        }
    }

    /// Returns filtered cursor to the children, if it's not an error node.
    pub fn filter<T: Leaf + Node>(&self) -> Cursor<Vec<T>> {
        match self {
            Self::Leaf(leaf) => leaf.data.filter(),
            _ => Cursor::empty(),
        }
    }

    /// Returns a terminal node, if it's not an error node.
    pub fn any_token(&self, kind: TokenKind) -> Vec<Spanned<Token>> {
        match self {
            Self::Leaf(leaf) => leaf.data.token(kind),
            _ => vec![],
        }
    }

    pub fn token(&self, kind: TokenKind) -> Spanned<Token> {
        match self {
            Self::Leaf(leaf) => leaf.data.token(kind).first().cloned().unwrap_or_default(),
            _ => Default::default(),
        }
    }

    /// Returns a terminal node, if it's not an error node.
    pub fn terminal<T: Terminal + 'static>(&self, nth: usize) -> Cursor<Lexeme<T>> {
        match self {
            Self::Leaf(leaf) => leaf.data.terminal(nth),
            _ => Cursor::empty(),
        }
    }

    /// Returns terminal filtered cursor to the children, if it's not an error node.
    pub fn filter_terminal<T: Terminal + 'static>(&self) -> Cursor<Vec<Lexeme<T>>> {
        match self {
            Self::Leaf(leaf) => leaf.data.filter_terminal(),
            _ => Cursor::empty(),
        }
    }

    /// Returns a leaf node, if it's not an error node.
    pub fn at<T: Node + Leaf>(&self, nth: usize) -> Cursor<T> {
        match self {
            Self::Leaf(leaf) => leaf.data.at(nth),
            _ => Cursor::empty(),
        }
    }

    /// Returns if the tree has the given name in the current name hash map.
    pub fn has(&self, name: LeafKey) -> bool {
        match self {
            Self::Leaf(leaf) => matches!(leaf.children.get(name), Some(..)),
            _ => false,
        }
    }
}
