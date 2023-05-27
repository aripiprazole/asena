use std::iter::Peekable;

use thiserror::Error;

use crate::ast::{App, Binary, Expr, ExprRef, FunctionId, GlobalId, Literal};
use crate::lexer::Token;
use crate::span::Spanned;

pub type TokenRef = Spanned<Token>;

pub type StringRef = Spanned<String>;

pub type Result<T, E = Spanned<ParseError>> = std::result::Result<T, E>;

/// Parsing errors, it can be indexed to the code using a [Spanned<ParseError>].
#[derive(Error, Debug, Clone, Copy)]
pub enum ParseError {
    #[error("Unexpected token at this position.")]
    UnexpectedToken,

    #[error("Could not parse primary, but expected it.")]
    CantParsePrimary,
}

/// The language parser struct, it takes a [Token] iterator, that can be lazy or eager initialized
/// to advance and identify tokens on the programming language.
pub struct Parser<'a, S: Iterator<Item = Spanned<Token>>> {
    pub source: &'a str,
    pub index: usize,
    pub stream: Peekable<S>,
}

impl<'a, S: Iterator<Item = Spanned<Token>>> Parser<'a, S> {
    /// Creates a new instance of the Parser, it takes the source code reference, and a lexer stream
    /// peekable.
    ///
    /// It does require a lazy parser.
    pub fn new(source: &'a str, stream: Peekable<S>) -> Self {
        Self {
            index: 0,
            source,
            stream,
        }
    }

    //>>>Parser functions
    /// Parses a reference to [Expr]
    pub fn expr(&mut self) -> Result<ExprRef> {
        self.binary()
    }

    /// Parses a reference to [Binary]
    pub fn binary(&mut self) -> Result<ExprRef> {
        let mut lhs = self.app()?;

        loop {
            let next = self.next();

            let Token::Symbol(symbol) = next.value() else {
                break;
            };

            let fn_id = Spanned::new(next.span().clone(), FunctionId::new(symbol));
            let rhs = self.app()?;

            // Combines two locations
            let span = lhs.span.start..rhs.span.end;

            lhs = ExprRef::new(span, Expr::Binary(Binary { lhs, fn_id, rhs }))
        }

        Ok(lhs)
    }

    /// Parses a reference to [App]
    pub fn app(&mut self) -> Result<ExprRef> {
        let mut callee = self.primary()?;

        while let Some(argument) = self.catch(Parser::primary)? {
            // Combines two locations
            let span = callee.span.start..argument.span.end;

            callee = ExprRef::new(span, Expr::App(App { callee, argument }))
        }

        Ok(callee)
    }

    /// Parses a reference to [Literal] or primary [Expr]
    pub fn primary(&mut self) -> Result<ExprRef> {
        use Token::*;

        let current = self.peek();
        let value = match current.value() {
            // Booleans
            True => Expr::Literal(Literal::True),
            False => Expr::Literal(Literal::False),

            // Integers
            Int8(n, signed) => Expr::Literal(Literal::Int8(*n, *signed)),
            Int16(n, signed) => Expr::Literal(Literal::Int16(*n, *signed)),
            Int32(n, signed) => Expr::Literal(Literal::Int32(*n, *signed)),
            Int64(n, signed) => Expr::Literal(Literal::Int64(*n, *signed)),
            Int128(n, signed) => Expr::Literal(Literal::Int128(*n, *signed)),

            // Floating pointers
            Float32(n) => Expr::Literal(Literal::Float32(*n)),
            Float64(n) => Expr::Literal(Literal::Float64(*n)),

            // String
            String(content) => {
                // Remove the `"` tokens of the string, they start with 1 gap in the start and in
                // the end of the content.
                let content = content[1..(content.len() - 1)].to_string();

                Expr::Literal(Literal::String(content))
            }

            // Starts with a Global expression, and its needed to be resolved in a further step, it
            // can be either a [Global] or a [Local].
            Ident(..) => {
                // skip <identifier>
                //
                // It does not uses the Ident(..) pattern, because of the location, we need locality
                // of the ast.
                let ident = self.identifier()?.map(|s| FunctionId::new(&s));

                // Creates a new path.
                let mut path = vec![ident];
                while let Token::Dot = self.peek().value() {
                    self.next(); // skip `.`
                    let fn_id = self.identifier()?.map(|s| FunctionId::new(&s));
                    path.push(fn_id); // adds new `.` <identifier>
                }

                // Creates a new location combining the first, and the last points in the source code
                let a = path.first().unwrap().span();
                let b = path.last().map(Spanned::span).unwrap_or(a);

                return Ok(ExprRef::new(a.start..b.end, Expr::Global(GlobalId(path))));
            }

            //>>>Composed tokens
            // Group expression
            LeftParen => {
                self.next(); // skip '('
                let expr = self.expr()?;
                self.expect(Token::RightParen)?; // consumes ')'

                return Ok(current.swap(Expr::Group(expr)));
            }

            // Help expression
            Help => {
                self.next(); // skip '?'
                let expr = self.expr()?;

                return Ok(current.swap(Expr::Help(expr)));
            }
            _ => return self.end_diagnostic(ParseError::CantParsePrimary),
        };

        self.next(); // Skips if hadn't any error

        Ok(current.swap(value))
    }

    /// Pares a valid identifier, and return it's content.
    fn identifier(&mut self) -> Result<StringRef> {
        self.eat(|next| match next.value() {
            Token::Ident(content) => Some(next.replace(content.clone())),

            // Accepts symbol, so the parser is able to parse something like `Functor.<$>`
            Token::Symbol(content) => Some(next.replace(content.clone())),
            _ => None,
        })
    }

    /// Eat a matching token, and return it if matching correctly.
    fn expect(&mut self, token: Token) -> Result<TokenRef> {
        self.eat(|next| {
            if next.value() == &token {
                Some(next.clone())
            } else {
                None
            }
        })
    }

    /// Tries to parse using a function [F], but it can't, the index would not be increased, so the
    /// result of the function is going to be Ok(None); but if everything is ok, the result is going
    /// to be the parsed value.
    fn catch<T, F>(&mut self, mut f: F) -> Result<Option<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let current_index = self.index;

        match f(self) {
            Ok(value) => Ok(Some(value)),
            Err(..) if self.index == current_index => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Peeks the current token using a function [F], and jumps to the next token.
    fn eat<T, F>(&mut self, f: F) -> Result<T>
    where
        F: Fn(&TokenRef) -> Option<T>,
    {
        let next = self.peek();
        match f(&next) {
            Some(value) => {
                self.next();
                Ok(value)
            }
            None => Err(next.swap(ParseError::UnexpectedToken)),
        }
    }

    /// Jumps to the next token, and increases the current token index.
    fn next(&mut self) -> TokenRef {
        self.index += 1;

        self.stream.next().unwrap()
    }

    /// End the diagnostic with an error of [ParseError], spanned with the current token location.
    fn end_diagnostic<T>(&mut self, error: ParseError) -> Result<T, Spanned<ParseError>> {
        Err(self.stream.peek().unwrap().replace(error))
    }

    /// Sees the current token, and return it cloned.
    fn peek(&mut self) -> Spanned<Token> {
        self.stream.peek().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, span::Loc};

    use super::*;

    #[test]
    fn it_works() {
        let code = "(person + 10).batata 10 10";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.run_diagnostic(Parser::expr))
    }

    impl<'a, S: Iterator<Item = Spanned<Token>>> Parser<'a, S> {
        fn run_diagnostic<F, T>(&mut self, f: F) -> T
        where
            F: Fn(&mut Self) -> Result<T>,
        {
            use ariadne::{Color, Label, Report, ReportKind, Source};

            match f(self) {
                Ok(value) => value,
                Err(err) => {
                    Report::<Loc>::build(ReportKind::Error, (), 0)
                        .with_message(err.value().to_string())
                        .with_label(
                            Label::new(err.span().clone())
                                .with_message(err.value().to_string())
                                .with_color(Color::Red),
                        )
                        .finish()
                        .print(Source::from(self.source.clone()))
                        .unwrap();

                    panic!("Running diagnostic");
                }
            }
        }
    }
}
