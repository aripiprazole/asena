use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenKind {
    Error,

    // keywords
    LetKeyword,      // let
    TrueKeyword,     // true
    FalseKeyword,    // false
    IfKeyword,       // if
    ElseKeyword,     // else
    ThenKeyword,     // then
    TypeKeyword,     // type
    RecordKeyword,   // record
    ReturnKeyword,   // return
    EnumKeyword,     // enum
    TraitKeyword,    // trait
    ClassKeyword,    // class
    CaseKeyword,     // case
    WhereKeyword,    // where
    MatchKeyword,    // match
    UseKeyword,      // use
    InstanceKeyword, // instance
    InKeyword,       // in

    // unicode
    Lambda, // λ
    Forall, // ∀
    Pi,     // Π
    Sigma,  // Σ

    // control symbols
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    Comma,        // ,
    Semi,         // ;
    Colon,        // :
    Dot,          // .
    Help,         // ?
    Equal,        // =

    DoubleArrow, // =>
    RightArrow,  // ->
    LeftArrow,   // <-

    // integers
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Int128,
    UInt128,

    // floats
    Float32,
    Float64,

    // literals
    Symbol,
    Identifier,
    String,

    // end of file
    Eof,
}

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
    TreeEof,

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

    pub fn child<T: TryFrom<Child>>(&self, _name: &str) -> T {
        todo!()
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
    fn render(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result {
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

impl Token {
    pub fn new(kind: TokenKind, text: &str) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }

    pub fn eof() -> Self {
        Self {
            kind: TokenKind::Eof,
            text: Default::default(),
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
    fn render(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result {
        write!(f, "{tab}")?;
        write!(f, "'{}'", self.text)
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

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f, "")?;
        writeln!(f)?; // Write the newline in the end of the tree
        Ok(())
    }
}

impl TreeKind {
    /// Transforms the token's kind name into a upper case underline splitted string. It goes
    /// something like:
    ///
    /// It does transforms the text: `ExprPrimary` into `EXPR_PRIMARY`
    pub fn name(&self) -> String {
        self.to_string()
            .chars()
            .enumerate()
            .flat_map(|(i, char)| {
                if char.is_uppercase() && i > 0 {
                    vec!['_', char]
                } else {
                    vec![char]
                }
            })
            .collect::<String>()
            .to_uppercase()
    }
}

impl TokenKind {
    /// Transforms the token's kind name into a upper case underline splitted string. It goes
    /// something like:
    ///
    /// It does transforms the text: `LitInt8` into `LIT_INT8`
    pub fn name(&self) -> String {
        self.to_string()
            .chars()
            .flat_map(|char| {
                if char.is_uppercase() {
                    vec!['_', char]
                } else {
                    vec![char]
                }
            })
            .collect::<String>()
    }
}

impl Display for TreeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

macro_rules! ast_node {
    (
        $(#[$outer:meta])*
        pub struct $name:ident;
    ) => {
        ast_node! {
            $(#[$outer:meta])* pub struct $name {}
        }
    };

    (
        $(#[$outer:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_outer:meta])*
                $vis:vis $field:ident: $field_type:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
                $(#[$field_outer])*
                $vis $field: $field_type
            ),*
        }
    };
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
            pub fn name() {
            }
        }
    }
}

pub(crate) use ast_enum;
pub(crate) use ast_node;

use crate::lexer::span::Spanned;
