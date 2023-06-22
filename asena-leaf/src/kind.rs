use std::fmt::Display;

use super::named::Named;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TreeKind {
    #[default]
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
    QualifiedPathTree,

    ExprGroup,
    ExprBinary,
    ExprAccessor,
    ExprApp,
    ExprDsl,
    ExprArray,
    ExprLam,
    ExprLet,
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
    PatGlobal,
    PatConstructor,
    PatList,

    StmtAsk,
    StmtLet,
    StmtReturn,
    StmtExpr,

    LetBinding,

    BodyValue,
    BodyDo,

    DeclUse,
    DeclSignature,
    DeclAssign,
    DeclCommand,
    DeclClass,
    DeclInstance,

    Param,
    LamParam,

    TypeConstraint,

    PropertyField,
    PropertyMethod,

    TypeInfer,
    TypeExplicit,
}

impl Named for TreeKind {}

impl Display for TreeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
