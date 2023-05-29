use thiserror::Error;

use crate::lexer::span::{Loc, Spanned};
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

    #[error("Expected Σ expression or [<expr>] array expression")]
    ExpectedBracketExpr,

    #[error("Expected Π expression or (<expr>) group expression")]
    ExpectedParenthesisExpr,

    #[error("{0}")]
    Many(Box<ParseError>, Vec<Tip>),
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Tip {
    #[error("{}", .0)]
    Error(ParseError),

    #[error("{}", .0.value())]
    Spanned(Spanned<ParseError>),

    #[error("Maybe add a semicolon here")]
    MaybeSemi(Spanned<Token>),

    #[error("No tips")]
    NoTips,
}

impl Spanned<ParseError> {
    pub fn with_error(&self, new_error: ParseError) -> Self {
        self.with_tip(Tip::Error(new_error))
    }

    pub fn many(&self) -> Vec<Tip> {
        match self.value() {
            ParseError::Many(_, errors) => errors
                .iter()
                .cloned()
                .flat_map(|error| match error {
                    Tip::Spanned(error) => error.many(),
                    _ => vec![error],
                })
                .collect(),
            _ => vec![Tip::Spanned(self.clone())],
        }
    }

    pub fn with_spanned(&self, new_error: Spanned<ParseError>) -> Self {
        // If the structure is the same, there's no need to duplicate the tip.
        if self == &new_error {
            return self.clone();
        }

        self.with_tip(Tip::Spanned(new_error))
    }

    pub fn with_tip(&self, new_tip: Tip) -> Self {
        use ParseError::*;

        self.clone().map(move |error| match error {
            Many(actual, mut reasons) => {
                reasons.push(new_tip.clone());

                Many(actual, reasons)
            }
            _ => Many(error.into(), vec![new_tip.clone()]),
        })
    }
}

impl Tip {
    pub fn span(&self, default: Loc) -> Loc {
        match self {
            Tip::Spanned(spanned) => spanned.span().clone(),
            _ => default,
        }
    }
}

impl ParseError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
