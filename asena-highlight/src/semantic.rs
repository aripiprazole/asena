use asena_derive::*;

use asena_ast::*;
use asena_leaf::{ast::Walkable, token::TokenKind::*};

use crate::{HighlightColor::*, SemanticHighlight};

#[ast_step(PatWalker, StmtWalker)]
struct SemanticHighlightTraversal<'a> {
    type_level: bool,
    buf: &'a mut crate::Annotator,
}

impl FileWalker for SemanticHighlightTraversal<'_> {}

impl PropertyWalker for SemanticHighlightTraversal<'_> {}

impl BodyWalker for SemanticHighlightTraversal<'_> {}

impl DeclWalker for SemanticHighlightTraversal<'_> {
    fn walk_decl_signature(&mut self, value: &Signature) {
        self.buf.annotate(&value.name(), GlobalFunction);
    }
}

impl ExprWalker for SemanticHighlightTraversal<'_> {
    fn walk_expr_group(&mut self, group: &Group) {
        self.buf.annotate(&group.token(LeftParen), Delimitator);
        self.buf.annotate(&group.token(RightParen), Delimitator);
    }

    fn walk_expr_array(&mut self, array: &Array) {
        self.buf.annotate(&array.token(LeftBracket), Delimitator);
        self.buf.annotate(&array.token(RightBracket), Delimitator);
    }

    fn walk_expr_app(&mut self, value: &App) {
        match value.callee() {
            Expr::Local(name) if name.is_ident("println") => {}
            Expr::Local(name) if name.is_ident("print") => {}
            Expr::Local(name) if name.is_ident("todo") => {}
            Expr::Local(name) if name.is_ident("Unit") && self.type_level => {}
            _ => {}
        }
    }
}

impl SemanticHighlight for Expr {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        self.walk(&mut SemanticHighlightTraversal {
            type_level: false,
            buf: annotator,
        })
    }
}

impl SemanticHighlight for Typed {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        self.walk(&mut SemanticHighlightTraversal {
            type_level: true,
            buf: annotator,
        })
    }
}

impl SemanticHighlight for Decl {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        self.walk(&mut SemanticHighlightTraversal {
            type_level: false,
            buf: annotator,
        })
    }
}

impl SemanticHighlight for AsenaFile {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        self.walk(&mut SemanticHighlightTraversal {
            type_level: false,
            buf: annotator,
        })
    }
}
