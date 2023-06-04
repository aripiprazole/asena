use crate::ast::node::TokenKind::*;
use crate::ast::node::TreeKind::*;

use super::error::ParseError::*;
use super::event::MarkClosed;
use super::Parser;

/// File = Decl*
pub fn file(p: &mut Parser) {
    let m = p.open();

    if !p.eof() {
        decl(p);
    }

    while !p.eof() && semi_eof(p) {
        decl(p);
    }

    p.close(m, File);
}

/// Decl = DeclSignature | DeclAssign
pub fn decl(p: &mut Parser) {
    let mut decl = p.savepoint();
    decl_signature(&mut decl);
    if !decl.has_errors() {
        p.return_at(decl);
        return;
    }

    decl_assign(p);
}

/// DeclAssign = Global Pat* '=' Expr
pub fn decl_assign(p: &mut Parser) {
    let m = p.open();
    global(p);
    p.field("name");
    while !p.eof() && !p.at(Equal) {
        pat(p);
    }
    p.expect(Equal);
    expr_dsl(p);
    p.field("value");
    p.close(m, DeclAssign);
}

/// DeclSignature = Global Param* ':' TypeExpr
pub fn decl_signature(p: &mut Parser) {
    let m = p.open();
    global(p);
    p.field("name");
    while !p.eof() && p.at(LeftParen) || p.at(LeftBracket) {
        param(p);
    }
    p.expect(Colon);
    type_expr(p);
    p.field("type");
    if p.eat(LeftBrace) {
        if !p.at(RightBrace) {
            stmt(p);
        }
        while !p.eof() && !p.at(RightBrace) && semi(p) {
            stmt(p);
        }
        last_semi(p);
        p.expect(RightBrace);
    }

    p.close(m, DeclSignature);
}

/// Param = ImplicitParam | ExplicitParam
pub fn param(p: &mut Parser) {
    let m = p.open();
    let token = p.peek();

    match token.kind {
        LeftParen => {
            p.expect(LeftParen);
            p.expect(Identifier);
            p.expect(Colon);
            type_expr(p);
            p.expect(RightParen);
        }
        LeftBracket => {
            p.expect(LeftBracket);
            type_expr(p);
            p.expect(RightBracket);
        }
        _ => {}
    }

    p.close(m, Param);
}

pub fn last_semi(p: &mut Parser) {
    while !p.eof() && p.eat(Semi) {
        p.warning(UeselessSemiError);
    }
}

pub fn semi_eof(p: &mut Parser) -> bool {
    if !p.eat(Semi) {
        p.report(MissingSemiError);
    } else if p.lookahead(0) == RightBrace {
        p.warning(UeselessSemiError);
    }

    while !p.eof() && p.eat(Semi) {
        p.warning(UeselessSemiError);
    }

    // returns if can continues
    !p.eof()
}

pub fn semi(p: &mut Parser) -> bool {
    if !p.eat(Semi) {
        p.report(MissingSemiError);
    } else if p.lookahead(0) == RightBrace {
        p.warning(UeselessSemiError);
    }

    while !p.eof() && p.eat(Semi) {
        p.warning(UeselessSemiError);
    }

    // returns if can continues
    // generally the end of the statement block is RightBrace
    !p.at(RightBrace)
}

pub fn stmt(p: &mut Parser) {
    match p.lookahead(0) {
        ReturnKeyword => stmt_return(p),
        LetKeyword => stmt_let(p),
        _ => {
            let mut ask = p.savepoint();
            stmt_ask(&mut ask);
            if !ask.has_errors() {
                p.return_at(ask);
                return;
            }

            stmt_expr(p)
        }
    }
}

pub fn stmt_return(p: &mut Parser) {
    let m = p.open();
    p.expect(ReturnKeyword);
    if !p.at(Semi) {
        expr_dsl(p);
    }
    p.close(m, StmtReturn);
}

pub fn stmt_ask(p: &mut Parser) {
    let m = p.open();
    pat(p);
    p.expect(LeftArrow);
    expr_dsl(p);
    p.close(m, StmtAsk);
}

pub fn stmt_let(p: &mut Parser) {
    let m = p.open();
    p.expect(LetKeyword);
    pat(p);
    p.expect(Equal);
    expr_dsl(p);
    p.close(m, StmtLet);
}

pub fn stmt_expr(p: &mut Parser) {
    let m = p.open();
    expr_dsl(p);
    p.close(m, StmtExpr);
}

/// TypeExpr = Expr
pub fn type_expr(p: &mut Parser) {
    let m = p.open();
    expr(p);
    p.close(m, Type);
}

/// Expr =
///   ExprGroup
/// | ExprBinary | ExprAccessor | ExprApp
/// | ExprDsl | ExprArray | ExprLam
/// | ExprLet | ExprGlobal | ExprLocal
/// | ExprLit | ExprAnn | ExprQual
/// | ExprPi | ExprSigma | ExprHelp
pub fn expr(p: &mut Parser) {
    let token = p.peek();
    match token.kind {
        Symbol if token.text == "\\" => return expr_lam(p),
        Help => return expr_help(p),
        _ => {}
    }

    expr_binary(p);
}

/// ExprHelp = '?' ExprDsl
pub fn expr_help(p: &mut Parser) {
    let m = p.open();
    p.expect(Help);
    expr_dsl(p);
    p.close(m, ExprHelp);
}

/// ExprLam = '\' Identifier* '->' ExprDsl
pub fn expr_lam(p: &mut Parser) {
    let m = p.open();
    p.advance();
    while !p.eof() && !p.at(RightArrow) {
        let m = p.open();
        p.expect(Identifier);
        p.close(m, LamParam);
    }
    p.expect(RightArrow);
    expr_dsl(p);
    p.close(m, ExprLam);
}

pub fn expr_dsl(p: &mut Parser) {
    let m = p.open();
    expr(p);

    if p.eat(LeftBrace) {
        if !p.at(RightBrace) {
            stmt(p);
        }
        while !p.eof() && !p.at(RightBrace) && semi(p) {
            stmt(p);
        }
        last_semi(p);
        p.expect(RightBrace);
        p.close(m, ExprDsl);
    } else {
        p.ignore(m);
    }
}

/// ExprBinary = ExprAccessor (Symbol ExprAccessor)*
pub fn expr_binary(p: &mut Parser) {
    let m = p.open();

    expr_ann(p);

    // simplify by returning the lhs symbol directly
    if p.at(Symbol) {
        while !p.eof() && p.eat(Symbol) {
            expr_ann(p);
        }

        p.close(m, ExprBinary);
    } else {
        p.ignore(m)
    }
}

/// ExprAnn = ExprQual (':' ExprQual)*
pub fn expr_ann(p: &mut Parser) {
    let m = p.open();

    expr_qual(p);

    // simplify by returning the lhs symbol directly
    if p.at(Colon) {
        while !p.eof() && p.eat(Colon) {
            expr_qual(p);
        }

        p.close(m, ExprAnn);
    } else {
        p.ignore(m)
    }
}

/// ExprQual = ExprAnonymousPi ('=>' ExprAnonymousPi)*
pub fn expr_qual(p: &mut Parser) {
    let m = p.open();

    expr_anonymous_pi(p);

    // simplify by returning the lhs symbol directly
    if p.at(DoubleArrow) {
        while !p.eof() && p.eat(DoubleArrow) {
            expr_anonymous_pi(p);
        }

        p.close(m, ExprQual);
    } else {
        p.ignore(m)
    }
}

/// ExprAnonymousPi = ExprAccessor ('->' ExprAccessor)*
pub fn expr_anonymous_pi(p: &mut Parser) {
    let m = p.open();

    expr_accessor(p);

    // simplify by returning the lhs symbol directly
    if p.at(RightArrow) {
        while !p.eof() && p.eat(RightArrow) {
            expr_accessor(p);
        }

        p.close(m, ExprPi);
    } else {
        p.ignore(m)
    }
}

/// ExprAccessor = ExprApp ('.' ExprApp)*
pub fn expr_accessor(p: &mut Parser) {
    let m = p.open();

    expr_app(p);

    // simplify by returning the lhs symbol directly
    if p.at(Dot) {
        while !p.eof() && p.eat(Dot) {
            expr_app(p);
        }

        p.close(m, ExprAcessor);
    } else {
        p.ignore(m)
    }
}

/// ExprApp = Primary Primary*
pub fn expr_app(p: &mut Parser) {
    let mut lhs = match primary(p) {
        Some(lhs) => lhs,
        None if p.eof() => {
            p.report(PrimaryExpectedError);
            return;
        }
        None => MarkClosed::new(0, p.peek().span().clone()),
    };

    while !p.eof() {
        let mut arg = p.savepoint();
        if matches!(primary(&mut arg), None) {
            // if can't parse anything, it's not a app expression
            break;
        };

        if arg.has_errors() && p.eof() {
            break;
        } else {
            p.return_at(arg);
            let m = p.open_before(lhs);
            lhs = p.close(m, ExprApp);
        }
    }
}

pub fn expr_pi(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    p.expect(LeftParen);
    if p.eat(Identifier) {
        p.field("parameter_name");
        p.expect(Colon);
    }
    type_expr(p);
    p.field("parameter_type");
    p.expect(RightParen);
    p.expect(RightArrow);
    type_expr(p);
    p.field("return_type");
    p.close(m, ExprPi)
}

pub fn expr_sigma(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    p.expect(LeftBracket);
    if p.eat(Identifier) {
        p.field("parameter_name");
        p.expect(Colon);
    }
    type_expr(p);
    p.field("parameter_type");
    p.expect(RightBracket);
    p.expect(RightArrow);
    type_expr(p);
    p.field("return_type");
    p.close(m, ExprSigma)
}

pub fn expr_group(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    p.expect(LeftParen);
    expr_dsl(p);
    p.expect(RightParen);
    p.close(m, ExprGroup)
}

pub fn expr_array(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    p.expect(LeftBracket);
    if !p.at(RightBracket) {
        expr_dsl(p);
    }
    while !p.eof() && !p.at(RightBracket) {
        p.expect(Comma);
        expr_dsl(p);
    }
    p.expect(RightBracket);
    p.close(m, ExprArray)
}

/// Primary = Nat 'n'? | Int 'i8'? | Int 'u8'?
///         | Int 'i16'? | Int 'u16'? | Int ('u' | 'i32')?
///         | Int ('u' | 'u32')? | Int 'i64'? | Int 'u64'?
///         | Int 'i128'? | Int 'u128'? | Float 'f32'?
///         | Float 'f64'? | 'true' | 'false'
pub fn primary(p: &mut Parser) -> Option<MarkClosed> {
    let token = p.peek();

    let result = match token.value.kind {
        TrueKeyword => p.terminal(LitTrue),
        FalseKeyword => p.terminal(LitFalse),

        Int8 => p.terminal(LitInt8),
        Int16 => p.terminal(LitInt16),
        Int32 => p.terminal(LitInt32),
        Int64 => p.terminal(LitInt64),
        Int128 => p.terminal(LitInt128),

        UInt8 => p.terminal(LitUInt8),
        UInt16 => p.terminal(LitUInt16),
        UInt32 => p.terminal(LitUInt32),
        UInt64 => p.terminal(LitUInt64),
        UInt128 => p.terminal(LitUInt128),

        Float32 => p.terminal(LitFloat32),
        Float64 => p.terminal(LitFloat64),

        Identifier => p.terminal(LitIdentifier),
        String => p.terminal(LitString),

        // Parse array or named sigma expressions
        // - Sigma
        // - Array
        LeftBracket => {
            let mut pi = p.savepoint();
            let closed = expr_sigma(&mut pi);
            if !pi.has_errors() {
                p.return_at(pi);
                return Some(closed);
            }

            let mut group = p.savepoint();
            let closed = expr_array(&mut group);
            if !group.has_errors() {
                p.return_at(group);
                return Some(closed);
            }

            return p.report(ExpectedParenExprError);
        }

        // Parse group or named pi expressions
        // - Pi
        // - Group
        LeftParen => {
            let mut pi = p.savepoint();
            let closed = expr_pi(&mut pi);
            if !pi.has_errors() {
                p.return_at(pi);
                return Some(closed);
            }

            let mut group = p.savepoint();
            let closed = expr_group(&mut group);
            if !group.has_errors() {
                p.return_at(group);
                return Some(closed);
            }

            return p.report(ExpectedParenExprError);
        }

        _ => {
            match token.value.kind {
                Eof => p.report(EofError),
                Symbol => p.report(ExpectedTokenError(Identifier)),

                LetKeyword | IfKeyword | MatchKeyword => {
                    // TODO: try to properly parse the expression
                    p.report(PrimarySurroundedError(token.value.kind))
                }

                ElseKeyword => p.report(DanglingElseError),
                CaseKeyword => p.report(ReservedKeywordError(CaseKeyword)),

                UseKeyword | TypeKeyword | RecordKeyword | ClassKeyword | TraitKeyword
                | InstanceKeyword => p.report(DeclReservedKeywordError(TypeKeyword)),

                ReturnKeyword => p.report(StmtReservedKeywordError(ReturnKeyword)),
                WhereKeyword => p.report(StmtReservedKeywordError(WhereKeyword)),
                InKeyword => p.report(ReservedKeywordError(InKeyword)),

                Lambda => p.report(UnicodeError(Lambda, "lambda")),
                Forall => p.report(UnicodeError(Lambda, "forall")),
                Pi => p.report(UnicodeError(Lambda, "pi")),
                Sigma => p.report(UnicodeError(Lambda, "sigma")),

                LeftBracket => p.report(UnicodeError(LeftBracket, "left_bracket")),
                RightBracket => p.report(UnicodeError(RightBracket, "right_bracket")),
                LeftBrace => p.report(UnicodeError(LeftBrace, "left_brace")),
                RightBrace => p.report(UnicodeError(RightBrace, "right_brace")),
                RightParen => p.report(UnicodeError(RightParen, "right_paren")),

                Comma => p.report(UnicodeError(Comma, "comma")),
                Semi => p.report(UnicodeError(Semi, "semi")),
                Colon => p.report(UnicodeError(Colon, "colon")),
                Dot => p.report(UnicodeError(Dot, "dot")),
                Help => p.report(UnicodeError(Help, "interrogation")),
                Equal => p.report(UnicodeError(Equal, "equal")),

                DoubleArrow => p.report(UnicodeError(DoubleArrow, "=>")),
                RightArrow => p.report(UnicodeError(RightArrow, "->")),
                LeftArrow => p.report(UnicodeError(LeftArrow, "<-")),

                _ => p.report(PrimaryExpectedError),
            };
            return None;
        }
    };

    Some(result)
}

/// Pat = Nat 'n'? | Int 'i8'? | Int 'u8'?
///     | Int 'i16'? | Int 'u16'? | Int ('u' | 'i32')?
///     | Int ('u' | 'u32')? | Int 'i64'? | Int 'u64'?
///     | Int 'i128'? | Int 'u128'? | Float 'f32'?
///     | Float 'f64'? | 'true' | 'false'
///     | '(' Global Pat* ')' | '_' | '..'
pub fn pat(p: &mut Parser) -> Option<MarkClosed> {
    let m = p.open();
    let token = p.peek();

    let result = match token.value.kind {
        Identifier if token.text == "_" => {
            p.advance();
            return Some(p.close(m, PatWildcard));
        }
        Dot => {
            p.advance();
            if p.eat(Dot) {
                p.report(UnicodeError(Dot, "dot"));

                return None;
            } else {
                return Some(p.close(m, PatSpread));
            }
        }
        Identifier => {
            p.terminal(LitIdentifier);
            return Some(p.close(m, PatLocal));
        }
        LeftBracket => {
            p.expect(LeftBracket);
            if !p.at(RightBracket) {
                pat(p);
            }
            while !p.eof() && !p.at(RightBracket) {
                p.expect(Comma);
                pat(p);
            }
            p.eat(Comma); // trailling comma
            p.expect(RightBracket);
            return Some(p.close(m, PatList));
        }
        LeftParen => {
            p.expect(LeftParen);
            global(p);
            while !p.eof() && !p.at(RightParen) {
                pat(p);
            }
            p.expect(RightParen);
            return Some(p.close(m, PatConstructor));
        }
        String => p.terminal(LitString),
        TrueKeyword => p.terminal(LitTrue),
        FalseKeyword => p.terminal(LitFalse),
        Int8 => p.terminal(LitInt8),
        Int16 => p.terminal(LitInt16),
        Int32 => p.terminal(LitInt32),
        Int64 => p.terminal(LitInt64),
        Int128 => p.terminal(LitInt128),
        UInt8 => p.terminal(LitUInt8),
        UInt16 => p.terminal(LitUInt16),
        UInt32 => p.terminal(LitUInt32),
        UInt64 => p.terminal(LitUInt64),
        UInt128 => p.terminal(LitUInt128),
        Float32 => p.terminal(LitFloat32),
        Float64 => p.terminal(LitFloat64),

        _ => {
            match token.value.kind {
                Eof => p.report(EofError),
                Symbol => p.report(ExpectedTokenError(Identifier)),

                LetKeyword | IfKeyword | MatchKeyword => {
                    // TODO: try to properly parse the expression
                    p.report(PrimarySurroundedError(token.value.kind))
                }

                ElseKeyword => p.report(DanglingElseError),
                CaseKeyword => p.report(ReservedKeywordError(CaseKeyword)),

                UseKeyword | TypeKeyword | RecordKeyword | ClassKeyword | TraitKeyword
                | InstanceKeyword => p.report(DeclReservedKeywordError(TypeKeyword)),

                ReturnKeyword => p.report(StmtReservedKeywordError(ReturnKeyword)),
                WhereKeyword => p.report(StmtReservedKeywordError(WhereKeyword)),
                InKeyword => p.report(ReservedKeywordError(InKeyword)),

                Lambda => p.report(UnicodeError(Lambda, "lambda")),
                Forall => p.report(UnicodeError(Lambda, "forall")),
                Pi => p.report(UnicodeError(Lambda, "pi")),
                Sigma => p.report(UnicodeError(Lambda, "sigma")),

                LeftBracket => p.report(UnicodeError(LeftBracket, "left_bracket")),
                RightBracket => p.report(UnicodeError(RightBracket, "right_bracket")),
                LeftBrace => p.report(UnicodeError(LeftBrace, "left_brace")),
                RightBrace => p.report(UnicodeError(RightBrace, "right_brace")),
                RightParen => p.report(UnicodeError(RightParen, "right_paren")),

                Comma => p.report(UnicodeError(Comma, "comma")),
                Semi => p.report(UnicodeError(Semi, "semi")),
                Colon => p.report(UnicodeError(Colon, "colon")),
                Help => p.report(UnicodeError(Help, "interrogation")),
                Equal => p.report(UnicodeError(Equal, "equal")),

                DoubleArrow => p.report(UnicodeError(DoubleArrow, "=>")),
                RightArrow => p.report(UnicodeError(RightArrow, "->")),
                LeftArrow => p.report(UnicodeError(LeftArrow, "<-")),

                _ => p.report(PrimaryExpectedError),
            };
            return None;
        }
    };

    p.close(m, PatLiteral);

    Some(result)
}

/// Global = <<Terminal>>
pub fn global(p: &mut Parser) {
    p.terminal(LitSymbol);
}

/// Symbol = <<Terminal>>
pub fn symbol(p: &mut Parser) {
    p.terminal(LitSymbol);
}
