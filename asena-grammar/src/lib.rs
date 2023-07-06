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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Semi {
    Optional,
    Required,
    OrNewLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlParser {
    Break,
    Continue,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Linebreak {
    Semi,
    Cont,
}

const PARAM_LIST_RECOVERY: &[TokenKind] = &[DoubleArrow, RightArrow, Semi, Colon, LeftBrace];

const STMT_RECOVERY: &[TokenKind] = &[
    LeftBrace,
    RightBrace,
    ClassKeyword,
    EnumKeyword,
    RecordKeyword,
    TypeKeyword,
    TraitKeyword,
    InstanceKeyword,
    UseKeyword,
];

const EXPR_RECOVERY: &[TokenKind] = &[
    LetKeyword,
    LeftBrace,
    ClassKeyword,
    EnumKeyword,
    RecordKeyword,
    TypeKeyword,
    TraitKeyword,
    InstanceKeyword,
    UseKeyword,
    Semi,
];

const ARRAY_RECOVERY: &[TokenKind] = &[Comma];

const METHOD_FIRST: &[TokenKind] = &[DefaultKeyword, FunKeyword];

const PAT_FIRST: &[TokenKind] = &[
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

const EXPR_FIRST: &[TokenKind] = &[
    LetKeyword,
    Identifier,
    LeftBracket,
    LeftParen,
    Str,
    TrueKeyword,
    FalseKeyword,
    MatchKeyword,
    IfKeyword,
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

const EXPR_FOLLOW: &[TokenKind] = &[Colon, Dot, RightArrow, DoubleArrow, Symbol];

/// File = Decl*
pub fn file(p: &mut Parser) {
    let m = p.open();

    while !p.eof() {
        decl(p);
    }

    p.close(m, File);
}

/// Decl = DeclUse | DeclCommand | DeclSignature | DeclAssign
pub fn decl(p: &mut Parser) {
    match p.lookahead(0) {
        UseKeyword => decl_use(p),
        HashSymbol => decl_command(p),
        EnumKeyword => decl_enum(p),
        ClassKeyword => decl_class(p),
        TraitKeyword => decl_trait(p),
        InstanceKeyword => decl_instance(p),
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

    if _semi(p, Semi::OrNewLine) {
        p.close(m, DeclCommand);
        return;
    }

    // Argument list
    rec_expr!(p, &[], ExpectedExprError, expr_dsl, Linebreak::Semi);
    while p.at(Comma) {
        p.expect(Comma);
        if p.at_any(EXPR_FIRST) {
            expr_dsl(p, Linebreak::Semi);
        } else if p.at_any(EXPR_RECOVERY) {
            p.report(ExpectedExprError);
            break;
        }
    }

    _semi(p, Semi::OrNewLine);
    p.close(m, DeclCommand);
}

/// DeclUse = 'use' Global
pub fn decl_use(p: &mut Parser) {
    let m = p.open();
    p.expect(UseKeyword);
    p.advance();
    while p.eat(Dot) && !p.eof() {
        p.advance();
    }
    _semi(p, Semi::Optional);
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
    rec_expr!(p, &[], ExpectedAssignValueError, expr_dsl, Linebreak::Semi);
    p.field("value");
    _semi(p, Semi::OrNewLine);
    p.close(m, DeclAssign);
}

/// DeclSignature = Global Param* ':' TypeExpr
pub fn decl_signature(p: &mut Parser) {
    let m = p.open();
    global(p);
    params(p);
    if p.eat(Colon) {
        type_expr(p, Linebreak::Semi);
    }
    where_clause(p);
    if p.at(LeftBrace) {
        _stmt_block(p);
    }
    _semi(p, Semi::OrNewLine);
    p.close(m, DeclSignature);
}

pub fn decl_trait(p: &mut Parser) {
    let m = p.open();
    p.expect(TraitKeyword);
    global(p);
    params(p);
    if p.at(WhereKeyword) {
        where_clause(p);
    }

    p.expect(LeftBrace);
    _trait_fields(p);
    _trait_methods(p);
    p.expect(RightBrace);
    _semi(p, Semi::OrNewLine);
    p.close(m, DeclTrait);
}

pub fn _trait_methods(p: &mut Parser) {
    while p.at_any(METHOD_FIRST) {
        trait_default(p);
    }
}

pub fn _trait_fields(p: &mut Parser) {
    let mut comma_count = 0;

    if !p.at(RightBrace) && p.at(Identifier) {
        class_field(p);
    }

    while !p.eof() && !p.at(RightBrace) {
        p.expect(Comma);
        if p.at(Comma) {
            if comma_count > 0 {
                p.report(UselessCommaError);
            }
            comma_count += 1;
            continue;
        } else if p.at(Identifier) {
            class_field(p);
        } else if p.at(RightBrace) || p.at_any(METHOD_FIRST) {
            break;
        } else {
            p.report(ExpectedFieldError);
            break;
        }
    }
}

pub fn decl_instance(p: &mut Parser) {
    let m = p.open();
    p.expect(InstanceKeyword);
    params(p);
    type_expr(p, Linebreak::Cont);
    where_clause(p);
    p.expect(LeftBrace);
    _instance_impls(p);
    p.expect(RightBrace);
    _semi(p, Semi::OrNewLine);
    p.close(m, DeclInstance);
}

pub fn _instance_impls(p: &mut Parser) {
    while !p.at(RightBrace) && !p.eof() {
        if p.at(Identifier) {
            instance_impl(p);
        } else if p.at_any(METHOD_FIRST) {
            class_method(p);
        } else {
            p.report(ExpectedImplError);
            break;
        }
    }
}

pub fn decl_class(p: &mut Parser) {
    let m = p.open();
    p.expect(ClassKeyword);
    global(p);
    params(p);
    where_clause(p);
    p.expect(LeftBrace);
    _class_fields(p);
    _class_methods(p);
    p.expect(RightBrace);
    p.close(m, DeclClass);
}

pub fn _class_methods(p: &mut Parser) {
    while p.at_any(METHOD_FIRST) {
        class_method(p);
    }
}

pub fn _class_fields(p: &mut Parser) {
    if !p.at(RightBrace) && p.at(Identifier) {
        class_field(p);
    }

    let mut comma_count = 0;
    while !p.eof() && !p.at(RightBrace) {
        p.expect(Comma);
        if p.at(Comma) {
            if comma_count > 0 {
                p.report(UselessCommaError);
            }
            comma_count += 1;
            continue;
        } else if p.at(Identifier) {
            class_field(p);
        } else if p.at(RightBrace) || p.at_any(METHOD_FIRST) {
            break;
        } else {
            p.report(ExpectedFieldError);
            break;
        }
    }
}

/// DeclEnum = 'enum' Global Params? GadtType? WhereClause? '{' EnumVariant* ClassMethod* '}'
pub fn decl_enum(p: &mut Parser) {
    let m = p.open();
    p.expect(EnumKeyword);
    global(p);
    params(p);
    _enum_gadt_type(p);
    where_clause(p);
    p.expect(LeftBrace);
    _enum_variants(p);
    _enum_methods(p);
    p.expect(RightBrace);
    p.close(m, DeclEnum);
}

pub fn _enum_variants(p: &mut Parser) {
    let mut comma_count = 0;

    if !p.at(RightBrace) && p.at(Identifier) {
        enum_variant(p);
    }

    while !p.eof() && !p.at(RightBrace) {
        p.expect(Comma);
        if p.at(Comma) {
            if comma_count > 0 {
                p.report(UselessCommaError);
            }
            comma_count += 1;
            continue;
        } else if p.at(Identifier) {
            enum_variant(p);
        } else if p.at(RightBrace) || p.at_any(METHOD_FIRST) {
            break;
        } else {
            p.report(ExpectedVariantError);
            break;
        }
    }
}

pub fn _enum_methods(p: &mut Parser) {
    while p.at_any(METHOD_FIRST) {
        class_method(p);
    }
}

pub fn _enum_gadt_type(p: &mut Parser) {
    if !p.at(Colon) {
        return;
    }
    p.expect(Colon);
    type_expr(p, Linebreak::Cont);
}

pub fn enum_variant(p: &mut Parser) {
    let m = p.open();
    global(p);
    match p.lookahead(0) {
        Colon => {
            p.expect(Colon);
            type_expr(p, Linebreak::Cont);
            p.close(m, VariantType);
        }
        LeftParen => {
            p.expect(LeftParen);
            if !p.at(RightParen) && p.at_any(EXPR_FIRST) {
                rec_expr!(
                    p,
                    &[],
                    ExpectedVariantParameterError,
                    type_expr,
                    Linebreak::Cont
                );
            }
            let mut comma_count = 0;
            while !p.eof() && !p.at(RightParen) {
                p.expect(Comma);
                if p.at(Comma) {
                    if comma_count > 0 {
                        p.report(UselessCommaError);
                    }
                    comma_count += 1;
                    continue;
                } else if p.at_any(EXPR_FIRST) {
                    rec_expr!(
                        p,
                        &[],
                        ExpectedVariantParameterError,
                        type_expr,
                        Linebreak::Cont
                    );
                } else if p.at_any(PARAM_LIST_RECOVERY) {
                    p.report(ExpectedParameterError);
                    break;
                }
            }
            p.expect(RightParen);
            p.close(m, VariantConstructor);
        }
        _ => {
            p.close(m, VariantConstructor);
        }
    }
}

pub fn class_field(p: &mut Parser) {
    let m = p.open();
    p.expect(Identifier);
    p.expect(Colon);
    type_expr(p, Linebreak::Cont);
    p.close(m, ClassField);
}

pub fn trait_default(p: &mut Parser) {
    let m = p.open();

    p.expect(DefaultKeyword);
    global(p);
    params(p);
    if p.eat(Colon) {
        type_expr(p, Linebreak::Cont);
    }
    where_clause(p);
    _stmt_block(p);

    p.close(m, TraitDefault);
}

pub fn instance_impl(p: &mut Parser) {
    if p.at_any(METHOD_FIRST) {
        class_method(p);
        p.report(MethodNotAllowedInInstanceError);
    }

    let m = p.open();
    global(p);
    while !p.eof() && !p.at(EqualSymbol) {
        pat(p);
    }
    p.expect(EqualSymbol);
    rec_expr!(p, &[], ExpectedImplValueError, expr_dsl, Linebreak::Semi);

    _semi(p, Semi::OrNewLine);
    p.close(m, InstanceImpl);
}

pub fn class_method(p: &mut Parser) {
    let m = p.open();
    p.expect(FunKeyword);
    global(p);
    params(p);
    if p.eat(Colon) {
        type_expr(p, Linebreak::Cont);
        where_clause(p);

        if p.at(LeftBrace) {
            _stmt_block(p);
        }
    } else {
        where_clause(p);
        _stmt_block(p);
    }

    p.close(m, ClassMethod);
}

pub fn where_clause(p: &mut Parser) {
    if !p.at(WhereKeyword) {
        return;
    }

    let m = p.open();
    p.expect(WhereKeyword);
    while !p.eof() && p.at_any(EXPR_FIRST) {
        constraint(p);
    }
    p.close(m, WhereClause);
}

pub fn constraint(p: &mut Parser) {
    let m = p.open();
    type_expr(p, Linebreak::Cont);
    p.close(m, TypeConstraint);
}

pub fn params(p: &mut Parser) {
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
}

/// Param = ImplicitParam | ExplicitParam
pub fn param(p: &mut Parser) -> bool {
    let m = p.open();
    let token = p.peek();

    match token.kind {
        LeftParen => {
            p.expect(LeftParen);
            if p.at(SelfKeyword) {
                p.expect(SelfKeyword);
                p.expect(RightParen);
                p.close(m, SelfParam);
                return false;
            }
            p.expect(Identifier);
            p.expect(Colon);
            type_expr(p, Linebreak::Cont);
            p.expect(RightParen);
        }
        LeftBracket => {
            p.expect(LeftBracket);
            type_expr(p, Linebreak::Cont);
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
        LetKeyword => {
            if let Some(let_stmt) = p.savepoint().run(stmt_let).as_succeded() {
                p.return_at(let_stmt);
                return false;
            }

            stmt_expr(p);
        }
        IfKeyword => stmt_if(p),
        _ => {
            if p.at_any(PAT_FIRST) {
                if let Some(ask) = p.savepoint().run(stmt_ask).as_succeded() {
                    p.return_at(ask);
                    return false;
                }
            }
            if p.at_any(EXPR_FIRST) {
                stmt_expr(p);
                return false;
            }
            return true;
        }
    }
    p.at_any(STMT_RECOVERY)
}

pub fn stmt_return(p: &mut Parser) {
    let m = p.open();
    p.expect(ReturnKeyword);
    if p.at_any(EXPR_FIRST) {
        rec_expr!(p, &[], ExpectedReturnValueError, expr_dsl, Linebreak::Semi);
    } else if p.at_any(STMT_RECOVERY) {
        p.report(ExpectedReturnStmtError);
    }
    _semi(p, Semi::OrNewLine);
    p.close(m, StmtReturn);
}

pub fn stmt_ask(p: &mut Parser) {
    let m = p.open();
    pat(p);
    p.expect(LeftArrow);
    rec_expr!(p, &[], ExpectedAskValueError, expr_dsl, Linebreak::Semi);
    _semi(p, Semi::OrNewLine);
    p.close(m, StmtAsk);
}

pub fn stmt_if(p: &mut Parser) {
    let m = p.open();
    p.expect(IfKeyword);
    rec_expr!(p, &[], ExpectedIfCondError);
    if_then(p);
    if p.at(ElseKeyword) {
        if_else(p, Linebreak::Semi);
    }
    _semi(p, Semi::OrNewLine);
    p.close(m, StmtIf);
}

pub fn stmt_let(p: &mut Parser) {
    let m = p.open();
    p.expect(LetKeyword);
    pat(p);
    p.expect(EqualSymbol);
    rec_expr!(p, &[], ExpectedLetValueError, expr_dsl, Linebreak::Semi);
    _semi(p, Semi::OrNewLine);
    p.close(m, StmtLet);
}

pub fn stmt_expr(p: &mut Parser) {
    let m = p.open();
    rec_expr!(p, &[], ExpectedExprError, expr_dsl, Linebreak::Semi);
    _semi(p, Semi::OrNewLine);
    p.close(m, StmtExpr);
}

/// TypeExpr = Expr
pub fn type_expr(p: &mut Parser, linebreak: Linebreak) {
    let m = p.open();
    rec_expr!(p, &[], ExpectedTypeError, expr, linebreak);
    p.close(m, TypeExplicit);
}

/// Expr =
///   ExprGroup
/// | ExprBinary | ExprAccessor | ExprApp
/// | ExprDsl | ExprArray | ExprLam
/// | ExprLet | ExprGlobal | ExprLocal
/// | ExprLit | ExprAnn | ExprQual
/// | ExprPi | ExprSigma | ExprHelp
pub fn expr(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let token = p.peek();
    match token.kind {
        Symbol if token.text == "\\" => expr_lam(p, linebreak),
        HelpSymbol => expr_help(p, linebreak),
        IfKeyword => expr_if(p, linebreak),
        LetKeyword => expr_let(p, linebreak),
        MatchKeyword => expr_match(p),
        _ => expr_ann(p, linebreak),
    }
}

pub fn expr_let(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();
    p.expect(LetKeyword);
    pat(p);
    p.expect(EqualSymbol);
    rec_expr!(p, &[], ExpectedLetValueError, expr_dsl, Linebreak::Cont);
    p.expect(InKeyword);
    rec_expr!(p, &[], ExpectedLetInValueError, expr_dsl, linebreak);
    p.close(m, ExprLet).into()
}

pub fn if_then(p: &mut Parser) {
    let m = p.open();

    match p.lookahead(0) {
        ThenKeyword => {
            p.expect(ThenKeyword);
            rec_expr!(p, &[], ExpectedIfThenExprError, expr_dsl, Linebreak::Cont);
            p.close(m, BranchExpr);
        }
        LeftBrace => {
            _stmt_block(p);
            p.close(m, BranchBlock);
        }
        _ => {
            p.report(ExpectedIfThenError);
            p.close(m, BranchExpr);
        }
    }
}

pub fn if_else(p: &mut Parser, linebreak: Linebreak) {
    let m = p.open();
    p.expect(ElseKeyword);
    match p.lookahead(0) {
        LeftBrace => {
            _stmt_block(p);
            p.close(m, BranchBlock);
        }
        _ if p.at_any(EXPR_FIRST) => {
            rec_expr!(p, &[], ExpectedIfElseExprError, expr, linebreak);
            p.close(m, BranchExpr);
        }
        _ => {
            p.report(ExpectedIfElseError);
            p.close(m, BranchExpr);
        }
    }
}

/// ExprIf = 'if' Expr 'then' Expr 'else' Expr
pub fn expr_if(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();
    p.expect(IfKeyword);
    rec_expr!(p, &[], ExpectedIfCondError);
    if_then(p);
    if_else(p, linebreak);
    p.close(m, ExprIf).into()
}

pub fn case(p: &mut Parser) {
    let m = p.open();
    pat_app(p);
    p.expect(DoubleArrow);
    case_branch(p);
    p.close(m, MatchCase);
}

pub fn case_branch(p: &mut Parser) {
    let m = p.open();
    match p.lookahead(0) {
        LeftBrace => {
            _stmt_block(p);
            p.close(m, BranchBlock);
        }
        _ if p.at_any(EXPR_FIRST) => {
            // TODO: FIXME
            rec_expr!(p, &[], ExpectedCaseExprError, expr, Linebreak::Cont);
            p.close(m, BranchExpr);
        }
        _ => {
            p.report(ExpectedCaseError);
        }
    }
}

/// ExprMatch = 'match' Expr '{' Case* '}'
pub fn expr_match(p: &mut Parser) -> Option<MarkClosed> {
    let m = p.open();
    p.expect(MatchKeyword);
    rec_expr!(p, &[], ExpectedMatchScrutineeError, expr, Linebreak::Cont);
    p.expect(LeftBrace);
    if !p.at(RightBrace) && p.at_any(PAT_FIRST) {
        case(p);
    }
    let mut comma_count = 0;
    while !p.eof() && !p.at(RightBrace) {
        p.expect(Comma);
        if p.at(Comma) {
            if comma_count > 0 {
                p.report(UselessCommaError);
            }
            comma_count += 1;
            continue;
        } else {
            case(p);
        }
    }
    p.expect(RightBrace);
    p.close(m, ExprMatch).into()
}

/// ExprAnn = ExprQual (':' ExprQual)*
pub fn expr_ann(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();

    rec_expr!(p, &[], ExpectedExprError, expr_qual, linebreak);

    // simplify by returning the lhs symbol directly
    if p.at(Colon) {
        while !p.eof() && p.eat(Colon) {
            if rec_expr!(p, &[], ExpectedAnnAgainstError, expr_qual, linebreak) {
                break;
            }
        }

        p.close(m, ExprAnn).into()
    } else {
        p.abandon(m);
        None
    }
}

/// ExprQual = ExprAnonymousPi ('=>' ExprAnonymousPi)*
pub fn expr_qual(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();

    rec_expr!(p, &[], ExpectedExprError, expr_anon_pi, linebreak);

    // simplify by returning the lhs symbol directly
    if p.at(DoubleArrow) {
        while !p.eof() && p.eat(DoubleArrow) {
            if rec_expr!(p, &[], ExpectedQualReturnError, expr_anon_pi, linebreak) {
                break;
            }
        }

        p.close(m, ExprQual).into()
    } else {
        p.abandon(m);
        None
    }
}

/// ExprAnonymousPi = ExprAccessor ('->' ExprAccessor)*
pub fn expr_anon_pi(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();

    rec_expr!(p, &[], ExpectedExprError, expr_binary, linebreak);

    // simplify by returning the lhs symbol directly
    if p.at(RightArrow) {
        while !p.eof() && p.eat(RightArrow) {
            if rec_expr!(p, &[], ExpectedPiReturnError, expr_binary, linebreak) {
                break;
            }
        }

        p.close(m, ExprPi).into()
    } else {
        p.abandon(m);
        None
    }
}

/// ExprHelp = '?' ExprDsl
pub fn expr_help(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();
    p.expect(HelpSymbol);
    rec_expr!(p, &[], ExpectedHelpValueError, expr_dsl, linebreak);
    p.close(m, ExprHelp).into()
}

/// ExprLam = '\' Identifier* '->' ExprDsl
pub fn expr_lam(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();
    p.advance();
    while !p.eof() && !p.at(RightArrow) {
        let m = p.open();
        p.expect(Identifier);
        p.close(m, LamParam);
    }
    p.expect(RightArrow);
    rec_expr!(p, &[], ExpectedLamBodyError, expr_dsl, linebreak);
    p.close(m, ExprLam).into()
}

pub fn expr_dsl(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let m = p.open();
    rec_expr!(p, &[], ExpectedExprError, expr, linebreak);

    if p.at(LeftBrace) {
        _stmt_block(p);
        p.close(m, ExprDsl).into()
    } else {
        p.abandon(m);
        None
    }
}

/// ExprBinary = ExprAccessor (Symbol ExprAccessor)*
pub fn expr_binary(p: &mut Parser, linebreak: Linebreak) {
    let m = p.open();

    rec_expr!(p, &[], ExpectedExprError, expr_app, linebreak);

    // simplify by returning the lhs symbol directly
    if p.at(Symbol) {
        while !p.eof() && p.eat(Symbol) {
            if rec_expr!(p, &[], ExpectedInfixRhsError, expr_app, linebreak) {
                break;
            }
        }

        p.close(m, ExprBinary);
    } else {
        p.abandon(m)
    }
}

/// ExprAccessor = ExprApp ('.' Accessor)*
// pub fn expr_accessor(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
//     let m = p.open();

//     rec_expr!(p, &[], ExpectedExprError, expr_app, linebreak);

//     // simplify by returning the lhs symbol directly
//     if p.at(Dot) {
//         while !p.eof() && p.at(Dot) {
//             accessor(p, linebreak);
//         }

//         p.close(m, ExprAccessor).into()
//     } else {
//         p.abandon(m);
//         None
//     }
// }

// pub fn accessor(p: &mut Parser, linebreak: Linebreak) {
//     let m = p.open();
//     p.expect(Dot);
//     p.expect(Identifier);
//     while !p.eof() && p.at_any(PAT_FIRST) {
//         if arg.has_errors() && p.eof() {
//             break;
//         } else if arg.has_errors() {
//             p.report(ExpectedAccessorArgExprError);
//             break;
//         } else {
//             p.return_at(arg);
//         }
//     }
//     p.close(m, AccessorArg);
// }

/// ExprApp = Primary Primary*
pub fn expr_app(p: &mut Parser, linebreak: Linebreak) -> Option<MarkClosed> {
    let mut lhs = match primary(p) {
        Some(lhs) => lhs,
        None if p.eof() => {
            p.report(PrimaryExpectedError);
            return None;
        }
        None => MarkClosed::new(0, p.peek().span().clone()),
    };

    while !p.eof() {
        match linebreak {
            Linebreak::Semi if p.at_newline(0) && !EXPR_FOLLOW.contains(&p.lookahead(0)) => {
                break;
            }
            Linebreak::Cont | Linebreak::Semi => {
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
    }

    lhs.into()
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
    rec_expr!(
        p,
        &[RightParen],
        ExpectedExprError,
        expr_dsl,
        Linebreak::Cont
    );
    p.expect(RightParen);
    p.close(m, ExprGroup)
}

pub fn expr_array(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    p.expect(LeftBracket);
    if !p.at(RightBracket) {
        expr_dsl(p, Linebreak::Cont);
    }
    while !p.eof() && !p.at(RightBracket) {
        p.expect(Comma);
        if p.at_any(EXPR_FIRST) {
            expr_dsl(p, Linebreak::Cont);
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
            while p.at(Dot) && !p.eof() {
                p.advance();
                p.expect(Identifier);
            }
            p.close(m, ExprLocal)
        }
        SelfKeyword => {
            let m = p.open();
            p.advance();
            p.close(m, ExprSelf)
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
            // '(' Identifier ':' TypeExpr ')' '->' 'TypeExpr' # Pi
            //
            //  ^  ^
            //  |  |
            //
            //  0  1
            if let Some((closed, pi)) = p.savepoint().as_closed(expr_pi) {
                p.return_at(pi);
                return Some(closed);
            }

            expr_group(p)
        }
        _ => return _non_primary(p, token.value.kind).and(None),
    };

    Some(result)
}

pub fn pat_app(p: &mut Parser) {
    if let Some(pat_constructor) = p.savepoint().run(pat_constructor).as_succeded() {
        p.return_at(pat_constructor);
    } else {
        pat(p);
    }
}

pub fn pat_constructor(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    global(p);
    if p.at(DoubleArrow) || p.at(EqualSymbol) {
        return p.close(m, PatConstructor);
    }
    while !p.eof() && p.at_any(PAT_FIRST) {
        pat(p);
        if p.at(DoubleArrow) || p.at(EqualSymbol) {
            break;
        }
    }
    p.close(m, PatConstructor)
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
            pat_constructor(p);
            p.expect(RightParen);
            p.close(m, PatGroup)
        }
        _ => return _non_primary(p, token.value.kind).and(None),
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

fn _non_primary(p: &mut Parser, kind: TokenKind) -> Option<MarkClosed> {
    match kind {
        Eof => p.report(EofError),
        Symbol => p.report(ExpectedTokenError(Identifier)),

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

fn _semi(p: &mut Parser, mode: Semi) -> bool {
    match mode {
        Semi::Optional | Semi::OrNewLine => {
            let v = p.at_newline(1) || p.at(Semi);
            while p.eat(Semi) {}
            v
        }
        Semi::Required => {
            if !p.eat(Semi) {
                p.fixable(MissingSemiError, |token| {
                    quickfix!(before, token.span, [Insert(";".into())])
                });
            }

            while !p.eof() && p.eat(Semi) {
                p.warning(UeselessSemiError);
            }

            // returns if can continues
            // generally the end of the statement block is RightBrace
            !p.at(RightBrace) && !p.eof()
        }
    }
}

fn _stmt_block(p: &mut Parser) {
    p.expect(LeftBrace);
    while !p.eof() && !p.at(RightBrace) {
        if stmt(p) {
            p.report(ExpectedStmtError);
            continue;
        }
    }
    p.expect(RightBrace);
}

macro_rules! rec_expr {
    ($p:expr, $recovery:expr) => {
        $crate::rec_expr!($p, $recovery, ExpectedExprError)
    };
    ($p:expr, $recovery:expr, $error:expr) => {
        $crate::rec_expr!(
            $p,
            $recovery,
            ExpectedExprError,
            $crate::expr,
            $crate::Linebreak::Semi
        )
    };
    ($p:expr, $recovery:expr, $error:expr, $f:expr, $linebreak:expr) => {
        if $p.at_any(EXPR_FIRST) {
            $f($p, $linebreak);
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
