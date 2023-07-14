use asena_hir_derive::*;

use crate::{hir_type::HirType, literal::HirLiteral, pattern::HirPattern, value::HirValue, *};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprLiteral(pub HirLiteral);

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprGroup {
    pub value: HirValue,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprCall {
    pub callee: data::HirCallee,
    pub arguments: Vec<HirValue>,
    pub as_dsl: Option<data::HirDsl>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprReference {
    pub name: Name,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprMatch {
    pub scrutinee: HirValue,
    pub cases: im::HashSet<data::HirMatchCase>,
    pub kind: data::HirMatchKind,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprHelp {
    pub value: HirValue,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprAnn {
    pub value: HirValue,
    pub against: HirType,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprLam {
    pub parameters: Vec<Name>,
    pub value: HirValue,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirExpr)]
pub struct HirExprArray {
    pub items: Vec<HirValue>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirExpr)]
pub enum HirExprKind {
    #[default]
    Error,
    Unit,
    This,
    Group(HirExprGroup),
    Literal(HirExprLiteral),
    Reference(HirExprReference),
    Call(HirExprCall),
    Match(HirExprMatch),
    Help(HirExprHelp),
    Ann(HirExprAnn),
    Lam(HirExprLam),
    Array(HirExprArray),
}

#[hir_struct]
pub struct HirExpr {
    pub kind: HirExprKind,
}

/// Data structures module split into its own module to better disposition, as
/// it is a bit large, and it's used as extension to [`Expr`].
pub mod data {
    use super::*;

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirMatchKind {
        If,
        Match,
        Switch,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirBranch {
        Error,
        Expr(HirValue),
        Block(HirValue),
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub struct HirMatchCase {
        pub pattern: HirPattern,
        pub value: HirBranch,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub struct HirDsl {
        pub parameters: Vec<Name>,
        pub value: HirValue,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirCallee {
        Value(HirValue),

        Do,

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
