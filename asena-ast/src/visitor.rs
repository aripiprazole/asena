#![allow(unused_variables)]

use std::default::default;

use asena_leaf::ast::Lexeme;

use crate::*;

pub fn new_walker<T: AsenaVisitor<()>>(concrete: &mut T) -> &mut dyn AsenaVisitor<()> {
    concrete
}

pub trait AsenaVisitor<T: Default> {
    fn visit_asena_file(&mut self, value: AsenaFile) -> T {
        default()
    }

    fn visit_qualified_path(&mut self, value: QualifiedPath) -> T {
        default()
    }

    fn visit_parameter(&mut self, value: Parameter) -> T {
        default()
    }

    fn visit_type_variant(&mut self, value: TypeVariant) -> T {
        default()
    }

    fn visit_constructor_variant(&mut self, value: ConstructorVariant) -> T {
        default()
    }

    fn visit_constraint(&mut self, value: Constraint) -> T {
        default()
    }

    fn visit_default_method(&mut self, value: DefaultMethod) -> T {
        default()
    }

    fn visit_field(&mut self, value: Field) -> T {
        default()
    }

    fn visit_method(&mut self, value: Method) -> T {
        default()
    }

    fn visit_where(&mut self, value: Where) -> T {
        default()
    }

    fn visit_decl(&mut self, value: Decl) -> T {
        default()
    }

    fn visit_branch(&mut self, value: Branch) -> T {
        default()
    }

    fn visit_lam_parameter(&mut self, value: LamParameter) -> T {
        default()
    }

    fn visit_case(&mut self, value: Case) -> T {
        default()
    }

    fn visit_accessor_segment(&mut self, value: AccessorSegment) -> T {
        default()
    }

    fn visit_expr_branch(&mut self, value: ExprBranch) -> T {
        self.visit_branch(value.into())
    }

    fn visit_block_branch(&mut self, value: BlockBranch) -> T {
        self.visit_branch(value.into())
    }

    fn visit_use(&mut self, value: Use) -> T {
        self.visit_decl(value.into())
    }

    fn visit_trait(&mut self, value: Trait) -> T {
        self.visit_decl(value.into())
    }

    fn visit_enum(&mut self, value: Enum) -> T {
        self.visit_decl(value.into())
    }

    fn visit_instance(&mut self, value: Instance) -> T {
        self.visit_decl(value.into())
    }

    fn visit_signature(&mut self, value: Signature) -> T {
        self.visit_decl(value.into())
    }

    fn visit_assign(&mut self, value: Assign) -> T {
        self.visit_decl(value.into())
    }

    fn visit_class(&mut self, value: Class) -> T {
        self.visit_decl(value.into())
    }

    fn visit_command(&mut self, value: Command) -> T {
        self.visit_decl(value.into())
    }

    fn visit_stmt(&mut self, value: Stmt) -> T {
        default()
    }

    fn visit_ask(&mut self, value: Ask) -> T {
        self.visit_stmt(value.into())
    }

    fn visit_if_stmt(&mut self, value: IfStmt) -> T {
        self.visit_stmt(value.into())
    }

    fn visit_let_stmt(&mut self, value: LetStmt) -> T {
        self.visit_stmt(value.into())
    }

    fn visit_expr_stmt(&mut self, value: ExprStmt) -> T {
        self.visit_stmt(value.into())
    }

    fn visit_return(&mut self, value: Return) -> T {
        self.visit_stmt(value.into())
    }

    fn visit_expr(&mut self, value: Expr) -> T {
        default()
    }

    fn visit_unit(&mut self, value: Unit) -> T {
        self.visit_expr(value.into())
    }

    fn visit_group(&mut self, value: Group) -> T {
        self.visit_expr(value.into())
    }

    fn visit_infix(&mut self, value: Infix) -> T {
        self.visit_expr(value.into())
    }

    fn visit_accessor(&mut self, value: Accessor) -> T {
        self.visit_expr(value.into())
    }

    fn visit_app(&mut self, value: App) -> T {
        self.visit_expr(value.into())
    }

    fn visit_if(&mut self, value: If) -> T {
        self.visit_expr(value.into())
    }

    fn visit_match(&mut self, value: Match) -> T {
        self.visit_expr(value.into())
    }

    fn visit_dsl(&mut self, value: Dsl) -> T {
        self.visit_expr(value.into())
    }

    fn visit_array(&mut self, value: Array) -> T {
        self.visit_expr(value.into())
    }

    fn visit_lam(&mut self, value: Lam) -> T {
        self.visit_expr(value.into())
    }

    fn visit_let(&mut self, value: Let) -> T {
        self.visit_expr(value.into())
    }

    fn visit_ann(&mut self, value: Ann) -> T {
        self.visit_expr(value.into())
    }

    fn visit_qual(&mut self, value: Qual) -> T {
        self.visit_expr(value.into())
    }

    fn visit_pi(&mut self, value: Pi) -> T {
        self.visit_expr(value.into())
    }

    fn visit_sigma(&mut self, value: Sigma) -> T {
        self.visit_expr(value.into())
    }

    fn visit_help(&mut self, value: Help) -> T {
        self.visit_expr(value.into())
    }

    fn visit_local_expr(&mut self, value: LocalExpr) -> T {
        self.visit_expr(value.into())
    }

    fn visit_literal_expr(&mut self, value: LiteralExpr) -> T {
        self.visit_expr(value.into())
    }

    fn visit_body(&mut self, value: Body) -> T {
        default()
    }

    fn visit_do(&mut self, value: Do) -> T {
        self.visit_body(value.into())
    }

    fn visit_value(&mut self, value: Value) -> T {
        self.visit_body(value.into())
    }

    fn visit_pat(&mut self, value: Pat) -> T {
        default()
    }

    fn visit_literal_pat(&mut self, value: LiteralPat) -> T {
        self.visit_pat(value.into())
    }

    fn visit_unit_pat(&mut self, value: UnitPat) -> T {
        self.visit_pat(value.into())
    }

    fn visit_global_pat(&mut self, value: GlobalPat) -> T {
        self.visit_pat(value.into())
    }

    fn visit_constructor_pat(&mut self, value: ConstructorPat) -> T {
        self.visit_pat(value.into())
    }

    fn visit_list_pat(&mut self, value: ListPat) -> T {
        self.visit_pat(value.into())
    }

    fn visit_wildcard_pat(&mut self, value: WildcardPat) -> T {
        self.visit_pat(value.into())
    }

    fn visit_spread_pat(&mut self, value: SpreadPat) -> T {
        self.visit_pat(value.into())
    }

    fn visit_local(&mut self, value: Lexeme<Local>) -> T {
        default()
    }

    fn visit_function_id(&mut self, value: Lexeme<FunctionId>) -> T {
        default()
    }

    fn visit_literal(&mut self, value: Lexeme<Literal>) -> T {
        default()
    }
}
