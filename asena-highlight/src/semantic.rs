use asena_ast::*;
use asena_leaf::{
    ast::{Lexeme, Walkable},
    token::kind::TokenKind::*,
};

use crate::{HighlightColor::*, SemanticHighlight};

struct ParserSemanticHighlight<'a> {
    buf: &'a mut crate::Annotator,
}

// impl VariantWalker for ParserSemanticHighlight<'_> {}

// impl BodyWalker for ParserSemanticHighlight<'_> {}

// impl StmtWalker for ParserSemanticHighlight<'_> {
//     fn walk_stmt_let_stmt(&mut self, value: &LetStmt) {
//         self.buf.annotate(&value.token(LetKeyword), HardKeyword);
//     }

//     fn walk_stmt_if_stmt(&mut self, value: &IfStmt) {
//         self.buf.annotate(&value.token(IfKeyword), HardKeyword);
//     }

//     fn walk_stmt_return(&mut self, value: &Return) {
//         self.buf.annotate(&value.token(ReturnKeyword), HardKeyword);
//     }
// }

// impl BranchWalker for ParserSemanticHighlight<'_> {
//     fn walk_branch_expr_branch(&mut self, value: &ExprBranch) {
//         self.buf.annotate(&value.token(ThenKeyword), HardKeyword);
//         self.buf.annotate(&value.token(ElseKeyword), HardKeyword);
//     }

//     fn walk_branch_block_branch(&mut self, value: &BlockBranch) {
//         self.buf.annotate(&value.token(ThenKeyword), HardKeyword);
//         self.buf.annotate(&value.token(ElseKeyword), HardKeyword);
//     }
// }

// impl FileWalker for ParserSemanticHighlight<'_> {}

// impl PatWalker for ParserSemanticHighlight<'_> {
//     fn walk_pat_literal(&mut self, value: &Lexeme<Literal>) {
//         hightlight_literal(self.buf, value);
//     }

//     fn walk_pat_global(&mut self, value: &GlobalPat) {
//         let name = value.name();
//         if name.segments().len() == 1 {
//             let segments = name.segments();
//             let name = segments.first().unwrap();
//             highlight_local(self.buf, name.value.clone(), name)
//         }
//     }
// }

// impl WhereWalker for ParserSemanticHighlight<'_> {
//     fn walk_where(&mut self, value: &Where) {
//         self.buf.annotate(&value.token(WhereKeyword), HardKeyword);
//     }
// }

// impl DeclWalker for ParserSemanticHighlight<'_> {
//     fn walk_decl_signature(&mut self, value: &Signature) {
//         self.buf.annotate(&value.name(), GlobalFunction);
//         hightlight_parameters(self.buf, &value.parameters());
//     }

//     fn walk_decl_assign(&mut self, value: &asena_ast::Assign) {
//         self.buf.annotate(&value.name(), GlobalVariable);
//     }

//     fn walk_decl_use(&mut self, value: &Use) {
//         self.buf.annotate(&value.token(UseKeyword), HardKeyword);
//     }

//     fn walk_decl_trait(&mut self, value: &Trait) {
//         self.buf.annotate(&value.token(TraitKeyword), HardKeyword);
//         hightlight_parameters(self.buf, &value.parameters());
//         highlight_default_methods(self.buf, &value.default_methods())
//     }

//     fn walk_decl_class(&mut self, value: &Class) {
//         self.buf.annotate(&value.token(ClassKeyword), HardKeyword);
//         hightlight_parameters(self.buf, &value.parameters());
//         highlight_methods(self.buf, &value.methods())
//     }

//     fn walk_decl_enum(&mut self, value: &Enum) {
//         self.buf.annotate(&value.token(EnumKeyword), HardKeyword);
//         hightlight_parameters(self.buf, &value.parameters());
//         highlight_methods(self.buf, &value.methods())
//     }

//     fn walk_decl_instance(&mut self, value: &Instance) {
//         self.buf
//             .annotate(&value.token(InstanceKeyword), HardKeyword);
//         hightlight_parameters(self.buf, &value.parameters());
//         highlight_methods(self.buf, &value.methods())
//     }
// }

// impl ExprWalker for ParserSemanticHighlight<'_> {
//     fn walk_expr_group(&mut self, group: &Group) {
//         self.buf.annotate(&group.token(LeftParen), Delimitator);
//         self.buf.annotate(&group.token(RightParen), Delimitator);
//     }

//     fn walk_expr_array(&mut self, array: &Array) {
//         self.buf.annotate(&array.token(LeftBracket), Delimitator);
//         self.buf.annotate(&array.token(RightBracket), Delimitator);
//     }

//     fn walk_expr_if(&mut self, value: &If) {
//         self.buf.annotate(&value.token(IfKeyword), HardKeyword);
//     }

//     fn walk_expr_match(&mut self, value: &Match) {
//         self.buf.annotate(&value.token(MatchKeyword), HardKeyword);
//         self.buf.annotate(&value.token(LeftBrace), Delimitator);
//         self.buf.annotate(&value.token(RightBrace), Delimitator);
//     }

//     fn walk_expr_literal(&mut self, value: &Lexeme<Literal>) {
//         hightlight_literal(self.buf, value)
//     }

//     fn walk_expr_pi(&mut self, value: &Pi) {
//         if let Some(name) = value.parameter_name() {
//             highlight_local(self.buf, name.to_fn_id(), &name)
//         }
//     }

//     fn walk_expr_local(&mut self, value: &Lexeme<Local>) {
//         match value.as_str() {
//             "Self" => self.buf.annotate(value, HardKeyword),
//             "Bool" => self.buf.annotate(value, BuiltinType),
//             "Set" => self.buf.annotate(value, BuiltinType),
//             "Maybe" => self.buf.annotate(value, BuiltinType),
//             "Nat" => self.buf.annotate(value, BuiltinType),
//             _ => match value.as_str().chars().next() {
//                 Some(c) if c.is_lowercase() => {
//                     self.buf.annotate(value, LocalReference);
//                 }
//                 Some(_) => {}
//                 None => {}
//             },
//         }
//     }

//     fn walk_expr_app(&mut self, value: &App) {
//         match value.callee() {
//             Expr::Local(name) if name.is_ident("println") => {
//                 self.buf.annotate(&name, BuiltinFunction);
//             }
//             Expr::Local(name) if name.is_ident("print") => {
//                 self.buf.annotate(&name, BuiltinFunction);
//             }
//             Expr::Local(name) if name.is_ident("todo") => {
//                 self.buf.annotate(&name, BuiltinFunction);
//             }
//             _ => {}
//         }
//     }
// }

impl SemanticHighlight for Expr {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        todo!()
        // self.walk(&mut ParserSemanticHighlight { buf: annotator })
    }
}

impl SemanticHighlight for Typed {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        todo!()
        // self.walk(&mut ParserSemanticHighlight { buf: annotator })
    }
}

impl SemanticHighlight for Decl {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        todo!()
        // self.walk(&mut ParserSemanticHighlight { buf: annotator })
    }
}

impl SemanticHighlight for AsenaFile {
    fn annotate(&self, annotator: &mut crate::Annotator) {
        todo!()
        // self.walk(&mut ParserSemanticHighlight { buf: annotator })
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
    match name.as_str() {
        "self" => buf.annotate(&name, HardKeyword),
        _ => highlight_local(buf, name.to_fn_id(), &name),
    }
}

fn hightlight_literal(buf: &mut crate::Annotator, literal: &Lexeme<Literal>) {
    match literal.value {
        Literal::Nat(_) => buf.annotate(literal, Number),
        Literal::String(_) => buf.annotate(literal, String),
        Literal::Int8(_, _) => buf.annotate(literal, Number),
        Literal::Int16(_, _) => buf.annotate(literal, Number),
        Literal::Int32(_, _) => buf.annotate(literal, Number),
        Literal::Int64(_, _) => buf.annotate(literal, Number),
        Literal::Int128(_, _) => buf.annotate(literal, Number),
        Literal::Float32(_) => buf.annotate(literal, Number),
        Literal::Float64(_) => buf.annotate(literal, Number),
        Literal::True => buf.annotate(literal, HardKeyword),
        Literal::False => buf.annotate(literal, HardKeyword),
        Literal::Error => buf.annotate(literal, crate::HighlightColor::Error),
    }
}
