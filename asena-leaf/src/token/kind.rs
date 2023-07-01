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
    DefaultKeyword,  // default
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
