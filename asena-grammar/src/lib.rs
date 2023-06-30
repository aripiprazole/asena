use asena_leaf::node::TreeKind;
use asena_leaf::node::TreeKind::*;
use asena_leaf::token::kind::TokenKind;
use asena_leaf::token::kind::TokenKind::*;

use asena_parser::error::ParseError;
use asena_parser::error::ParseError::*;
use asena_parser::event::MarkClosed;
use asena_parser::Parser;

pub mod macros;

use asena_report::quickfix;
use asena_report::Fragment::Insert;
use asena_report::Quickfix;
pub use macros::*;

const PARAM_LIST_RECOVERY: &[TokenKind] = &[Semi, Colon, LeftBrace];
const STMT_RECOVERY: &[TokenKind] = &[
    LeftBrace,
    ClassKeyword,
    EnumKeyword,
    RecordKeyword,
    TypeKeyword,
    TraitKeyword,
    UseKeyword,
];

const EXPR_FIRST: &[TokenKind] = &[
    Identifier,
    LeftBracket,
    LeftParen,
    Str,
    TrueKeyword,
    FalseKeyword,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float32,
    Float64,
];

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

/// Decl = DeclUse | DeclCommand | DeclSignature | DeclAssign
pub fn decl(p: &mut Parser) {
    match p.lookahead(0) {
        UseKeyword => decl_use(p),
        HashSymbol => decl_command(p),
        _ => {
            let decl = p.savepoint().run(decl_assign);
            if !decl.has_errors() {
                return p.return_at(decl);
            };

            decl_signature(p)
        }
    }
}

/// DeclCommand = '#' Global Expr* ';'
pub fn decl_command(p: &mut Parser) {
    let m = p.open();
    p.expect(HashSymbol);
    global(p);
    expr(p);
    while p.at(Comma) {
        p.expect(Comma);
        if p.at(Semi) || p.eof() {
            break;
        }
        expr(p);
    }
    p.close(m, DeclCommand);
}

/// DeclUse = 'use' Global
pub fn decl_use(p: &mut Parser) {
    let m = p.open();
    p.expect(UseKeyword);
    global(p);
    p.close(m, DeclUse);
}

/// DeclAssign = Global Pat* '=' Expr
pub fn decl_assign(p: &mut Parser) {
    let m = p.open();
    global(p);
    p.field("name");
    while !p.eof() && !p.at(EqualSymbol) {
        pat(p);
    }
    p.expect(EqualSymbol);
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
        if param(p) && p.at_any(PARAM_LIST_RECOVERY) {
            p.report(ParseError::ExpectedParameterError);
            break;
        }
    }
    if p.eat(Colon) {
        type_expr(p);
        p.field("type");
        if p.eat(LeftBrace) {
            if !p.at(RightBrace) {
                stmt(p);
            }
            while !p.eof() && !p.at(RightBrace) && semi(p) {
                if stmt(p) {
                    break;
                }
            }
            last_semi(p);
            p.expect(RightBrace);
        }
    } else {
        p.expect(LeftBrace);
        if !p.at(RightBrace) {
            stmt(p);
        }
        while !p.eof() && !p.at(RightBrace) && semi(p) {
            if stmt(p) {
                break;
            }
        }
        last_semi(p);
        p.expect(RightBrace);
    }

    p.close(m, DeclSignature);
}

/// Param = ImplicitParam | ExplicitParam
pub fn param(p: &mut Parser) -> bool {
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
        _ => return true,
    }

    p.close(m, Param);
    false
}

pub fn stmt(p: &mut Parser) -> bool {
    match p.lookahead(0) {
        ReturnKeyword => stmt_return(p),
        LetKeyword => stmt_let(p),
        _ => {
            if p.at_any(EXPR_FIRST) {
                let ask = p.savepoint().run(stmt_ask);
                if !ask.has_errors() {
                    p.return_at(ask);
                    return false;
                }

                stmt_expr(p)
            } else if p.at_any(STMT_RECOVERY) {
                p.report(ParseError::ExpectedStmtError);
                return true;
            }
        }
    }
    false
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
    p.expect(EqualSymbol);
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
    p.close(m, TypeExplicit);
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
        HelpSymbol => return expr_help(p),
        _ => {}
    }

    expr_ann(p);
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

    expr_binary(p);

    // simplify by returning the lhs symbol directly
    if p.at(RightArrow) {
        while !p.eof() && p.eat(RightArrow) {
            expr_binary(p);
        }

        p.close(m, ExprPi);
    } else {
        p.ignore(m)
    }
}

/// ExprHelp = '?' ExprDsl
pub fn expr_help(p: &mut Parser) {
    let m = p.open();
    p.expect(HelpSymbol);
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

    expr_accessor(p);

    // simplify by returning the lhs symbol directly
    if p.at(Symbol) {
        while !p.eof() && p.eat(Symbol) {
            expr_accessor(p);
        }

        p.close(m, ExprBinary);
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

        p.close(m, ExprAccessor);
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
    expr(p);
    p.field("parameter_type");
    p.expect(RightParen);
    p.expect(RightArrow);
    expr(p);
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
    expr(p);
    p.field("parameter_type");
    p.expect(RightBracket);
    p.expect(RightArrow);
    expr(p);
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

/// Primary =
///   Lit
/// | Local # Local
/// | '(' Identifier ':' TypeExpr ')' '->' 'TypeExpr' # Pi
/// | '(' ExprDsl ')' # Group
/// | '[' Identifier ':' TypeExpr ']' '->' 'TypeExpr' # Sigma
/// | '[' ExprDsl (',' ExprDsl)* ','? ']'  # Pi
pub fn primary(p: &mut Parser) -> Option<MarkClosed> {
    if let Some(literal) = lit(p, ExprLit) {
        return Some(literal);
    }

    let token = p.peek();

    let result = match token.value.kind {
        Identifier => {
            let m = p.open();
            p.advance();
            p.close(m, ExprLocal)
        }

        // Parse array or named sigma expressions
        // - Sigma
        // - Array
        LeftBracket => {
            let mut sigma = p.savepoint();
            let closed = expr_sigma(&mut sigma);
            if !sigma.has_errors() {
                p.return_at(sigma);
                return Some(closed);
            }

            expr_array(p)
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

            expr_group(p)
        }
        otherwise => {
            report_non_primary(p, otherwise);

            return None;
        }
    };

    Some(result)
}

/// Pat = '(' Global Pat* ')' | '_' | Lit | '..'
pub fn pat(p: &mut Parser) -> Option<MarkClosed> {
    if let Some(literal) = lit(p, PatLit) {
        return Some(literal);
    }

    let token = p.peek();

    let result = match token.value.kind {
        Identifier if token.text == "_" => {
            let m = p.open();
            p.advance();
            p.close(m, PatWildcard)
        }
        Dot => {
            let m = p.open();
            p.advance();
            if p.eat(Dot) {
                p.report(UnicodeError(Dot, "dot"));

                return None;
            } else {
                p.close(m, PatSpread)
            }
        }
        Identifier => {
            let m = p.open();
            global(p);
            p.close(m, PatGlobal)
        }
        LeftBracket => {
            let m = p.open();
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
            p.close(m, PatList)
        }
        LeftParen => {
            let m = p.open();
            p.expect(LeftParen);
            global(p);
            while !p.eof() && !p.at(RightParen) {
                pat(p);
            }
            p.expect(RightParen);
            p.close(m, PatConstructor)
        }

        otherwise => {
            report_non_primary(p, otherwise);

            return None;
        }
    };

    Some(result)
}

/// Lit =
///   Nat 'n'? | Int 'i8'? | Int 'u8'?
/// | Int 'i16'? | Int 'u16'? | Int ('u' | 'i32')?
/// | Int ('u' | 'u32')? | Int 'i64'? | Int 'u64'?
/// | Int 'i128'? | Int 'u128'? | Float 'f32'?
/// | Float 'f64'? | 'true' | 'false'
pub fn lit(p: &mut Parser, kind: TreeKind) -> Option<MarkClosed> {
    let result = match p.lookahead(0) {
        Str => p.terminal(kind),
        TrueKeyword => p.terminal(kind),
        FalseKeyword => p.terminal(kind),
        Int8 => p.terminal(kind),
        Int16 => p.terminal(kind),
        Int32 => p.terminal(kind),
        Int64 => p.terminal(kind),
        Int128 => p.terminal(kind),
        UInt8 => p.terminal(kind),
        UInt16 => p.terminal(kind),
        UInt32 => p.terminal(kind),
        UInt64 => p.terminal(kind),
        UInt128 => p.terminal(kind),
        Float32 => p.terminal(kind),
        Float64 => p.terminal(kind),
        _ => return None,
    };

    Some(result)
}

/// Global = <<Terminal>>
pub fn global(p: &mut Parser) {
    let m = p.open();
    p.advance();
    while p.eat(Dot) && !p.eof() {
        p.advance();
    }
    p.close(m, QualifiedPathTree);
}

/// Symbol = <<Terminal>>
pub fn symbol(p: &mut Parser) {
    p.terminal(SymbolIdentifier);
}

fn report_non_primary(p: &mut Parser, kind: TokenKind) -> Option<MarkClosed> {
    match kind {
        Eof => p.report(EofError),
        Symbol => p.report(ExpectedTokenError(Identifier)),

        LetKeyword | IfKeyword | MatchKeyword => {
            // TODO: try to properly parse the expression
            p.report(PrimarySurroundedError(kind))
        }

        ElseKeyword => p.report(DanglingElseError),
        CaseKeyword => p.report(ReservedKeywordError(CaseKeyword)),

        UseKeyword => p.report(DeclReservedKeywordError(UseKeyword)),
        TypeKeyword => p.report(DeclReservedKeywordError(TypeKeyword)),
        RecordKeyword => p.report(DeclReservedKeywordError(RecordKeyword)),
        ClassKeyword => p.report(DeclReservedKeywordError(ClassKeyword)),
        TraitKeyword => p.report(DeclReservedKeywordError(TraitKeyword)),
        InstanceKeyword => p.report(DeclReservedKeywordError(InstanceKeyword)),

        ReturnKeyword => p.report(StmtReservedKeywordError(ReturnKeyword)),
        WhereKeyword => p.report(StmtReservedKeywordError(WhereKeyword)),
        InKeyword => p.report(ReservedKeywordError(InKeyword)),

        LambdaUnicode => p.report(UnicodeError(LambdaUnicode, "lambda")),
        ForallUnicode => p.report(UnicodeError(LambdaUnicode, "forall")),
        PiUnicode => p.report(UnicodeError(LambdaUnicode, "pi")),
        SigmaUnicode => p.report(UnicodeError(LambdaUnicode, "sigma")),

        LeftBracket => p.report(UnicodeError(LeftBracket, "left_bracket")),
        RightBracket => p.report(UnicodeError(RightBracket, "right_bracket")),
        LeftBrace => p.report(UnicodeError(LeftBrace, "left_brace")),
        RightBrace => p.report(UnicodeError(RightBrace, "right_brace")),
        RightParen => p.report(UnicodeError(RightParen, "right_paren")),

        Comma => p.report(UnicodeError(Comma, "comma")),
        Semi => p.report(UnicodeError(Semi, "semi")),
        Colon => p.report(UnicodeError(Colon, "colon")),
        HelpSymbol => p.report(UnicodeError(HelpSymbol, "interrogation")),
        EqualSymbol => p.report(UnicodeError(EqualSymbol, "equal")),

        DoubleArrow => p.report(UnicodeError(DoubleArrow, "double_arrow")),
        RightArrow => p.report(UnicodeError(RightArrow, "right_arrow")),
        LeftArrow => p.report(UnicodeError(LeftArrow, "left_arrow")),

        _ => p.report(PrimaryExpectedError),
    }
}

fn last_semi(p: &mut Parser) {
    while !p.eof() && p.eat(Semi) {
        p.warning(UeselessSemiError);
    }
}

fn semi_eof(p: &mut Parser) -> bool {
    if !p.eat(Semi) {
        p.fixable(MissingSemiError, |token| {
            quickfix!(before, token.span, [Insert(";".into())])
        });
    } else if p.lookahead(0) == RightBrace {
        p.warning(UeselessSemiError);
    }

    while !p.eof() && p.eat(Semi) {
        p.warning(UeselessSemiError);
    }

    // returns if can continues
    !p.eof()
}

fn semi(p: &mut Parser) -> bool {
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
