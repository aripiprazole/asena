use std::fmt::Display;

use super::named::Named;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TreeKind {
    Error,

    File,

    LitNat,
    LitInt8,
    LitUInt8,
    LitInt16,
    LitUInt16,
    LitInt32,
    LitUInt32,
    LitInt64,
    LitUInt64,
    LitInt128,
    LitUInt128,

    LitFloat32,
    LitFloat64,

    LitTrue,
    LitFalse,

    LitString,

    SymbolIdentifier,
    QualifiedPath,

    ExprGroup,
    ExprBinary,
    ExprAccessor,
    ExprApp,
    ExprDsl,
    ExprArray,
    ExprLam,
    ExprLet,
    ExprGlobal,
    ExprLocal,
    ExprLit,
    ExprAnn,
    ExprQual,
    ExprPi,
    ExprSigma,
    ExprHelp,

    PatWildcard,
    PatSpread,
    PatLit,
    PatLocal,
    PatConstructor,
    PatList,

    StmtAsk,
    StmtLet,
    StmtReturn,
    StmtExpr,

    Binding,

    BodyValue,
    BodyDo,

    Parameter,

    DeclUse,
    DeclSignature,
    DeclAssign,
    DeclCommand,
    DeclClass,
    DeclInstance,

    Param,
    LamParam,

    Constraint,

    Field,
    Method,

    TypeInfer,
    Type,
}

impl Named for TreeKind {}

impl Display for TreeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
