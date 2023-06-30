use std::fmt::Display;

use asena_span::Spanned;

use crate::node::HasTokens;

use super::named::Named;

#[derive(Debug, Clone, Hash, Default)]
pub struct Text {
    pub before_whitespace: String,
    pub code: String,
}

#[derive(Debug, Clone, Hash, Default)]
pub struct Token {
    pub name: Option<&'static str>,
    pub kind: TokenKind,
    pub text: String,
    pub full_text: Text,
}

impl HasTokens for Spanned<Token> {
    fn tokens(&self) -> Vec<Spanned<Token>> {
        vec![self.clone()]
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenKind {
    #[default]
    Error,

    Nat,

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
    FunKeyword,      // fun
    SelfKeyword,     // self

    // unicode
    LambdaUnicode, // λ
    ForallUnicode, // ∀
    PiUnicode,     // Π
    SigmaUnicode,  // Σ

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
    HelpSymbol,   // ?
    EqualSymbol,  // =
    HashSymbol,   // #

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
    Str,

    // end of file
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind, text: &str) -> Self {
        Self {
            name: None,
            kind,
            text: text.into(),
            full_text: Default::default(),
        }
    }

    pub fn eof() -> Self {
        Self {
            name: None,
            kind: TokenKind::Eof,
            text: Default::default(),
            full_text: Default::default(),
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
        write!(f, "'{}'", self.text)
    }
}

impl Named for TokenKind {}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.before_whitespace)?;
        write!(f, "{}", self.code)?;
        Ok(())
    }
}
