use std::fmt::{Debug, Display};

pub use super::named::*;
use super::spec::{Node, Spec, Terminal};
pub use super::token::*;

#[derive(Clone)]
pub struct Tree {
    pub kind: TreeKind,
    pub children: Vec<Spanned<Child>>,
}

#[derive(Debug, Clone)]
pub enum Child {
    Tree(Tree),
    Token(Token),
}

#[derive(Debug, Clone)]
pub enum TreeKind {
    Error,

    File,

    LitNat,
    LitInt8,
    LitUInt8,
    LitInt16,
    LitUInt16,
    LitInt32,
    LitUInt32,
    LitInt64,
    LitUInt64,
    LitInt128,
    LitUInt128,

    LitFloat32,
    LitFloat64,

    LitTrue,
    LitFalse,

    LitSymbol,
    LitIdentifier,
    LitString,

    ExprGroup,
    ExprBinary,
    ExprAcessor,
    ExprApp,
    ExprDsl,
    ExprArray,
    ExprLam,
    ExprLet,
    ExprGlobal,
    ExprLocal,
    ExprLit,
    ExprAnn,
    ExprQual,
    ExprPi,
    ExprSigma,
    ExprHelp,

    PatWildcard,
    PatSpread,
    PatLiteral,
    PatLocal,
    PatConstructor,
    PatList,

    StmtAsk,
    StmtLet,
    StmtReturn,
    StmtExpr,

    Binding,

    BodyValue,
    BodyDo,

    Parameter,

    DeclSignature,
    DeclAssign,
    DeclCommand,
    DeclClass,
    DeclInstance,

    Constraint,

    Field,
    Method,

    TypeInfer,
    Type,
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
        let child = self.children.get(nth).unwrap(); // TODO
        match &child.value {
            Child::Tree(tree) => T::spec(child.replace(tree.clone())),
            Child::Token(..) => Node::empty(),
        }
    }

    pub fn terminal<T: Terminal>(&self, nth: usize) -> Node<Spanned<T>> {
        let child = self.children.get(nth).unwrap(); // TODO
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
    fn render(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result {
        match self {
            Child::Tree(tree) => tree.render(f, tab),
            Child::Token(token) => token.render(f, tab),
        }
    }
}

impl Named for TreeKind {}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f, "")?;
        writeln!(f)?; // Write the newline in the end of the tree
        Ok(())
    }
}

impl Display for TreeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

macro_rules! ast_enum {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $(
                $(#[$field_outer:meta])*
                $variant:ident <- $kind:expr
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone)]
        pub enum $name {
            $(
                $(#[$field_outer])*
                $variant($variant),
            )*
        }

        impl $name {
            #[allow(dead_code)]
            #[allow(path_statements)]
            #[allow(clippy::no_effect)]
            fn __show_type_info() {
                $($kind;)*
            }
        }
    }
}

pub(crate) use ast_enum;

use crate::lexer::span::Spanned;
