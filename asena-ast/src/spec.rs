use asena_leaf::node::{Tree, TreeKind};
use asena_leaf::spec::{Node, Spec};
use asena_span::Spanned;

use crate::*;

impl Spec for Type {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use TreeKind::*;

        match from.kind {
            Type => {
                let expr = from.at::<Expr>(0)?;

                Node::new(from.swap(Self::Explicit(expr.value)))
            }
            _ => Node::empty(),
        }
    }
}

impl Spec for Expr {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use TreeKind::*;

        let value = match from.kind {
            TreeQualifiedPath => Expr::QualifiedPath(QualifiedPath::new(from.clone().into())),
            ExprGroup => Expr::Group(Group::new(from.clone().into())),
            ExprBinary => Expr::Infix(Infix::new(from.clone().into())),
            ExprAccessor => Expr::Accessor(Accessor::new(from.clone().into())),
            ExprApp => Expr::App(App::new(from.clone().into())),
            ExprArray => Expr::Array(Array::new(from.clone().into())),
            ExprDsl => Expr::Dsl(Dsl::new(from.clone().into())),
            ExprLam => Expr::Lam(Lam::new(from.clone().into())),
            ExprLet => Expr::Let(Let::new(from.clone().into())),
            ExprAnn => Expr::Ann(Ann::new(from.clone().into())),
            ExprQual => Expr::Qual(Qual::new(from.clone().into())),
            ExprPi => Expr::Pi(Pi::new(from.clone().into())),
            ExprSigma => Expr::Sigma(Sigma::new(from.clone().into())),
            ExprHelp => Expr::Help(Help::new(from.clone().into())),
            ExprLocal => return from.terminal::<Local>(0)?.map(Expr::Local).into(),
            ExprLit => match from.filter_terminal::<Literal>().first().cloned() {
                Some(x) => return Node::new(from.swap(Expr::Literal(x.value.clone()))),
                None => return Node::empty(),
            },
            _ => return Node::empty(),
        };

        from.replace(value).into()
    }
}

impl Spec for Decl {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use TreeKind::*;

        let value = match from.kind {
            DeclUse => Decl::Use(Use::new(from.clone().into())),
            DeclSignature => Decl::Signature(Signature::new(from.clone().into())),
            DeclAssign => Decl::Assign(Assign::new(from.clone().into())),
            DeclCommand => Decl::Command(Command::new(from.clone().into())),
            DeclClass => Decl::Class(Class::new(from.clone().into())),
            DeclInstance => Decl::Instance(Instance::new(from.clone().into())),
            _ => return Node::empty(),
        };

        from.replace(value).into()
    }
}

impl Spec for Stmt {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use TreeKind::*;

        let value = match from.kind {
            StmtExpr => Stmt::Eval(Eval::new(from.clone().into())),
            StmtLet => Stmt::Set(Set::new(from.clone().into())),
            StmtAsk => Stmt::Ask(Ask::new(from.clone().into())),
            StmtReturn => Stmt::Return(Return::new(from.clone().into())),
            _ => return Node::empty(),
        };

        from.replace(value).into()
    }
}

impl Spec for Pat {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use TreeKind::*;

        let value = match from.kind {
            PatList => Pat::List(List::new(from.clone().into())),
            PatWildcard => Pat::Wildcard(Wildcard::new(from.clone().into())),
            PatSpread => Pat::Spread(Spread::new(from.clone().into())),
            PatConstructor => Pat::Constructor(Constructor::new(from.clone().into())),
            PatGlobal => return from.at::<QualifiedPath>(0)?.map(Pat::QualifiedPath).into(),
            PatLit => {
                return from
                    .filter_terminal::<Literal>()
                    .first()
                    .cloned()
                    .unwrap()
                    .map(|x| from.swap(Pat::Literal(x.value)));
            }
            _ => {
                return Node::empty();
            }
        };

        from.replace(value).into()
    }
}

impl Spec for QualifiedPath {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        if from.kind != TreeKind::TreeQualifiedPath {
            return Node::empty();
        }

        let tree = from.clone().into();

        Node::new(from.swap(QualifiedPath::new(tree)))
    }
}

impl Spec for Parameter {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        if from.kind != TreeKind::Param {
            return Node::empty();
        }

        let tree = from.clone();

        Node::new(from.swap(Parameter::new(tree.into())))
    }
}
