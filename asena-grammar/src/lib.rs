use asena_leaf::node::TreeKind;
use asena_leaf::node::TreeKind::*;
use asena_leaf::token::kind::TokenKind;
use asena_leaf::token::kind::TokenKind::*;

use asena_parser::error::ParseError::*;
use asena_parser::event::MarkClosed;
use asena_parser::Parser;

pub mod macros;

use asena_report::quickfix;
use asena_report::Fragment::Insert;
use asena_report::Quickfix;
pub use macros::*;

const PARAM_LIST_RECOVERY: &[TokenKind] = &[DoubleArrow, RightArrow, Semi, Colon, LeftBrace];

const STMT_RECOVERY: &[TokenKind] = &[
    LeftBrace,
    ClassKeyword,
    EnumKeyword,
    RecordKeyword,
    TypeKeyword,
    TraitKeyword,
    UseKeyword,
];

const EXPR_RECOVERY: &[TokenKind] = &[
    LeftBrace,
    ClassKeyword,
    EnumKeyword,
    RecordKeyword,
    TypeKeyword,
    TraitKeyword,
    UseKeyword,
    Semi,
];

const ARRAY_RECOVERY: &[TokenKind] = &[Comma];

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
            if let Some(decl) = p.savepoint().run(decl_assign).as_succeded() {
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
    rec_expr!(p, &[]);
    while p.at(Comma) {
        p.expect(Comma);
        if p.at(Semi) || p.eof() {
            break;
        }
        if rec_expr!(p, &[]) {
            break;
        }
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
    rec_expr!(p, &[], ExpectedAssignValueError, expr_dsl);
    p.field("value");
    p.close(m, DeclAssign);
}

/// DeclSignature = Global Param* ':' TypeExpr
pub fn decl_signature(p: &mut Parser) {
    let m = p.open();
    global(p);
    p.field("name");
    while !p.eof() && p.at(LeftParen) || p.at(LeftBracket) {
        if p.at_any(PARAM_LIST_RECOVERY) {
            p.report(ExpectedParameterError);
            break;
        } else if p.at(Comma) {
            p.report(ParameterIsCurryiedAndNotTupleError);
            continue;
        } else {
            param(p);
        }
    }
    if p.eat(Colon) {
        type_expr(p);
        p.field("type");
        if p.eat(LeftBrace) {
            if !p.at(RightBrace) && stmt(p) {
                p.report(ExpectedStmtError);
            }
            while !p.eof() && !p.at(RightBrace) && semi(p) {
                if stmt(p) {
                    p.report(ExpectedStmtError);
                    break;
                }
            }
            last_semi(p);
            p.expect(RightBrace);
        }
    } else {
        p.expect(LeftBrace);
        if !p.at(RightBrace) && stmt(p) {
            p.report(ExpectedStmtError);
        }
        while !p.eof() && !p.at(RightBrace) && semi(p) {
            if stmt(p) {
                p.report(ExpectedStmtError);
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
            if let Some(ask) = p.savepoint().run(stmt_ask).as_succeded() {
                p.return_at(ask);
                return false;
            } else if p.at_any(EXPR_FIRST) {
                stmt_expr(p)
            } else if p.at_any(STMT_RECOVERY) {
                return true;
            }
        }
    }
    false
}

pub fn stmt_return(p: &mut Parser) {
    let m = p.open();
    p.expect(ReturnKeyword);
    if p.at_any(EXPR_FIRST) {
        rec_expr!(p, &[], ExpectedReturnValueError, expr_dsl);
    } else if p.at_any(STMT_RECOVERY) {
        p.report(ExpectedReturnStmtError);
    }
    p.close(m, StmtReturn);
}

pub fn stmt_ask(p: &mut Parser) {
    let m = p.open();
    pat(p);
    p.expect(LeftArrow);
    rec_expr!(p, &[], ExpectedAskValueError, expr_dsl);
    p.close(m, StmtAsk);
}

pub fn stmt_let(p: &mut Parser) {
    let m = p.open();
    p.expect(LetKeyword);
    pat(p);
    p.expect(EqualSymbol);
    rec_expr!(p, &[], ExpectedLetValueError, expr_dsl);
    p.close(m, StmtLet);
}

pub fn stmt_expr(p: &mut Parser) {
    let m = p.open();
    rec_expr!(p, &[], ExpectedExprError, expr_dsl);
    p.close(m, StmtExpr);
}

/// TypeExpr = Expr
pub fn type_expr(p: &mut Parser) {
    let m = p.open();
    rec_expr!(p, &[], ExpectedTypeError, expr);
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

    rec_expr!(p, &[], ExpectedExprError, expr_qual);

    // simplify by returning the lhs symbol directly
    if p.at(Colon) {
        while !p.eof() && p.eat(Colon) {
            if rec_expr!(p, &[], ExpectedAnnAgainstError, expr_qual) {
                break;
            }
        }

        p.close(m, ExprAnn);
    } else {
        p.ignore(m)
    }
}

/// ExprQual = ExprAnonymousPi ('=>' ExprAnonymousPi)*
pub fn expr_qual(p: &mut Parser) {
    let m = p.open();

    rec_expr!(p, &[], ExpectedExprError, expr_anonymous_pi);

    // simplify by returning the lhs symbol directly
    if p.at(DoubleArrow) {
        while !p.eof() && p.eat(DoubleArrow) {
            if rec_expr!(p, &[], ExpectedQualReturnError, expr_anonymous_pi) {
                break;
            }
        }

        p.close(m, ExprQual);
    } else {
        p.ignore(m)
    }
}

/// ExprAnonymousPi = ExprAccessor ('->' ExprAccessor)*
pub fn expr_anonymous_pi(p: &mut Parser) {
    let m = p.open();

    rec_expr!(p, &[], ExpectedExprError, expr_binary);

    // simplify by returning the lhs symbol directly
    if p.at(RightArrow) {
        while !p.eof() && p.eat(RightArrow) {
            if rec_expr!(p, &[], ExpectedPiReturnError, expr_binary) {
                break;
            }
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
    rec_expr!(p, &[], ExpectedHelpValueError, expr_dsl);
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
    rec_expr!(p, &[], ExpectedLamBodyError, expr_dsl);
    p.close(m, ExprLam);
}

pub fn expr_dsl(p: &mut Parser) {
    let m = p.open();
    rec_expr!(p, &[]);

    if p.eat(LeftBrace) {
        if !p.at(RightBrace) && stmt(p) {
            p.report(ExpectedStmtError);
        }
        while !p.eof() && !p.at(RightBrace) && semi(p) {
            if stmt(p) {
                p.report(ExpectedStmtError);
                break;
            }
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
            if rec_expr!(p, &[], ExpectedInfixRhsError, expr_accessor) {
                break;
            }
        }

        p.close(m, ExprBinary);
    } else {
        p.ignore(m)
    }
}

/// ExprAccessor = ExprApp ('.' ExprApp)*
pub fn expr_accessor(p: &mut Parser) {
    let m = p.open();

    rec_expr!(p, &[], ExpectedExprError, expr_app);

    // simplify by returning the lhs symbol directly
    if p.at(Dot) {
        while !p.eof() && p.eat(Dot) {
            if rec_expr!(p, &[], ExpectedFieldError, expr_app) {
                break;
            }
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
        if !p.eat(Colon) {
            p.report(ExpectedParameterTypeError);
        }
    }
    rec_expr!(p, &[RightParen, RightArrow], ExpectedPiParamError);
    p.field("parameter_type");
    while p.at(Comma) && !p.eof() {
        p.report(ParameterIsCurryiedAndNotTupleError);
    }
    p.expect(RightParen);
    p.expect(RightArrow);
    rec_expr!(p, &[], ExpectedPiParamError);
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
    rec_expr!(p, &[RightBracket, RightArrow], ExpectedSigmaParamError);
    p.field("parameter_type");
    p.expect(RightBracket);
    p.expect(RightArrow);
    rec_expr!(p, &[], ExpectedSigmaReturnError);
    p.field("return_type");
    p.close(m, ExprSigma)
}

pub fn expr_group(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    p.expect(LeftParen);
    if p.eat(RightParen) {
        return p.close(m, ExprUnit);
    }
    if p.at_any(EXPR_FIRST) {
        expr_dsl(p);
    } else if p.at_any(EXPR_RECOVERY) {
        p.report(ExpectedExprError);
    }
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
        if p.at_any(EXPR_FIRST) {
            expr_dsl(p);
        } else if p.at_any(EXPR_RECOVERY) {
            p.report(ExpectedExprError);
            break;
        } else if p.at_any(ARRAY_RECOVERY) {
            p.report(ExpectedExprAndCloseListError);
            continue;
        }
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
            if let Some((closed, sigma)) = p.savepoint().as_closed(expr_sigma) {
                p.return_at(sigma);
                return Some(closed);
            }

            expr_array(p)
        }
        // Parse group or named pi expressions
        // - Pi
        // - Group
        LeftParen => {
            if let Some((closed, pi)) = p.savepoint().as_closed(expr_pi) {
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

macro_rules! rec_expr {
    ($p:expr, $recovery:expr) => {
        $crate::rec_expr!($p, $recovery, ExpectedExprError)
    };
    ($p:expr, $recovery:expr, $error:expr) => {
        $crate::rec_expr!($p, $recovery, $error, $crate::expr)
    };
    ($p:expr, $recovery:expr, $error:expr, $f:expr) => {
        if $p.at_any(EXPR_FIRST) {
            $f($p);
            false
        } else {
            let mut new_recovery = EXPR_RECOVERY.clone().to_vec();
            new_recovery.append(&mut $recovery.to_vec());
            if $p.at_any(new_recovery.as_slice()) {
                $p.report($error);
                true
            } else {
                false
            }
        }
    };
}

use rec_expr;
