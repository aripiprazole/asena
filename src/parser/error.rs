use thiserror::Error;

use crate::ast::node::TokenKind;
use crate::lexer::span::Spanned;
use crate::report::InternalError;

pub type Result<T, E = Spanned<ParseError>> = std::result::Result<T, E>;

/// Parsing errors, it can be indexed to the code using a [Spanned<ParseError>].
#[derive(Error, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ParseError {
    #[error("The stack should contain just the tree element but has {} elements", .0)]
    StackError(usize),

    #[error("The token stream still contain something: `{}`", .0)]
    StreamStillContainElements(TokenKind),

    #[error("Unexpected token")]
    UnexpectedTokenError,

    #[error("Missing semicolon")]
    MissingSemiError,

    #[error("Expression should be surrounded by parenthesis: `({} ..)`", .0)]
    PrimarySurroundedError(TokenKind),

    #[error("Found an `else` without previous `if` node")]
    DanglingElseError,

    #[error("Found unicode `{}`, you can rewrite in the language norm as {}", .0, .1)]
    UnicodeError(TokenKind, &'static str),

    #[error("Reserved keyword `{}` in the wrong position, must be a constraint", .0)]
    ConstraintReservedKeywordError(TokenKind),

    #[error("Reserved keyword `{}` in the wrong position, must be a statement", .0)]
    StmtReservedKeywordError(TokenKind),

    #[error("Reserved keyword `{}` in the wrong position, must be a top-level declaration", .0)]
    DeclReservedKeywordError(TokenKind),

    #[error("Reserved keyword `{}` to be used in a feature, you can use like: `{}_`", .0, .0)]
    ReservedKeywordError(TokenKind),

    #[error("Invalid identifier, found symbol")]
    InvalidSymbolIdentifierError,

    #[error("Invalid identifier, found unicode symbol")]
    InvalidUnicodeIdentifierError,

    #[error("Expected token: `{}`. But got this instead", .0.to_string())]
    ExpectedTokenError(TokenKind),

    #[error("Could not parse primary")]
    PrimaryExpectedError,

    #[error("Could not parse pattern")]
    PatExpectedError,

    #[error("Could not parse statement")]
    StmtExpectedError,

    #[error("Could not parse anything, found end of file")]
    EofError,

    #[error("Expected signature parameter")]
    ParameterExpectedError,

    #[error("Unfinished parenthesis, expected `)`")]
    UnfinishedParenError,

    #[error("Unfinished brackets, expected `]`")]
    UnfinishedBracketError,

    #[error("Unfinished block, expected `}}`")]
    UnfinishedBraceError,

    #[error("Expected Σ expression or [<expr>] array expression")]
    ExpectedBracketExprError,

    #[error("Expected Π expression or (<expr>) group expression")]
    ExpectedParenExprError,

    #[error("Useless semicolon here, you can just ignore it")]
    UeselessSemiError,
}

impl ParseError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl InternalError for ParseError {
    fn code(&self) -> u16 {
        self.discriminant() as u16
    }

    fn kind() -> crate::report::DiagnosticKind {
        crate::report::DiagnosticKind::Error
    }
}
