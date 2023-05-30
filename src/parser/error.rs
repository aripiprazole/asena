use thiserror::Error;

use crate::lexer::span::Spanned;
use crate::lexer::token::Token;

pub type Result<T, E = Spanned<ParseError>> = std::result::Result<T, E>;

/// Parsing errors, it can be indexed to the code using a [Spanned<ParseError>].
#[derive(Error, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ParseError {
    #[error("Unexpected token")]
    UnexpectedToken,

    #[error("Missing semicolon")]
    MissingSemi,

    #[error("Invalid identifier, found symbol")]
    SymbolInsteadOfIdentifier,

    #[error("Invalid identifier, found unicode symbol")]
    UnicodeInsteadOfIdentifier,

    #[error("Expected token: `{}`. But got this instead", .0.to_string())]
    Expected(Token),

    #[error("Could not parse primary")]
    CantParsePrimary,

    #[error("Could not parse pattern")]
    CantParsePattern,

    #[error("Could not parse statement")]
    CantParseStatement,

    #[error("Could not parse anything, found end of file")]
    CantParseDueToEof,

    #[error("Expected signature parameter")]
    ExpectedParameter,

    #[error("Unfinished parenthesis, expected `)`")]
    UnfinishedParenthesis,

    #[error("Unfinished brackets, expected `]`")]
    UnfinishedBrackets,

    #[error("Unfinished block, expected `}}`")]
    UnfinishedBlock,

    #[error("Expected Σ expression or [<expr>] array expression")]
    ExpectedBracketExpr,

    #[error("Expected Π expression or (<expr>) group expression")]
    ExpectedParenthesisExpr,

    #[error("Useless semicolon here, you can just ignore it")]
    UeselessSemi,
}

impl ParseError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
