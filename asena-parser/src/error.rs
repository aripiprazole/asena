use thiserror::Error;

use asena_leaf::node::kind::TokenKind;
use asena_report::InternalError;
use asena_span::Spanned;

pub type Result<T, E = Spanned<ParseError>> = std::result::Result<T, E>;

/// Parsing errors, it can be indexed to the code using a [Spanned<ParseError>].
#[derive(Error, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ParseError {
    #[error("The stack should contain the last close element")]
    EmptyStackError,

    #[error("The stack should contain just the tree element but has {} elements", .0)]
    StackError(usize),

    #[error("The token stream still contain something: `{}`", .0)]
    StreamStillContainElements(TokenKind),

    #[error("Unexpected token")]
    UnexpectedTokenError,

    #[error("Missing semicolon")]
    MissingSemiError,

    #[error("Expression should be surrounded by parenthesis: `({} ..)`", .0)]
    PrimarySurroundedError(TokenKind),

    #[error("Found an `else` without previous `if` node")]
    DanglingElseError,

    #[error("Found unicode `{}`, you can rewrite in the language norm as `{}`", .0, .1)]
    UnicodeError(TokenKind, &'static str),

    #[error("Reserved keyword `{}` in the wrong position, must be a constraint", .0)]
    ConstraintReservedKeywordError(TokenKind),

    #[error("Reserved keyword `{}` in the wrong position, must be a statement", .0)]
    StmtReservedKeywordError(TokenKind),

    #[error("Reserved keyword `{}` in the wrong position, must be a top-level declaration", .0)]
    DeclReservedKeywordError(TokenKind),

    #[error("Reserved keyword `{}` to be used in a feature, you can use like: `{}_`", .0, .0)]
    ReservedKeywordError(TokenKind),

    #[error("Invalid identifier, found symbol")]
    InvalidSymbolIdentifierError,

    #[error("Invalid identifier, found unicode symbol")]
    InvalidUnicodeIdentifierError,

    #[error("Expected token: `{}`. But got this instead", .0.to_string())]
    ExpectedTokenError(TokenKind),

    #[error("Expected expression and close list")]
    ExpectedExprAndCloseListError,

    #[error("Expected expression")]
    ExpectedExprError,

    #[error("Expected match scrutinee")]
    ExpectedMatchScrutineeError,

    #[error("Expected case expr")]
    ExpectedCaseExprError,

    #[error("Expected case")]
    ExpectedCaseError,

    #[error("Expected if condition")]
    ExpectedIfCondError,

    #[error("Expected if else expression")]
    ExpectedIfElseExprError,

    #[error("Expected if else")]
    ExpectedIfElseError,

    #[error("Expected if then expression")]
    ExpectedIfThenExprError,

    #[error("Expected if then or open braces")]
    ExpectedIfThenError,

    #[error("Expected field")]
    ExpectedFieldError,

    #[error("Expected impl function")]
    ExpectedImplError,

    #[error("Expected rhs of infix")]
    ExpectedInfixRhsError,

    #[error("Expected pattern")]
    ExpectedPatError,

    #[error("Could not parse primary")]
    PrimaryExpectedError,

    #[error("Expected sigma parameter type")]
    ExpectedSigmaParamError,

    #[error("Expected sigma return type")]
    ExpectedSigmaReturnError,

    #[error("Expected pi parameter type")]
    ExpectedPiParamError,

    #[error("Expected pi return type")]
    ExpectedPiReturnError,

    #[error("Expected qual return type")]
    ExpectedQualReturnError,

    #[error("Expected ann against")]
    ExpectedAnnAgainstError,

    #[error("Expected help value")]
    ExpectedHelpValueError,

    #[error("Expected lam body")]
    ExpectedLamBodyError,

    #[error("Expected impl value")]
    ExpectedImplValueError,

    #[error("Expected assign value")]
    ExpectedAssignValueError,

    #[error("Expected ask name")]
    ExpectedAskNameError,

    #[error("Expected parameter type")]
    ExpectedParameterTypeError,

    #[error("Expected ask value")]
    ExpectedAskValueError,

    #[error("Expected type")]
    ExpectedTypeError,

    #[error("Expected patterns")]
    ExpectedPatternsError,

    #[error("Expected let name")]
    ExpectedLetNameError,

    #[error("Expected let value")]
    ExpectedLetValueError,

    #[error("Expected return value")]
    ExpectedReturnValueError,

    #[error("Expected return stmt")]
    ExpectedReturnStmtError,

    #[error("Could not parse anything, found end of file")]
    EofError,

    #[error("Only one where clause is permitted")]
    OnlyOneWhereClauseIsPermittedError,

    #[error("Expected parameter")]
    ExpectedParameterError,

    #[error("Expected implementations, not methods")]
    MethodNotAllowedInInstanceError,

    #[error("Expected parameter to be a tuple")]
    ParameterIsCurryiedAndNotTupleError,

    #[error("Expected variant")]
    ExpectedVariantError,

    #[error("Expected variant parameter")]
    ExpectedVariantParameterError,

    #[error("Expected statement")]
    ExpectedStmtError,

    #[error("Unfinished parenthesis, expected `)`")]
    UnfinishedParenError,

    #[error("Unfinished brackets, expected `]`")]
    UnfinishedBracketError,

    #[error("Unfinished block, expected `}}`")]
    UnfinishedBraceError,

    #[error("Expected <SIGMA_EXPR> or <ARRAY_EXPR>")]
    ExpectedBracketExprError,

    #[error("Expected <PI_EXPR> or <GROUP_EXPR>")]
    ExpectedParenExprError,

    #[error("Expected <CONSTRUCTOR_PAT> or <GROUP_PAT>")]
    ExpectedConstructorError,

    #[error("Useless semicolon here, you can just ignore it")]
    UeselessSemiError,

    #[error("Useless comma here, you can just ignore it")]
    UselessCommaError,

    #[error("Trailling comma is required")]
    RequiredTraillingCommaError,
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
