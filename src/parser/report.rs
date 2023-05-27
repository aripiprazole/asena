use crate::lexer::span::{Loc, Spanned};
use crate::lexer::token::Token;
use crate::parser::error::{ParseError, Tip};

use super::error::Result;
use super::Parser;

impl<'a, S: Iterator<Item = Spanned<Token>>> Parser<'a, S> {
    /// Runs the parser, and if it fails, prints the error using a report crate. Returns Some(value)
    /// if the parsing is correct.
    pub fn run_diagnostic<F, T>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&mut Self) -> Result<T>,
    {
        use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};

        let mut colors = ColorGenerator::new();

        match f(self) {
            Ok(value) => Some(value),
            Err(err) => {
                let errors = match err.value().clone() {
                    ParseError::Many(_, errors) => errors,
                    otherwise => vec![Tip::Error(otherwise)],
                };

                Report::<Loc>::build(ReportKind::Error, (), 0)
                    .with_code(format!("E0{:X}", err.value().discriminant()))
                    .with_message(err.value().to_string())
                    .with_labels(errors.into_iter().map(|reason| {
                        Label::new(err.span().clone())
                            .with_message(reason.to_string())
                            .with_color(match reason {
                                Tip::Error(..) => Color::Red,
                                _ => colors.next(),
                            })
                    }))
                    .finish()
                    .print(Source::from(self.source.clone()))
                    .unwrap();

                None
            }
        }
    }
}
