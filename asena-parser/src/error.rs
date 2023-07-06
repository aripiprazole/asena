use thiserror::Error;

use asena_leaf::named::Named;
use asena_leaf::node::kind::TokenKind;
use asena_report::InternalError;
use asena_span::Spanned;

pub type Result<T, E = Spanned<ParseError>> = std::result::Result<T, E>;

/// Parsing errors, it can be indexed to the code using a [Spanned<ParseError>].
#[derive(Error, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ParseError {
    #[error("[internal] the stack should contain the last close element")]
    EmptyStackError,

    #[error("[internal] the stack should contain just the tree element but has {} elements", .0)]
    StackError(usize),

    #[error("[internal] the token stream still contain something: `{}`", .0)]
    StreamStillContainElements(TokenKind),

    #[error("could not parse primary")]
    PrimaryExpectedError,

    #[error("unexpected token")]
    UnexpectedTokenError,

    #[error("missing semicolon")]
    MissingSemiError,

    #[error("expression should be surrounded by parenthesis")]
    PrimarySurroundedError(TokenKind),

    #[error("found an `else` without previous `if` node")]
    DanglingElseError,

    #[error("found unicode `{}`, you can rewrite in the language norm as `{}`", .0, .1)]
    UnicodeError(TokenKind, &'static str),

    #[error("reserved keyword `{}` in the wrong position, must be a constraint", .0.name())]
    ConstraintReservedKeywordError(TokenKind),

    #[error("reserved keyword `{}` in the wrong position, must be a statement", .0.name())]
    StmtReservedKeywordError(TokenKind),

    #[error("reserved keyword `{}` in the wrong position, must be a top-level declaration", .0.name())]
    DeclReservedKeywordError(TokenKind),

    #[error("reserved keyword `{}` to be used in a feature", .0.name())]
    ReservedKeywordError(TokenKind),

    #[error("invalid identifier, found symbol")]
    InvalidSymbolIdentifierError,

    #[error("invalid identifier, found unicode symbol")]
    InvalidUnicodeIdentifierError,

    #[error("expected `{}`", .0.name())]
    ExpectedTokenError(TokenKind),

    #[error("expected expression and close list")]
    ExpectedExprAndCloseListError,

    #[error("expected expression and close parameters")]
    ExpectedExprAndCloseParamsError,

    #[error("expected expression")]
    ExpectedExprError,

    #[error("expected accessor argument")]
    ExpectedAccessorArgExprError,

    #[error("expected match scrutinee")]
    ExpectedMatchScrutineeError,

    #[error("expected case expr")]
    ExpectedCaseExprError,

    #[error("expected case")]
    ExpectedCaseError,

    #[error("expected if condition")]
    ExpectedIfCondError,

    #[error("expected if else expression")]
    ExpectedIfElseExprError,

    #[error("expected if else")]
    ExpectedIfElseError,

    #[error("expected if then expression")]
    ExpectedIfThenExprError,

    #[error("expected if then or open braces")]
    ExpectedIfThenError,

    #[error("expected field")]
    ExpectedFieldError,

    #[error("expected impl function")]
    ExpectedImplError,

    #[error("expected rhs of infix")]
    ExpectedInfixRhsError,

    #[error("expected pattern")]
    ExpectedPatError,

    #[error("expected sigma parameter type")]
    ExpectedSigmaParamError,

    #[error("expected sigma return type")]
    ExpectedSigmaReturnError,

    #[error("expected pi parameter type")]
    ExpectedPiParamError,

    #[error("expected pi return type")]
    ExpectedPiReturnError,

    #[error("expected qual return type")]
    ExpectedQualReturnError,

    #[error("expected ann against")]
    ExpectedAnnAgainstError,

    #[error("expected help value")]
    ExpectedHelpValueError,

    #[error("expected lam body")]
    ExpectedLamBodyError,

    #[error("expected impl value")]
    ExpectedImplValueError,

    #[error("expected assign value")]
    ExpectedAssignValueError,

    #[error("expected ask name")]
    ExpectedAskNameError,

    #[error("expected parameter type")]
    ExpectedParameterTypeError,

    #[error("expected ask value")]
    ExpectedAskValueError,

    #[error("expected type")]
    ExpectedTypeError,

    #[error("expected patterns")]
    ExpectedPatternsError,

    #[error("expected let name")]
    ExpectedLetNameError,

    #[error("expected let value")]
    ExpectedLetValueError,

    #[error("expected let in value")]
    ExpectedLetInValueError,

    #[error("expected return value")]
    ExpectedReturnValueError,

    #[error("expected return stmt")]
    ExpectedReturnStmtError,

    #[error("could not parse anything, found end of file")]
    EofError,

    #[error("only one where clause is permitted")]
    OnlyOneWhereClauseIsPermittedError,

    #[error("expected parameter")]
    ExpectedParameterError,

    #[error("expected implementations, not methods")]
    MethodNotAllowedInInstanceError,

    #[error("expected parameter to be a tuple")]
    ParameterIsCurryiedAndNotTupleError,

    #[error("expected variant")]
    ExpectedVariantError,

    #[error("expected variant parameter")]
    ExpectedVariantParameterError,

    #[error("expected statement")]
    ExpectedStmtError,

    #[error("unfinished parenthesis, expected `)`")]
    UnfinishedParenError,

    #[error("unfinished brackets, expected `]`")]
    UnfinishedBracketError,

    #[error("unfinished block, expected `}}`")]
    UnfinishedBraceError,

    #[error("expected sigma or array expression")]
    ExpectedBracketExprError,

    #[error("expected pi expression or group expression")]
    ExpectedParenExprError,

    #[error("expected constructor pattern or group pattern")]
    ExpectedConstructorError,

    #[error("useless semicolon here, you can just ignore it")]
    UeselessSemiWarning,

    #[error("useless comma here, you can just ignore it")]
    UselessCommaWarning,

    #[error("trailling comma is required")]
    RequiredTraillingCommaLint,
}

impl ParseError {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl InternalError for ParseError {
    fn code(&self) -> u16 {
        self.discriminant() as u16
    }

    fn kind(&self) -> asena_report::DiagnosticKind {
        asena_report::DiagnosticKind::Error
    }
}
