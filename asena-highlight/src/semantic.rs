use asena_ast::*;
use asena_leaf::{
    ast::{Lexeme, Walkable},
    token::kind::TokenKind::*,
};

use crate::{HighlightColor::*, SemanticHighlight};

struct ParserSemanticHighlight<'a> {
    buf: &'a mut crate::Annotator,
}

impl AsenaVisitor<()> for ParserSemanticHighlight<'_> {
    fn visit_let_stmt(&mut self, value: LetStmt) {
        self.buf.annotate(&value.token(LetKeyword), HardKeyword);
    }

    fn visit_if_stmt(&mut self, value: IfStmt) {
        self.buf.annotate(&value.token(IfKeyword), HardKeyword);
    }

    fn visit_return(&mut self, value: Return) {
        self.buf.annotate(&value.token(ReturnKeyword), HardKeyword);
    }

    fn visit_expr_branch(&mut self, value: ExprBranch) {
        self.buf.annotate(&value.token(ThenKeyword), HardKeyword);
        self.buf.annotate(&value.token(ElseKeyword), HardKeyword);
    }

    fn visit_block_branch(&mut self, value: BlockBranch) {
        self.buf.annotate(&value.token(ThenKeyword), HardKeyword);
        self.buf.annotate(&value.token(ElseKeyword), HardKeyword);
    }

    fn visit_where(&mut self, value: Where) {
        self.buf.annotate(&value.token(WhereKeyword), HardKeyword);
    }

    fn visit_qualified_path(&mut self, name: QualifiedPath) {
        if name.segments().len() == 1 {
            let segments = name.segments();
            let name = segments.first().unwrap();
            highlight_local(self.buf, name.to_fn_id(), name)
        }
    }

    fn visit_signature(&mut self, value: Signature) {
        self.buf.annotate(&value.name(), GlobalFunction);
        hightlight_parameters(self.buf, &value.parameters());
    }

    fn visit_assign(&mut self, value: asena_ast::Assign) {
        self.buf.annotate(&value.name(), GlobalVariable);
    }

    fn visit_use(&mut self, value: Use) {
        self.buf.annotate(&value.token(UseKeyword), HardKeyword);
    }

    fn visit_trait(&mut self, value: Trait) {
        self.buf.annotate(&value.token(TraitKeyword), HardKeyword);
        hightlight_parameters(self.buf, &value.parameters());
        highlight_default_methods(self.buf, &value.default_methods())
    }

    fn visit_class(&mut self, value: Class) {
        self.buf.annotate(&value.token(ClassKeyword), HardKeyword);
        hightlight_parameters(self.buf, &value.parameters());
        highlight_methods(self.buf, &value.methods())
    }

    fn visit_enum(&mut self, value: Enum) {
        self.buf.annotate(&value.token(EnumKeyword), HardKeyword);
        hightlight_parameters(self.buf, &value.parameters());
        highlight_methods(self.buf, &value.methods())
    }

    fn visit_instance(&mut self, value: Instance) {
        self.buf
            .annotate(&value.token(InstanceKeyword), HardKeyword);
        hightlight_parameters(self.buf, &value.parameters());
        highlight_methods(self.buf, &value.methods())
    }

    fn visit_group(&mut self, group: Group) {
        self.buf.annotate(&group.token(LeftParen), Delimitator);
        self.buf.annotate(&group.token(RightParen), Delimitator);
    }

    fn visit_array(&mut self, array: Array) {
        self.buf.annotate(&array.token(LeftBracket), Delimitator);
        self.buf.annotate(&array.token(RightBracket), Delimitator);
    }

    fn visit_if(&mut self, value: If) {
        self.buf.annotate(&value.token(IfKeyword), HardKeyword);
    }

    fn visit_match(&mut self, value: Match) {
        self.buf.annotate(&value.token(MatchKeyword), HardKeyword);
        self.buf.annotate(&value.token(LeftBrace), Delimitator);
        self.buf.annotate(&value.token(RightBrace), Delimitator);
    }

    fn visit_app(&mut self, value: App) {
        match value.callee() {
            Expr::LocalExpr(local) if local.is_some_ident("println") => {
                self.buf.annotate(&local, BuiltinFunction);
            }
            Expr::LocalExpr(local) if local.is_some_ident("print") => {
                self.buf.annotate(&local, BuiltinFunction);
            }
            Expr::LocalExpr(local) if local.is_some_ident("todo") => {
                self.buf.annotate(&local, BuiltinFunction);
            }
            _ => {}
        }
    }

    fn visit_literal(&mut self, literal: Lexeme<Literal>) {
        self.buf.annotate(
            &literal,
            match literal.data() {
                Literal::Nat(_) => Number,
                Literal::String(_) => String,
                Literal::Int8(_, _) => Number,
                Literal::Int16(_, _) => Number,
                Literal::Int32(_, _) => Number,
                Literal::Int64(_, _) => Number,
                Literal::Int128(_, _) => Number,
                Literal::Float32(_) => Number,
                Literal::Float64(_) => Number,
                Literal::True => HardKeyword,
                Literal::False => HardKeyword,
                Literal::Error => crate::HighlightColor::Error,
            },
        )
    }
}

impl SemanticHighlight for Expr {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        self.walk(&mut new_walker(&mut ParserSemanticHighlight {
            buf: annotator,
        }))
    }
}

impl SemanticHighlight for AsenaFile {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        self.walk(&mut new_walker(&mut ParserSemanticHighlight {
            buf: annotator,
        }))
    }
}

fn highlight_default_methods(buf: &mut crate::Annotator, methods: &[DefaultMethod]) {
    for method in methods {
        buf.annotate(&method.token(DefaultKeyword), HardKeyword);
        hightlight_parameters(buf, &method.parameters())
    }
}

fn hightlight_parameters(buf: &mut crate::Annotator, parameters: &[Parameter]) {
    for parameter in parameters {
        highlight_parameter(buf, parameter);
    }
}

fn highlight_methods(buf: &mut crate::Annotator, methods: &[Method]) {
    for method in methods {
        buf.annotate(&method.token(FunKeyword), HardKeyword);
        hightlight_parameters(buf, &method.parameters())
    }
}

fn highlight_local<T>(buf: &mut crate::Annotator, local: FunctionId, lexeme: &Lexeme<T>) {
    match local.as_str().chars().next() {
        Some(c) if c.is_lowercase() => {
            buf.annotate(lexeme, LocalReference);
        }
        Some(_) => {}
        None => {}
    }
}

fn highlight_parameter(buf: &mut crate::Annotator, parameter: &Parameter) {
    let name = parameter.name();
    match name.to_fn_id().as_str() {
        "self" => buf.annotate(&name, HardKeyword),
        _ => {},
    }
}
