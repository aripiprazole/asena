use std::fmt::Display;

use super::named::Named;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TreeKind {
    #[default]
    Error,

    File,

    ListTree,

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

    ExprUnit,
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
    ExprIf,
    ExprMatch,

    AccessorArg,
    AccessorExpr,

    BranchExpr,
    BranchBlock,

    IfThen,
    IfElse,

    MatchCase,

    IdSymbol,

    PatWildcard,
    PatSpread,
    PatLit,
    PatGlobal,
    PatConstructor,
    PatConstructorList,
    PatList,
    PatGroup,
    PatUnit,

    StmtAsk,
    StmtLet,
    StmtReturn,
    StmtExpr,
    StmtIf,

    LetBinding,

    BodyValue,
    BodyDo,

    DeclUse,
    DeclSignature,
    DeclAssign,
    DeclCommand,
    DeclClass,
    DeclTrait,
    DeclEnum,
    DeclInstance,

    EnumGadtType,

    WhereClause,

    ClassField,
    ClassMethod,

    InstanceImpl,

    TraitDefault,

    VariantConstructor,
    VariantType,

    Param,
    SelfParam,
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
