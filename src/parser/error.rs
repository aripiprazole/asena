use thiserror::Error;

use crate::span::Spanned;
use crate::token::Token;

pub type Result<T, E = Spanned<ParseError>> = std::result::Result<T, E>;

/// Parsing errors, it can be indexed to the code using a [Spanned<ParseError>].
#[derive(Error, Debug, Clone)]
#[repr(u8)]
pub enum ParseError {
    #[error("Unexpected token at this position")]
    UnexpectedToken,

    #[error("Expected token: {0}. But got this instead")]
    Expected(Token),

    #[error("Could not parse primary")]
    CantParsePrimary,

    #[error("{0}")]
    Many(Box<ParseError>, Vec<Tip>),
}

#[derive(Error, Debug, Clone)]
pub enum Tip {
    #[error("{0}")]
    Error(ParseError),

    #[error("No tips")]
    NoTips,
}

impl Spanned<ParseError> {
    pub fn with_error(&self, new_error: ParseError) -> Self {
        self.with_tip(Tip::Error(new_error))
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

impl ParseError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
