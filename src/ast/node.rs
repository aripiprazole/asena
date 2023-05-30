use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenKind {
    Error,

    // keywords
    Let,    // let
    True,   // true
    False,  // false
    If,     // if
    Else,   // else
    Then,   // then
    Type,   // type
    Record, // record
    Return, // return
    Enum,   // enum
    Trait,  // trait
    Class,  // class
    Case,   // case
    Where,  // where
    Match,  // match
    Use,    // use
    In,     // in

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

    DoubleArrow,  // =>
    Arrow,        // ->
    InverseArrow, // <-

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

#[derive(Debug, Clone)]
pub struct Tree {
    pub kind: TreeKind,
    pub children: Vec<Child>,
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
    TypeExplicit,
}

impl Tree {
    pub fn child<T: From<Child>>(&self, name: &str) -> T {
        todo!()
    }
}

impl Token {
    pub fn eof() -> Self {
        Self {
            kind: TokenKind::Eof,
            text: Default::default(),
        }
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
                $(
                    $kind;
                )*
            }
        }
    }
}

pub(crate) use ast_enum;
pub(crate) use ast_node;
