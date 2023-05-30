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

    ExprBinary,
    ExprAcessor,
    ExprApp,
    ExprDsl,
    ExprArray,
    ExprLam,
    ExprLet,
    ExprAnn,
    ExprQual,
    ExprPi,
    ExprSigma,

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
