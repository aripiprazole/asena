use asena_leaf::ast::Leaf;
use asena_leaf::node::{Tree, TreeKind};
use asena_span::Spanned;
use TreeKind::*;

use crate::*;

impl Leaf for crate::Type {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        Some(match tree.kind {
            Type => Self::Explicit(tree.at::<Expr>(0).try_as_leaf()?),
            _ => Self::Infer,
        })
    }
}

impl Leaf for crate::Expr {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        Some(match tree.kind {
            ExprGroup => Self::Group(Group::new(tree)),
            ExprBinary => Self::Infix(Infix::new(tree)),
            ExprAccessor => Self::Accessor(Accessor::new(tree)),
            ExprApp => Self::App(App::new(tree)),
            ExprArray => Self::Array(Array::new(tree)),
            ExprDsl => Self::Dsl(Dsl::new(tree)),
            ExprLam => Self::Lam(Lam::new(tree)),
            ExprLet => Self::Let(Let::new(tree)),
            ExprAnn => Self::Ann(Ann::new(tree)),
            ExprQual => Self::Qual(Qual::new(tree)),
            ExprPi => Self::Pi(Pi::new(tree)),
            ExprSigma => Self::Sigma(Sigma::new(tree)),
            ExprHelp => Self::Help(Help::new(tree)),
            ExprLocal => Self::Local(tree.terminal::<Local>(0).try_as_leaf()?),
            ExprLit => Self::Literal(tree.filter_terminal::<Literal>().first().try_as_leaf()?),
            TreeQualifiedPath => Self::QualifiedPath(QualifiedPath::new(tree)),
            _ => return None,
        })
    }
}

impl Leaf for crate::Decl {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        Some(match tree.kind {
            DeclUse => Self::Use(Use::new(tree)),
            DeclSignature => Self::Signature(Signature::new(tree)),
            DeclAssign => Self::Assign(Assign::new(tree)),
            DeclCommand => Self::Command(Command::new(tree)),
            DeclClass => Self::Class(Class::new(tree)),
            DeclInstance => Self::Instance(Instance::new(tree)),
            _ => return None,
        })
    }
}

impl Leaf for crate::Stmt {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        Some(match tree.kind {
            StmtExpr => Self::Eval(Eval::new(tree)),
            StmtLet => Self::Set(Set::new(tree)),
            StmtAsk => Self::Ask(Ask::new(tree)),
            StmtReturn => Self::Return(Return::new(tree)),
            _ => return None,
        })
    }
}

impl Leaf for crate::Pat {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        Some(match tree.kind {
            PatList => Self::List(List::new(tree)),
            PatWildcard => Self::Wildcard(Wildcard::new(tree)),
            PatSpread => Self::Spread(Spread::new(tree)),
            PatConstructor => Self::Constructor(Constructor::new(tree)),
            PatGlobal => Self::QualifiedPath(tree.at::<QualifiedPath>(0).try_as_leaf()?),
            PatLit => Self::Literal(tree.filter_terminal::<Literal>().first().try_as_leaf()?),
            _ => return None,
        })
    }
}

impl Leaf for crate::QualifiedPath {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        Some(match tree.kind {
            TreeQualifiedPath => QualifiedPath::new(tree),
            _ => return None,
        })
    }
}

impl Leaf for crate::Parameter {
    fn make(tree: Spanned<Tree>) -> Option<Self> {
        Some(match tree.kind {
            TreeQualifiedPath => Parameter::new(tree),
            _ => return None,
        })
    }
}

impl Leaf for crate::Binding {
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}

impl Leaf for crate::Property {
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}

impl Leaf for crate::Constraint {
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}

impl Leaf for crate::Body {
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}

impl Leaf for crate::Method {
    fn make(_tree: Spanned<Tree>) -> Option<Self> {
        todo!()
    }
}
