use asena_hir_derive::*;

use crate::{
    hir_type::HirTypeId, literal::HirLiteral, pattern::HirPatternId, stmt::HirStmtId,
    value::HirValueId, *,
};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprLiteral(pub HirLiteral);

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprGroup {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprCall {
    pub callee: data::HirCallee,
    pub arguments: Vec<HirValueId>,
    pub as_dsl: Option<data::HirDsl>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprReference {
    pub name: NameId,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprMatch {
    pub scrutinee: HirValueId,
    pub cases: Vec<data::HirMatchCase>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprHelp {
    pub value: HirValueId,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprAnn {
    pub value: HirValueId,
    pub against: HirTypeId,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprLam {
    pub parameters: Vec<NameId>,
    pub value: HirExprId,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
#[hir_debug]
pub struct HirExprArray {
    pub items: Vec<HirValueId>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirExpr)]
pub enum HirExprKind {
    #[default]
    Error,
    Unit,
    This,
    HirExprGroup(HirExprGroup),
    HirExprLiteral(HirExprLiteral),
    HirExprReference(HirExprReference),
    HirExprCall(HirExprCall),
    HirExprMatch(HirExprMatch),
    HirExprHelp(HirExprHelp),
    HirExprAnn(HirExprAnn),
    HirExprLam(HirExprLam),
    HirExprArray(HirExprArray),
}

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirExpr {
    pub kind: HirExprKind,
}

/// Data structures module split into its own module to better disposition, as
/// it is a bit large, and it's used as extension to [`Expr`].
pub mod data {
    use super::*;

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub enum HirMatchKind {
        If,
        Match,
        Switch,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub enum HirMatchArm {
        Expr(HirValueId),
        Block(HirStmtId),
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub struct HirMatchCase {
        pub pattern: HirPatternId,
        pub value: HirMatchArm,
        pub kind: HirMatchKind,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub struct HirDsl {
        pub parameters: Vec<NameId>,
        pub stmts: Vec<HirStmtId>,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub enum HirCallee {
        Value(HirValueId),

        // any operations
        Add,
        Sub,
        Mul,
        Div,

        // int only operations
        IAdd,
        ISub,
        IMul,
        IDiv,
    }
}
