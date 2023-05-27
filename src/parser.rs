use std::iter::Peekable;

use chumsky::span::SimpleSpan;

use crate::{
    ast::{Expr, ExprRef, FunctionId, GlobalId, Literal},
    lexer::{Loc, Spanned, Token},
};

pub type ParseableToken = (Token, SimpleSpan);

pub type TokenRef = Spanned<Token>;

pub type StringRef = Spanned<String>;

#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    UnexpectedToken,
    CantParsePrimary,
}

pub type Result<T, E = Spanned<ParseError>> = std::result::Result<T, E>;

/// The language parser struct, it takes a [Token] iterator, that can be lazy or eager initialized
/// to advance and identify tokens on the programming language.
pub struct Parser<'a, S: Iterator<Item = ParseableToken>> {
    pub source: &'a str,
    pub index: Vec<Loc>,
    pub stream: Peekable<S>,
}

impl<'a, S: Iterator<Item = ParseableToken>> Parser<'a, S> {
    pub fn new(source: &'a str, stream: Peekable<S>) -> Self {
        Self {
            index: Default::default(),
            source,
            stream,
        }
    }

    pub fn expr(&mut self) -> Result<ExprRef> {
        todo!()
    }

    pub fn primary(&mut self) -> Result<ExprRef> {
        use Token::*;

        let current = self.next();
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
            Ident(ident) => {
                let mut path = vec![FunctionId::new(ident)];
                while let Token::Dot = self.peek().value() {
                    self.next(); // skip `.`
                    let identifier = self.identifier()?;
                    path.push(FunctionId::new(identifier.value())); // adds new `.` <identifier>
                }

                Expr::Global(GlobalId(path))
            }

            //>>>Composed tokens
            // Group expression
            LeftParen => {
                // skip '('
                let expr = self.expr()?;
                self.expect(Token::RightParen)?; // consumes ')'

                Expr::Group(expr)
            }

            // Help expression
            Help => {
                // skip '?'
                let expr = self.expr()?;

                Expr::Help(expr)
            }
            _ => return self.end_diagnostic(ParseError::CantParsePrimary),
        };

        Ok(current.swap(value))
    }

    fn symbol(&mut self) -> Result<StringRef> {
        self.eat(|next| match next.value() {
            Token::Symbol(content) => Some(next.replace(content.clone())),
            _ => None,
        })
    }

    fn identifier(&mut self) -> Result<StringRef> {
        self.eat(|next| match next.value() {
            Token::Ident(content) => Some(next.replace(content.clone())),

            // Accepts symbol, so the parser is able to parse something like `Functor.<$>`
            Token::Symbol(content) => Some(next.replace(content.clone())),
            _ => None,
        })
    }

    fn expect(&mut self, token: Token) -> Result<TokenRef> {
        self.eat(|next| {
            if next.value() == &token {
                Some(next.clone())
            } else {
                None
            }
        })
    }

    fn eat<T, F>(&mut self, f: F) -> Result<T>
    where
        F: Fn(&TokenRef) -> Option<T>,
    {
        let next = self.next();
        match f(&next) {
            Some(value) => Ok(value),
            None => Err(next.swap(ParseError::UnexpectedToken)),
        }
    }

    fn next(&mut self) -> TokenRef {
        self.stream
            .next()
            .map(|(token, span)| Spanned::new(span.into_range(), token))
            .unwrap()
    }

    fn end_diagnostic<T>(&mut self, error: ParseError) -> Result<T, Spanned<ParseError>> {
        Err(self
            .stream
            .peek()
            .map(|(_, span)| Spanned::new(span.into_range(), error))
            .unwrap())
    }

    fn peek(&mut self) -> Spanned<Token> {
        self.stream
            .peek()
            .map(|(value, span)| Spanned::new(span.into_range(), value.clone()))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let code = "x + 10";
        let (tokens, ..) = {
            use chumsky::Parser; // use parser locally

            crate::lexer::lexer().parse(code).into_output_errors()
        };

        let stream = tokens.unwrap_or_default().into_iter().peekable();
        let parser = Parser::new(code, stream);
    }
}
