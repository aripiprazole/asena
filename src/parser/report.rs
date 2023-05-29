use crate::lexer::span::{Loc, Spanned};
use crate::lexer::token::Token;
use crate::parser::error::Tip;

use super::error::Result;
use super::Parser;

impl<'a, S: Iterator<Item = Spanned<Token>> + Clone> Parser<'a, S> {
    /// Runs the parser, and if it fails, prints the error using a report crate. Returns Some(value)
    /// if the parsing is correct.
    pub fn diagnostic<F, T>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&mut Self) -> Result<T>,
    {
        use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};

        let mut colors = ColorGenerator::new();

        match f(self) {
            Ok(value) => Some(value),
            Err(main_error) => {
                let errors = main_error.many();

                Report::<Loc>::build(ReportKind::Error, (), 0)
                    .with_code(format!("E0{:X}", main_error.value().discriminant()))
                    .with_message(main_error.value().to_string())
                    .with_labels(errors.into_iter().map(|reason| {
                        Label::new(reason.span(main_error.span().clone()))
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
