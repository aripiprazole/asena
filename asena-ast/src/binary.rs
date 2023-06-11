use std::ops::DerefMut;

use asena_leaf::green::{Green, GreenTree};
use asena_leaf::spec::Node;
use asena_span::Spanned;

use crate::*;

pub trait Binary: DerefMut + DerefMut<Target = GreenTree> + Clone {
    fn make_expr(tree: GreenTree) -> Expr;

    fn lhs(&self) -> Green<Node<Spanned<Expr>>> {
        self.lazy("lhs", |this| this.at(0))
    }

    fn fn_id(&self) -> Green<Node<Spanned<FunctionId>>> {
        self.lazy("fn_id", |this| this.terminal(1))
    }

    fn rhs(&self) -> Green<Node<Spanned<Expr>>> {
        self.lazy("rhs", |this| {
            let mut rhs = this.clone();

            // Checks the integrity of the length for safety
            match rhs.children.len() {
                0 => return Node::empty(),
                1 => return rhs.at(0),
                _ => {}
            }

            // Remove the first twice
            rhs.children.remove(0);
            rhs.children.remove(0);

            if rhs.is_single() {
                rhs.at(0)
            } else {
                Node::new(this.replace(Self::make_expr(rhs)))
            }
        })
    }
}

impl Binary for Infix {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Infix(Infix::new(tree))
    }
}

/// Binary operation represented by `fn_id`: `.`, and the two operands: `receiver`, `name`
impl Binary for Accessor {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Accessor(Accessor::new(tree))
    }
}

impl Binary for Ann {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Ann(Ann::new(tree))
    }
}

impl Binary for Qual {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Qual(Qual::new(tree))
    }
}
