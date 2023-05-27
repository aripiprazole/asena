use std::iter::Peekable;

use crate::ast::*;
use crate::parser::error::ParseError;
use crate::span::Spanned;
use crate::token::Token;

use self::error::Result;

pub type TokenRef = Spanned<Token>;

pub type StringRef = Spanned<String>;

pub mod error;

pub mod report;
pub mod support;

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

    /// Parses a reference to [Expr]
    pub fn expr(&mut self) -> Result<ExprRef> {
        self.binary()
    }

    /// Parses a reference to [Binary]
    pub fn binary(&mut self) -> Result<ExprRef> {
        let mut lhs = self.ann()?;

        while let Ok(fn_id) = self.operator() {
            let fn_id = fn_id.map(FunctionId);
            let rhs = self.ann()?;

            // Combines two locations
            let span = lhs.span.start..rhs.span.end;

            lhs = ExprRef::new(span, Expr::Binary(Binary { lhs, fn_id, rhs }))
        }

        Ok(lhs)
    }

    /// Parses a reference to [Ann]
    pub fn ann(&mut self) -> Result<ExprRef> {
        let mut value = self.accessor()?;

        while let Token::Symbol(fn_id) = self.peek().value() {
            // Currently, is impossible to pattern match agains't a [String], so it's the workaround
            if fn_id != ":" {
                break;
            }

            self.next(); // skips ':'

            let against = self.accessor()?;

            // Combines two locations
            let span = value.span.start..against.span.end;

            value = ExprRef::new(span, Expr::Ann(Ann { value, against }))
        }

        Ok(value)
    }

    /// Parses a reference to [Accessor]
    pub fn accessor(&mut self) -> Result<ExprRef> {
        let mut receiver = self.app()?;

        while let Token::Dot = self.peek().value() {
            self.next(); // skips '.'

            let accessor = self.identifier()?.map(FunctionId);

            // Combines two locations
            let span = receiver.span.start..accessor.span.end;

            receiver = ExprRef::new(
                span,
                Expr::Accessor(Accessor {
                    receiver,
                    accessor: LocalId(accessor),
                }),
            )
        }

        Ok(receiver)
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

    /// Parses a valid identifier, and return it's content.
    fn identifier(&mut self) -> Result<StringRef> {
        self.eat(|next| match next.value() {
            Token::Ident(content) => Some(next.replace(content.clone())),

            // Accepts symbol, so the parser is able to parse something like `Functor.<$>`
            Token::Symbol(content) => Some(next.replace(content.clone())),
            _ => None,
        })
    }

    /// Parses a valid binary operator, and return it's content.
    fn operator(&mut self) -> Result<StringRef> {
        self.eat(|next| match next.value() {
            Token::Symbol(content) => Some(next.replace(content.clone())),
            _ => None,
        })
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
            // Can parse the following expressions
            // * [Group]
            // * [Pi]
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
            _ => {
                return self
                    .end_diagnostic(ParseError::CantParsePrimary)
                    .map_err(|error| error.with_error(ParseError::UnexpectedToken))
            }
        };

        self.next(); // Skips if hadn't any error

        Ok(current.swap(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn it_works() {
        let code = "|> a b";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.run_diagnostic(Parser::expr))
    }
}
