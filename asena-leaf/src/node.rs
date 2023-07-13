use std::fmt::Debug;

use asena_span::Spanned;

use self::kind::TokenKind;

pub use super::kind::*;
pub use super::macros::ast_enum;
pub use super::named::*;
pub use super::token::*;

/// Syntax tree using the kind [TreeKind], built from tokens with the type [Token]. It can be pretty
/// printed, and converted to an abstract-syntax-tree.
///
/// The [Tree] data structure is intended to be used as a concrete syntax tree, and its debug print
/// its like the following:
///
/// ```txt
/// EXPR_BINARY
///     LIT_FLOAT64
///         '1' @ 0..1
///     '+' @ 2..3
///     LIT_FLOAT64
///         '2' @ 4..5
///     '+' @ 6..7
///     LIT_FLOAT64
///        '1' @ 8..9
///      @ 0..9
/// ```
#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct Tree {
    pub name: Option<&'static str>,
    pub kind: TreeKind,
    pub children: Vec<Spanned<Child>>,
}

/// Polymorphic variants of [Token] and [Tree], it can and must be used as an abstract to them.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Child {
    Tree(Tree),
    Token(Token),
}

impl Tree {
    pub fn new(kind: TreeKind) -> Self {
        Self {
            name: None,
            kind,
            children: vec![],
        }
    }

    pub fn single(&self) -> &Token {
        match self.children.first() {
            Some(token) => match &token.value {
                Child::Token(token) => token,
                Child::Tree(..) => panic!("called `Tree::single` on a non-terminal node"),
            },
            None => panic!("called `Tree::single` on a empty node"),
        }
    }

    pub fn is_single(&self) -> bool {
        self.children.len() == 1
    }

    pub fn matches(&self, nth: usize, kind: TokenKind) -> bool {
        let Some(child) = self.children.get(nth) else {
            return false;
        };

        match &child.value {
            Child::Tree(..) => false,
            Child::Token(token) => token.kind == kind,
        }
    }

    /// Uses the [std::fmt::Formatter] to write a pretty-printed tree in the terminal for debug
    /// porpuses.
    ///
    /// It usually likes like the following printed code:
    /// ```txt
    /// EXPR_BINARY
    ///     LIT_INT8
    ///         '1' @ 0..1
    ///     '+' @ 2..3
    ///     LIT_INT8
    ///         '1' @ 4..5
    /// @ 0..5
    /// ```
    ///
    /// The use of this code is like the following code, you should never use directly this function
    /// since its local:
    /// ```
    /// let tree = Tree::new(TreeKind::Error); // just to show
    /// println!("{:#?}", tree);
    /// ```
    pub(crate) fn render(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result {
        write!(f, "{tab}")?;
        if let Some(name) = self.name {
            write!(f, "{name} = ")?;
        }
        write!(f, "{}", self.kind.name())?;
        for child in &self.children {
            writeln!(f)?;
            child.value.render(f, &format!("{tab}    "))?;
            if matches!(child.value, Child::Token(..)) {
                write!(f, " @ ")?;
                write!(f, "{:?}", child.span)?;
            }
        }
        Ok(())
    }
}

impl Child {
    /// Uses the [std::fmt::Formatter] to write a pretty-printed tree in the terminal for debug
    /// porpuses.
    ///
    /// It usually likes like the following printed code:
    /// ```txt
    /// EXPR_BINARY
    ///     LIT_INT8
    ///         '1' @ 0..1
    ///     '+' @ 2..3
    ///     LIT_INT8
    ///         '1' @ 4..5
    /// @ 0..5
    /// ```
    ///
    /// The use of this code is like the following code, you should never use directly this function
    /// since its local:
    /// ```
    /// let tree = Tree::new(TreeKind::Error); // just to show
    /// println!("{:#?}", tree);
    /// ```
    pub(crate) fn render(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result {
        match self {
            Child::Tree(tree) => tree.render(f, tab),
            Child::Token(token) => token.render(f, tab),
        }
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f, "")?;
        writeln!(f)?; // Write the newline in the end of the tree
        Ok(())
    }
}
