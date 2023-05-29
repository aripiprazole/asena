use std::fmt::Debug;

use crate::lexer::span::{Loc, Spanned};
use crate::lexer::token::Token;
use crate::parser::error::Tip;

use super::error::{ParseError, Result};
use super::Parser;

impl<'a, S: Iterator<Item = Spanned<Token>> + Clone> Parser<'a, S> {
    pub(crate) fn report(&mut self, error: Spanned<ParseError>) {
        self.tip(Tip::Spanned(error))
    }

    pub(crate) fn warn(&mut self, error: Spanned<ParseError>) {
        self.tip(Tip::Warning(error))
    }

    pub(crate) fn tip(&mut self, error: Tip) {
        self.errors.push(error)
    }

    /// Runs the parser, and if it fails, prints the error using a report crate. Returns Some(value)
    /// if the parsing is correct.
    pub fn diagnostic<F, T>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&mut Self) -> Result<T>,
        T: Debug,
    {
        use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};

        let mut colors = ColorGenerator::new();

        match f(self) {
            Ok(value) if self.errors.is_empty() => Some(value),
            Ok(value) => {
                let errors = self.errors.clone(); // get the reported resilient errors

                Report::<Loc>::build(ReportKind::Error, (), 0)
                    .with_code("E00")
                    .with_message("Recovered tree with errors")
                    .with_labels(errors.into_iter().map(|reason| {
                        Label::new(reason.span(0..0))
                            .with_message(reason.to_string())
                            .with_color(match reason {
                                Tip::Error(..) => Color::Red,
                                _ => colors.next(),
                            })
                    }))
                    .finish()
                    .print(Source::from(self.source.clone()))
                    .unwrap();

                println!();
                println!("Recovered AST");
                println!();
                println!("{value:#?}");
                println!();

                None
            }
            Err(main_error) => {
                let mut errors = main_error.many();
                errors.extend(self.errors.clone()); // get the reported resilient errors

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
