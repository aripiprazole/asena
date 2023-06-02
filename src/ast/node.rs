use std::fmt::Debug;

use super::spec::{Node, Spec, Terminal};
use crate::lexer::span::Spanned;

pub use super::kind::*;
pub(crate) use super::macros::ast_enum;
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
#[derive(Clone, Hash)]
pub struct Tree {
    pub kind: TreeKind,
    pub children: Vec<Spanned<Child>>,
}

/// Polymorphic variants of [Token] and [Tree], it can and must be used as an abstract to them.
#[derive(Debug, Clone, Hash)]
pub enum Child {
    Tree(Tree),
    Token(Token),
}

impl Tree {
    pub fn new(kind: TreeKind) -> Self {
        Self {
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

    pub fn child<T: TryFrom<Child>>(&self, _name: &str) -> Option<T> {
        todo!()
    }

    pub fn at<T: Spec>(&self, nth: usize) -> Node<Spanned<T>> {
        let Some(child) = self.children.get(nth) else {
            return Node::empty();
        };

        match &child.value {
            Child::Tree(tree) => T::spec(child.replace(tree.clone())),
            Child::Token(..) => Node::empty(),
        }
    }

    pub fn terminal<T: Terminal>(&self, nth: usize) -> Node<Spanned<T>> {
        let Some(child) = self.children.get(nth) else {
            return Node::empty();
        };

        match &child.value {
            Child::Tree(..) => Node::empty(),
            Child::Token(token) => T::spec(child.replace(token.clone())),
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
