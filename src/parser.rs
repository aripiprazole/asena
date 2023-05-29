use std::iter::Peekable;

use crate::lexer::span::Spanned;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;
use crate::{ast::*, lexer::span::Span};

use self::error::{Result, Tip};

pub type TokenRef = Spanned<Token>;

pub type StringRef = Spanned<String>;

pub mod error;

pub mod report;
pub mod support;

/// The language parser struct, it takes a [Token] iterator, that can be lazy or eager initialized
/// to advance and identify tokens on the programming language.
#[derive(Clone)]
pub struct Parser<'a, S: Iterator<Item = Spanned<Token>> + Clone> {
    pub errors: Vec<Tip>,
    pub source: &'a str,
    pub index: usize,
    pub stream: Peekable<S>,
}

impl<'a, S: Iterator<Item = Spanned<Token>> + Clone> Parser<'a, S> {
    /// Creates a new instance of the Parser, it takes the source code reference, and a lexer stream
    /// peekable.
    ///
    /// It does require a lazy parser.
    pub fn new(source: &'a str, stream: Peekable<S>) -> Self {
        Self {
            index: 0,
            errors: vec![],
            source,
            stream,
        }
    }

    /// Parses a reference to [Decl]
    pub fn decl(&mut self) -> Result<DeclRef> {
        let start = self.measure();
        let decl = self.sig_decl();
        self.expect_semi(start)?;
        decl
    }

    /// Parses a reference to [Signature]
    pub fn sig_decl(&mut self) -> Result<DeclRef> {
        let start = self.measure();

        let name = self.qualified_identifier()?;
        let mut parameters = vec![];

        // can match `[` or `(`
        while let Token::LeftParen | Token::LeftBracket = self.peek().value() {
            parameters.push(self.parameter()?);
        }

        let return_type = if let Token::Colon = self.peek().value() {
            self.next(); // skip ':'
            self.type_expr()?
        } else {
            Type::Infer
        };

        Ok(DeclRef::new(
            start.on(self.measure()),
            Decl::Signature(Signature {
                name,
                parameters,
                return_type,
                body: None,
            }),
        ))
    }

    /// Parses a reference to parameter [Expr]
    pub fn parameter(&mut self) -> Result<Parameter> {
        let mut errors = vec![];

        let go = |this: &mut Self, errors: &mut Vec<Spanned<ParseError>>| {
            this.recover(errors, Parser::named_parameter)
                .or_else(|| this.recover(errors, Parser::unnamed_parameter))
                .map(Ok)
                .unwrap_or_else(|| {
                    this.end_diagnostic(ParseError::ExpectedParameter)
                        .map_err(|error| {
                            errors
                                .iter()
                                .fold(error, |acc, next| acc.with_spanned(next.clone()))
                        })
                })
        };

        match self.peek().value() {
            Token::LeftParen => {
                self.expect(Token::LeftParen)?;
                let (name, parameter_type) = go(self, &mut errors)?;
                self.expect(Token::RightParen)?;

                Ok(Parameter {
                    name,
                    parameter_type,
                    explicit: true,
                })
            }

            Token::LeftBracket => {
                self.expect(Token::LeftBracket)?;
                let (name, parameter_type) = go(self, &mut errors)?;
                self.expect(Token::RightBracket)?;

                Ok(Parameter {
                    name,
                    parameter_type,
                    explicit: false,
                })
            }
            _ => unreachable!(),
        }
    }

    /// Parses a reference to named parameter [Parameter]
    pub fn named_parameter(&mut self) -> Result<(Option<LocalId>, Type)> {
        let parameter_name = self.name()?.map(FunctionId);
        self.expect(Token::Colon)?;
        let parameter_type = self.type_expr()?;

        Ok((Some(LocalId(parameter_name)), parameter_type))
    }

    /// Parses a reference to unnamed parameter [Parameter]
    pub fn unnamed_parameter(&mut self) -> Result<(Option<LocalId>, Type)> {
        let parameter_type = self.type_expr()?;

        Ok((None, parameter_type))
    }

    /// Parses a reference to [Stmt]
    pub fn stmt(&mut self) -> Result<StmtRef> {
        let mut errors = vec![];

        let start = self.measure();

        match self.peek().value() {
            Token::Return => return self.return_stmt(),
            Token::Let => match self.let_stmt() {
                Ok(value) => return Ok(value),
                Err(error) => errors.push(error),
            },
            _ => {}
        };

        self.recover(&mut errors, Parser::eval_stmt)
            .or_else(|| self.recover(&mut errors, Parser::ask_stmt))
            .map(Ok)
            .unwrap_or_else(|| {
                self.end_diagnostic(ParseError::CantParseStatement)
                    .map_err(|error| error.on(start.on(self.measure())))
                    .map_err(|error| {
                        errors
                            .into_iter()
                            .fold(error, |acc, next| acc.with_spanned(next))
                    })
            })
    }

    /// Parses a reference to [Stmt::Return]
    pub fn return_stmt(&mut self) -> Result<StmtRef> {
        let start = self.measure();

        self.expect(Token::Return)?;

        let value = match self.peek().value() {
            Token::Semi => None,
            _ => Some(self.expr()?),
        };

        Ok(StmtRef::new(start.on(self.measure()), Stmt::Return(value)))
    }

    /// Parses a reference to [Stmt::Eval]
    pub fn eval_stmt(&mut self) -> Result<StmtRef> {
        let start = self.measure();

        let value = self.expr()?;

        Ok(StmtRef::new(start.on(self.measure()), Stmt::Eval(value)))
    }

    /// Parses a reference to [Stmt::Ask]
    pub fn ask_stmt(&mut self) -> Result<StmtRef> {
        let start = self.measure();

        let pat = self.pat()?;
        self.expect(Token::InverseArrow)?;
        let value = self.expr()?;

        Ok(StmtRef::new(
            start.on(self.measure()),
            Stmt::Ask(pat, value),
        ))
    }

    /// Parses a reference to [Stmt::Let]
    pub fn let_stmt(&mut self) -> Result<StmtRef> {
        let start = self.measure();

        self.expect(Token::Let)?;
        let pat = self.pat()?;
        self.expect(Token::Equal)?;
        let value = self.expr()?;

        Ok(StmtRef::new(
            start.on(self.measure()),
            Stmt::Let(pat, value),
        ))
    }

    /// Parses a reference to [Pat]
    pub fn pat(&mut self) -> Result<PatRef> {
        use Token::*;

        let current = self.peek();
        let value = match current.value() {
            Symbol(..) => {
                let error = current
                    .replace(ParseError::SymbolInsteadOfIdentifier)
                    .with_tip(Tip::MaybeWriteSymbolName(current.value().clone()));

                self.report(error);
                self.next(); // skip ant tries to parse the next token

                Pat::Error
            }

            // Booleans
            True => Pat::Literal(Literal::True),
            False => Pat::Literal(Literal::False),

            // Integers
            Int8(n, signed) => Pat::Literal(Literal::Int8(*n, *signed)),
            Int16(n, signed) => Pat::Literal(Literal::Int16(*n, *signed)),
            Int32(n, signed) => Pat::Literal(Literal::Int32(*n, *signed)),
            Int64(n, signed) => Pat::Literal(Literal::Int64(*n, *signed)),
            Int128(n, signed) => Pat::Literal(Literal::Int128(*n, *signed)),

            // Floating pointers
            Float32(n) => Pat::Literal(Literal::Float32(*n)),
            Float64(n) => Pat::Literal(Literal::Float64(*n)),

            Ident(..) => {
                // skip <identifier>
                //
                // It does not uses the Ident(..) pattern, because of the location, we need locality
                // of the ast.
                let ident = self.identifier()?.map(FunctionId);

                Pat::Local(LocalId(ident))
            }

            //>>>Composed tokens
            // Can parse the following expressions
            // * [Constructor]
            LeftParen => {
                self.next(); // skip '('

                let name = self.constructor_identifier()?;

                let mut arguments = vec![];
                while let Some(pattern) = self.catch(Parser::pat)? {
                    arguments.push(pattern);
                }

                self.expect(Token::RightParen) // consumes ')'
                    .map_err(|error| error.swap(ParseError::UnfinishedParenthesis))?;

                Pat::Constructor(Constructor { name, arguments })
            }

            // * [List]
            LeftBracket => {
                self.next(); // skip '['

                let mut items = vec![];
                if !self.match_token(Token::RightBracket) {
                    items = self.comma(Parser::pat)?;
                }

                self.expect(Token::RightBracket) //   consumes ']'
                    .map_err(|error| error.swap(ParseError::UnfinishedBrackets))?;

                Pat::List(List { items })
            }

            _ => {
                return self
                    .end_diagnostic(ParseError::CantParsePattern)
                    .map_err(|error| error.with_error(ParseError::UnexpectedToken))
            }
        };

        Ok(current.swap(value))
    }

    /// Parses a reference to [Expr]
    pub fn expr(&mut self) -> Result<ExprRef> {
        let start = self.measure();

        let expr = self.runtime_expr()?;

        // starts parsing dsl, that is only available in runtime/evaluating code
        if let Token::LeftBrace = self.peek().value() {
            self.next(); // skip '{'

            let mut stmts = vec![self.stmt()?];
            while let Token::Comma = self.peek().value() {
                self.next(); // skips ';'

                stmts.push(self.stmt()?);
            }

            self.expect(Token::RightBrace)
                .map_err(|error| error.swap(ParseError::UnfinishedBlock))?;

            return Ok(ExprRef::new(
                start.on(self.measure()),
                Expr::Dsl(Dsl {
                    callee: expr,
                    parameters: vec![], // TODO
                    block: stmts,
                }),
            ));
        }

        Ok(expr)
    }

    /// Parses a reference to [Expr]
    pub fn runtime_expr(&mut self) -> Result<ExprRef> {
        let current = self.peek();

        match current.value() {
            Token::Let => return self.val(),
            Token::Lambda => return self.unicode_lam(),
            Token::Forall => return self.unicode_qualifier(),
            Token::Pi => return self.unicode_pi(),
            Token::Sigma => return self.unicode_sigma(),
            Token::Symbol(n) if n == "\\" => return self.lam(),
            _ => {}
        }

        self.binary()
    }

    /// Parses a reference to [Type]
    pub fn type_expr(&mut self) -> Result<Type> {
        let expr = self.runtime_expr()?;

        Ok(Type::Explicit(expr))
    }

    /// Parses a reference to [Let]
    pub fn val(&mut self) -> Result<ExprRef> {
        let start = self.measure();

        self.next(); // skip 'let'
        let bindings = self.comma(Parser::binding)?;
        self.expect(Token::In)?;
        let in_value = self.expr()?;

        Ok(ExprRef::new(
            start.on(self.measure()),
            Expr::Let(Let { bindings, in_value }),
        ))
    }

    /// Parses a reference to [Binding]
    pub fn binding(&mut self) -> Result<BindingRef> {
        let start = self.measure();

        let name = self.name()?.map(FunctionId);
        self.expect(Token::Equal)?;
        let value = self.expr()?;

        Ok(BindingRef::new(
            start.on(self.measure()),
            Binding {
                name: LocalId(name),
                value,
            },
        ))
    }

    /// Parses a reference to [Lam]
    pub fn lam(&mut self) -> Result<ExprRef> {
        let mut parameters = vec![];
        let start = self.measure();

        self.next(); // skip '\'

        while let Token::Ident(..) = self.peek().value() {
            let ident = self.identifier()?.map(FunctionId);

            parameters.push(LocalId(ident));
        }

        self.expect(Token::Arrow)?;
        let value = self.expr()?;

        Ok(ExprRef::new(
            start.on(self.measure()),
            Expr::Lam(Lam { parameters, value }),
        ))
    }

    /// Parses a reference to [Lam] using unicode characters: λ
    pub fn unicode_lam(&mut self) -> Result<ExprRef> {
        self.lam()
    }

    /// Parses a [Qualifier] expression [Expr] using unicode characters: ∀
    pub fn unicode_qualifier(&mut self) -> Result<ExprRef> {
        let start = self.measure();

        self.next(); //                                 skip '∀'
        self.expect(Token::LeftParen)?; //              consumes '('
        let constraints = self.comma(Parser::expr)?; // consumes <constraint*>
        self.expect(Token::RightParen)?; //             consumes ')'
        self.expect(Token::Arrow)?; //                  consumes '->'
        let return_type = self.type_expr()?; //         consumes <expr>

        Ok(ExprRef::new(
            start.on(self.measure()),
            Expr::Qualifier(Qualifier {
                constraint: constraints.into_iter().map(Constraint).collect(),
                return_type,
            }),
        ))
    }

    /// Parses a [Pi] expression [Expr] using unicode characters: Π
    pub fn unicode_pi(&mut self) -> Result<ExprRef> {
        let start = self.measure();

        self.next(); //                            skip 'Π'
        self.expect(Token::LeftParen)?; //         consumes '('
        let parameter_name = self.name()?; //      consumes <identifier>
        self.expect(Token::Colon)?; //             consumes ':'
        let parameter_type = self.type_expr()?; // consumes <expr>
        self.expect(Token::RightParen)?; //        consumes ')'
        self.expect(Token::Arrow)?; //             consumes '->'
        let return_type = self.type_expr()?; //    consumes <expr>

        Ok(ExprRef::new(
            start.on(self.measure()),
            Expr::Pi(Pi {
                parameter_name: Some(LocalId(parameter_name.map(FunctionId))),
                parameter_type,
                return_type,
            }),
        ))
    }

    /// Parses a [Sigma] expression [Expr] using unicode characters: Σ
    pub fn unicode_sigma(&mut self) -> Result<ExprRef> {
        let start = self.measure();

        self.next(); //                            skip 'Σ'
        self.expect(Token::LeftParen)?; //         consumes '('
        let parameter_name = self.name()?; //      consumes <identifier>
        self.expect(Token::Colon)?; //             consumes ':'
        let parameter_type = self.type_expr()?; // consumes <expr>
        self.expect(Token::RightParen)?; //        consumes ')'
        self.expect(Token::Arrow)?; //             consumes '->'
        let return_type = self.type_expr()?; //    consumes <expr>

        Ok(ExprRef::new(
            start.on(self.measure()),
            Expr::Sigma(Sigma {
                parameter_name: LocalId(parameter_name.map(FunctionId)),
                parameter_type,
                return_type,
            }),
        ))
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
        let mut value = self.qualifier()?;

        while let Token::Colon = self.peek().value() {
            self.next(); // skips ':'

            let against = self.qualifier()?;

            // Combines two locations
            let span = value.span.start..against.span.end;

            value = ExprRef::new(span, Expr::Ann(Ann { value, against }))
        }

        Ok(value)
    }

    /// Parses a reference to [Qualifier]
    pub fn qualifier(&mut self) -> Result<ExprRef> {
        let mut constraint = self.unnamed_pi()?;

        while let Token::DoubleArrow = self.peek().value() {
            self.next(); // skips '=>'

            let return_type = self.unnamed_pi()?;

            // Combines two locations
            let span = constraint.span.start..return_type.span.end;

            constraint = ExprRef::new(
                span,
                Expr::Qualifier(Qualifier {
                    constraint: vec![Constraint(constraint)],
                    return_type: Type::Explicit(return_type),
                }),
            )
        }

        Ok(constraint)
    }

    /// Parses a reference to [Pi]
    pub fn unnamed_pi(&mut self) -> Result<ExprRef> {
        let mut lhs = self.accessor()?;

        while let Token::Arrow = self.peek().value() {
            self.next(); // skips '->'

            let rhs = self.accessor()?;

            // Combines two locations
            let span = lhs.span.start..rhs.span.end;

            lhs = ExprRef::new(
                span,
                Expr::Pi(Pi {
                    parameter_name: None,
                    parameter_type: Type::Explicit(lhs),
                    return_type: Type::Explicit(rhs),
                }),
            )
        }

        Ok(lhs)
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
    fn name(&mut self) -> Result<StringRef> {
        self.eat(|next| match next.value() {
            Token::Ident(content) => Some(next.replace(content.clone())),
            _ => None,
        })
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

    /// Parses a qualified identifier, and return it's content.
    fn constructor_identifier(&mut self) -> Result<ConstructorId> {
        self.eat(|next| match next.value() {
            Token::Ident(content) => Some(ConstructorId(vec![
                next.replace(FunctionId(content.clone()))
            ])),

            // Accepts symbol, so the parser is able to parse something like `Functor.<$>`
            Token::Symbol(content) => Some(ConstructorId(vec![
                next.replace(FunctionId(content.clone()))
            ])),
            _ => None,
        })
    }

    /// Parses a qualified identifier, and return it's content.
    fn qualified_identifier(&mut self) -> Result<GlobalId> {
        self.eat(|next| match next.value() {
            Token::Ident(content) => {
                Some(GlobalId(vec![next.replace(FunctionId(content.clone()))]))
            }

            // Accepts symbol, so the parser is able to parse something like `Functor.<$>`
            Token::Symbol(content) => {
                Some(GlobalId(vec![next.replace(FunctionId(content.clone()))]))
            }
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

    /// Parses a [Pi] expression [Expr]
    pub fn pi(&mut self) -> Result<Expr> {
        self.next(); //                            skip '('
        let parameter_name = self.name()?; //      consumes <identifier>
        self.expect(Token::Colon)?; //             consumes ':'
        let parameter_type = self.type_expr()?; // consumes <expr>
        self.expect(Token::RightParen)?; //        consumes ')'
        self.expect(Token::Arrow)?; //             consumes '->'
        let return_type = self.type_expr()?; //    consumes <expr>

        Ok(Expr::Pi(Pi {
            parameter_name: Some(LocalId(parameter_name.map(FunctionId))),
            parameter_type,
            return_type,
        }))
    }

    /// Parses a [Expr::Group]
    pub fn group(&mut self) -> Result<Expr> {
        self.next(); //                     skip '('
        let expr = self.expr()?; //         consumes <expr>
        self.expect(Token::RightParen) //   consumes ')'
            .map_err(|error| error.swap(ParseError::UnfinishedParenthesis))?;

        Ok(Expr::Group(expr))
    }

    /// Parses a [Sigma] expression [Expr]
    pub fn sigma(&mut self) -> Result<Expr> {
        self.next(); //                            skip '['
        let parameter_name = self.name()?; //      consumes <identifier>
        self.expect(Token::Colon)?; //             consumes ':'
        let parameter_type = self.type_expr()?; // consumes <expr>
        self.expect(Token::RightBracket)?; //      consumes ']'
        self.expect(Token::Arrow)?; //             consumes '->'
        let return_type = self.type_expr()?; //    consumes <expr>

        Ok(Expr::Sigma(Sigma {
            parameter_name: LocalId(parameter_name.map(FunctionId)),
            parameter_type,
            return_type,
        }))
    }

    /// Parses a [Array] expression [Expr]
    pub fn array(&mut self) -> Result<Expr> {
        let mut items = vec![];
        self.next(); //                       skip '['

        if !self.match_token(Token::RightBracket) {
            items = self.comma(Parser::expr)?;
        }

        self.expect(Token::RightBracket) //   consumes ']'
            .map_err(|error| error.swap(ParseError::UnfinishedBrackets))?;

        Ok(Expr::Array(Array { items }))
    }

    /// Parses a reference to [Literal] or primary [Expr]
    pub fn primary(&mut self) -> Result<ExprRef> {
        use Token::*;

        let current = self.peek();
        let value = match current.value() {
            Symbol(..) => {
                let error = current
                    .replace(ParseError::SymbolInsteadOfIdentifier)
                    .with_tip(Tip::MaybeWriteSymbolName(current.value().clone()));

                self.report(error);
                self.next(); // skip ant tries to parse the next token

                Expr::Error
            }

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
                let ident = self.identifier()?.map(FunctionId);

                // Creates a new path.
                let mut path = vec![ident];
                while let Token::Dot = self.peek().value() {
                    self.next(); // skip `.`
                    let fn_id = self.identifier()?.map(FunctionId);
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
                let mut errors = vec![];

                return self
                    .recover(&mut errors, Parser::pi)
                    .or_else(|| self.recover(&mut errors, Parser::group))
                    .map(|expr| Ok(current.swap(expr)))
                    .unwrap_or_else(|| {
                        self.end_diagnostic(ParseError::ExpectedParenthesisExpr)
                            .map_err(|error| {
                                errors
                                    .into_iter()
                                    .fold(error, |acc, next| acc.with_spanned(next))
                            })
                    });
            }

            // * [Sigma]
            // * [Array]
            LeftBracket => {
                let mut errors = vec![];

                return self
                    .recover(&mut errors, Parser::sigma)
                    .or_else(|| self.recover(&mut errors, Parser::array))
                    .map(|expr| Ok(current.swap(expr)))
                    .unwrap_or_else(|| {
                        self.end_diagnostic(ParseError::ExpectedBracketExpr)
                            .map_err(|error| {
                                errors
                                    .into_iter()
                                    .fold(error, |acc, next| acc.with_spanned(next))
                            })
                    });
            }

            // Help expression
            Help => {
                self.next(); // skip '?'
                let expr = self.expr()?;

                return Ok(current.swap(Expr::Help(expr)));
            }

            Eof => return self.end_diagnostic(ParseError::CantParseDueToEof),

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
        let code = "let combine = [x, MonadIO m, F a, C] => (a: m a) -> [b: m b] -> m c in todo";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.diagnostic(Parser::expr).unwrap())
    }

    #[test]
    fn sig_decl() {
        let code = "cond : (f true) -> ((f false) -> (f cond));";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.diagnostic(Parser::decl).unwrap())
    }

    #[test]
    fn lam_expr() {
        let code = "\\a b -> c";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.diagnostic(Parser::expr).unwrap())
    }

    #[test]
    fn sigma_expr() {
        let code = "[a: t] -> b";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.diagnostic(Parser::expr).unwrap())
    }

    #[test]
    fn unicode_expr() {
        let code = "Π (d: t) -> e";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.diagnostic(Parser::expr).unwrap())
    }

    #[test]
    fn group_expr() {
        let code = "[Monad m] => m a";

        let stream = Lexer::new(code);
        let mut parser = Parser::new(code, stream.peekable());

        println!("{:#?}", parser.diagnostic(Parser::expr).unwrap())
    }

    #[test]
    fn pi_expr() {
        let code = "(a: t) -> b";

        let lexer = Lexer::new(code);

        let mut parser = Parser::new(code, lexer.peekable());
        println!("{:#?}", parser.diagnostic(Parser::expr).unwrap())
    }

    #[test]
    fn ask_stmt() {
        let code = "(Just a) <- findUser 105 { pao }";

        let lexer = Lexer::new(code);

        let mut parser = Parser::new(code, lexer.peekable());

        println!("{:#?}", parser.diagnostic(Parser::stmt).unwrap())
    }
}
