use crate::ast::node::TokenKind::*;
use crate::ast::node::TreeKind::*;

use super::error::ParseError::*;
use super::event::MarkClosed;
use super::Parser;

/// File = Decl*
pub fn file(p: &mut Parser) {
    let m = p.open();

    while !p.eof() {
        decl(p);
    }

    p.close(m, File);
}

/// Decl = DeclSignature
pub fn decl(p: &mut Parser) {
    decl_signature(p)
}

/// DeclSignature = Global ':' TypeExpr
pub fn decl_signature(p: &mut Parser) {
    let m = p.open();

    global(p);
    p.at(Colon);
    type_expr(p);

    p.close(m, DeclSignature);
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
    expr_binary(p)
}

/// ExprBinary = ExprAccessor (Symbol ExprAccessor)*
pub fn expr_binary(p: &mut Parser) {
    let m = p.open();

    expr_ann(p);

    // simplify by returning the lhs symbol directly
    if p.at(Symbol) {
        while p.eat(Symbol) {
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
        while p.eat(Colon) {
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
        while p.eat(DoubleArrow) {
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
        while p.eat(RightArrow) {
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
        while p.eat(Dot) {
            expr_app(p);
        }

        p.close(m, ExprAcessor);
    } else {
        p.ignore(m)
    }
}

/// ExprApp = Primary Primary*
pub fn expr_app(p: &mut Parser) {
    let Some(mut lhs) = primary(p) else {
        p.report(PrimaryExpectedError);
        return;
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

pub fn expr_group(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    p.expect(LeftParen);
    expr(p);
    p.expect(RightParen);
    p.close(m, ExprGroup)
}

/// Primary = Nat 'n'? | Int 'i8'? | Int 'u8'?
///         | Int 'i16'? | Int 'u16'? | Int ('u' | 'i32')?
///         | Int ('u' | 'u32')? | Int 'i64'? | Int 'u64'?
///         | Int 'i128'? | Int 'u128'? | Float 'f32'?
///         | Float 'f64'? | 'true' | 'false'
pub fn primary(p: &mut Parser) -> Option<MarkClosed> {
    if p.eof() {
        return None;
    }

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

/// Global = <<Terminal>>
pub fn global(p: &mut Parser) {
    p.terminal(LitSymbol);
}

/// Symbol = <<Terminal>>
pub fn symbol(p: &mut Parser) {
    p.terminal(LitSymbol);
}
