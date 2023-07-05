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

    fn visit_qualified_binding_id(&mut self, value: BindingId) -> T {
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

pub trait AsenaListener<T: Default = ()> {
    fn enter_asena_file(&mut self, value: AsenaFile) -> T {
        default()
    }

    fn enter_qualified_path(&mut self, value: QualifiedPath) -> T {
        default()
    }

    fn enter_qualified_binding_id(&mut self, value: BindingId) -> T {
        default()
    }

    fn enter_parameter(&mut self, value: Parameter) -> T {
        default()
    }

    fn enter_type_variant(&mut self, value: TypeVariant) -> T {
        default()
    }

    fn enter_constructor_variant(&mut self, value: ConstructorVariant) -> T {
        default()
    }

    fn enter_constraint(&mut self, value: Constraint) -> T {
        default()
    }

    fn enter_default_method(&mut self, value: DefaultMethod) -> T {
        default()
    }

    fn enter_field(&mut self, value: Field) -> T {
        default()
    }

    fn enter_method(&mut self, value: Method) -> T {
        default()
    }

    fn enter_where(&mut self, value: Where) -> T {
        default()
    }

    fn enter_decl(&mut self, value: Decl) -> T {
        default()
    }

    fn enter_branch(&mut self, value: Branch) -> T {
        default()
    }

    fn enter_lam_parameter(&mut self, value: LamParameter) -> T {
        default()
    }

    fn enter_case(&mut self, value: Case) -> T {
        default()
    }

    fn enter_expr_branch(&mut self, value: ExprBranch) -> T {
        self.enter_branch(value.into())
    }

    fn enter_block_branch(&mut self, value: BlockBranch) -> T {
        self.enter_branch(value.into())
    }

    fn enter_use(&mut self, value: Use) -> T {
        self.enter_decl(value.into())
    }

    fn enter_trait(&mut self, value: Trait) -> T {
        self.enter_decl(value.into())
    }

    fn enter_enum(&mut self, value: Enum) -> T {
        self.enter_decl(value.into())
    }

    fn enter_instance(&mut self, value: Instance) -> T {
        self.enter_decl(value.into())
    }

    fn enter_signature(&mut self, value: Signature) -> T {
        self.enter_decl(value.into())
    }

    fn enter_assign(&mut self, value: Assign) -> T {
        self.enter_decl(value.into())
    }

    fn enter_class(&mut self, value: Class) -> T {
        self.enter_decl(value.into())
    }

    fn enter_command(&mut self, value: Command) -> T {
        self.enter_decl(value.into())
    }

    fn enter_stmt(&mut self, value: Stmt) -> T {
        default()
    }

    fn enter_ask(&mut self, value: Ask) -> T {
        self.enter_stmt(value.into())
    }

    fn enter_if_stmt(&mut self, value: IfStmt) -> T {
        self.enter_stmt(value.into())
    }

    fn enter_let_stmt(&mut self, value: LetStmt) -> T {
        self.enter_stmt(value.into())
    }

    fn enter_expr_stmt(&mut self, value: ExprStmt) -> T {
        self.enter_stmt(value.into())
    }

    fn enter_return(&mut self, value: Return) -> T {
        self.enter_stmt(value.into())
    }

    fn enter_expr(&mut self, value: Expr) -> T {
        default()
    }

    fn enter_unit(&mut self, value: Unit) -> T {
        self.enter_expr(value.into())
    }

    fn enter_group(&mut self, value: Group) -> T {
        self.enter_expr(value.into())
    }

    fn enter_infix(&mut self, value: Infix) -> T {
        self.enter_expr(value.into())
    }

    fn enter_app(&mut self, value: App) -> T {
        self.enter_expr(value.into())
    }

    fn enter_if(&mut self, value: If) -> T {
        self.enter_expr(value.into())
    }

    fn enter_match(&mut self, value: Match) -> T {
        self.enter_expr(value.into())
    }

    fn enter_dsl(&mut self, value: Dsl) -> T {
        self.enter_expr(value.into())
    }

    fn enter_array(&mut self, value: Array) -> T {
        self.enter_expr(value.into())
    }

    fn enter_lam(&mut self, value: Lam) -> T {
        self.enter_expr(value.into())
    }

    fn enter_let(&mut self, value: Let) -> T {
        self.enter_expr(value.into())
    }

    fn enter_ann(&mut self, value: Ann) -> T {
        self.enter_expr(value.into())
    }

    fn enter_qual(&mut self, value: Qual) -> T {
        self.enter_expr(value.into())
    }

    fn enter_pi(&mut self, value: Pi) -> T {
        self.enter_expr(value.into())
    }

    fn enter_sigma(&mut self, value: Sigma) -> T {
        self.enter_expr(value.into())
    }

    fn enter_help(&mut self, value: Help) -> T {
        self.enter_expr(value.into())
    }

    fn enter_local_expr(&mut self, value: LocalExpr) -> T {
        self.enter_expr(value.into())
    }

    fn enter_literal_expr(&mut self, value: LiteralExpr) -> T {
        self.enter_expr(value.into())
    }

    fn enter_body(&mut self, value: Body) -> T {
        default()
    }

    fn enter_do(&mut self, value: Do) -> T {
        self.enter_body(value.into())
    }

    fn enter_value(&mut self, value: Value) -> T {
        self.enter_body(value.into())
    }

    fn enter_pat(&mut self, value: Pat) -> T {
        default()
    }

    fn enter_literal_pat(&mut self, value: LiteralPat) -> T {
        self.enter_pat(value.into())
    }

    fn enter_unit_pat(&mut self, value: UnitPat) -> T {
        self.enter_pat(value.into())
    }

    fn enter_global_pat(&mut self, value: GlobalPat) -> T {
        self.enter_pat(value.into())
    }

    fn enter_constructor_pat(&mut self, value: ConstructorPat) -> T {
        self.enter_pat(value.into())
    }

    fn enter_list_pat(&mut self, value: ListPat) -> T {
        self.enter_pat(value.into())
    }

    fn enter_wildcard_pat(&mut self, value: WildcardPat) -> T {
        self.enter_pat(value.into())
    }

    fn enter_spread_pat(&mut self, value: SpreadPat) -> T {
        self.enter_pat(value.into())
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

    fn exit_asena_file(&mut self, value: AsenaFile) -> T {
        default()
    }

    fn exit_qualified_path(&mut self, value: QualifiedPath) -> T {
        default()
    }

    fn exit_qualified_binding_id(&mut self, value: BindingId) -> T {
        default()
    }

    fn exit_parameter(&mut self, value: Parameter) -> T {
        default()
    }

    fn exit_type_variant(&mut self, value: TypeVariant) -> T {
        default()
    }

    fn exit_constructor_variant(&mut self, value: ConstructorVariant) -> T {
        default()
    }

    fn exit_constraint(&mut self, value: Constraint) -> T {
        default()
    }

    fn exit_default_method(&mut self, value: DefaultMethod) -> T {
        default()
    }

    fn exit_field(&mut self, value: Field) -> T {
        default()
    }

    fn exit_method(&mut self, value: Method) -> T {
        default()
    }

    fn exit_where(&mut self, value: Where) -> T {
        default()
    }

    fn exit_decl(&mut self, value: Decl) -> T {
        default()
    }

    fn exit_branch(&mut self, value: Branch) -> T {
        default()
    }

    fn exit_lam_parameter(&mut self, value: LamParameter) -> T {
        default()
    }

    fn exit_case(&mut self, value: Case) -> T {
        default()
    }

    fn exit_expr_branch(&mut self, value: ExprBranch) -> T {
        self.exit_branch(value.into())
    }

    fn exit_block_branch(&mut self, value: BlockBranch) -> T {
        self.exit_branch(value.into())
    }

    fn exit_use(&mut self, value: Use) -> T {
        self.exit_decl(value.into())
    }

    fn exit_trait(&mut self, value: Trait) -> T {
        self.exit_decl(value.into())
    }

    fn exit_enum(&mut self, value: Enum) -> T {
        self.exit_decl(value.into())
    }

    fn exit_instance(&mut self, value: Instance) -> T {
        self.exit_decl(value.into())
    }

    fn exit_signature(&mut self, value: Signature) -> T {
        self.exit_decl(value.into())
    }

    fn exit_assign(&mut self, value: Assign) -> T {
        self.exit_decl(value.into())
    }

    fn exit_class(&mut self, value: Class) -> T {
        self.exit_decl(value.into())
    }

    fn exit_command(&mut self, value: Command) -> T {
        self.exit_decl(value.into())
    }

    fn exit_stmt(&mut self, value: Stmt) -> T {
        default()
    }

    fn exit_ask(&mut self, value: Ask) -> T {
        self.exit_stmt(value.into())
    }

    fn exit_if_stmt(&mut self, value: IfStmt) -> T {
        self.exit_stmt(value.into())
    }

    fn exit_let_stmt(&mut self, value: LetStmt) -> T {
        self.exit_stmt(value.into())
    }

    fn exit_expr_stmt(&mut self, value: ExprStmt) -> T {
        self.exit_stmt(value.into())
    }

    fn exit_return(&mut self, value: Return) -> T {
        self.exit_stmt(value.into())
    }

    fn exit_expr(&mut self, value: Expr) -> T {
        default()
    }

    fn exit_unit(&mut self, value: Unit) -> T {
        self.exit_expr(value.into())
    }

    fn exit_group(&mut self, value: Group) -> T {
        self.exit_expr(value.into())
    }

    fn exit_infix(&mut self, value: Infix) -> T {
        self.exit_expr(value.into())
    }

    fn exit_app(&mut self, value: App) -> T {
        self.exit_expr(value.into())
    }

    fn exit_if(&mut self, value: If) -> T {
        self.exit_expr(value.into())
    }

    fn exit_match(&mut self, value: Match) -> T {
        self.exit_expr(value.into())
    }

    fn exit_dsl(&mut self, value: Dsl) -> T {
        self.exit_expr(value.into())
    }

    fn exit_array(&mut self, value: Array) -> T {
        self.exit_expr(value.into())
    }

    fn exit_lam(&mut self, value: Lam) -> T {
        self.exit_expr(value.into())
    }

    fn exit_let(&mut self, value: Let) -> T {
        self.exit_expr(value.into())
    }

    fn exit_ann(&mut self, value: Ann) -> T {
        self.exit_expr(value.into())
    }

    fn exit_qual(&mut self, value: Qual) -> T {
        self.exit_expr(value.into())
    }

    fn exit_pi(&mut self, value: Pi) -> T {
        self.exit_expr(value.into())
    }

    fn exit_sigma(&mut self, value: Sigma) -> T {
        self.exit_expr(value.into())
    }

    fn exit_help(&mut self, value: Help) -> T {
        self.exit_expr(value.into())
    }

    fn exit_local_expr(&mut self, value: LocalExpr) -> T {
        self.exit_expr(value.into())
    }

    fn exit_literal_expr(&mut self, value: LiteralExpr) -> T {
        self.exit_expr(value.into())
    }

    fn exit_body(&mut self, value: Body) -> T {
        default()
    }

    fn exit_do(&mut self, value: Do) -> T {
        self.exit_body(value.into())
    }

    fn exit_value(&mut self, value: Value) -> T {
        self.exit_body(value.into())
    }

    fn exit_pat(&mut self, value: Pat) -> T {
        default()
    }

    fn exit_literal_pat(&mut self, value: LiteralPat) -> T {
        self.exit_pat(value.into())
    }

    fn exit_unit_pat(&mut self, value: UnitPat) -> T {
        self.exit_pat(value.into())
    }

    fn exit_global_pat(&mut self, value: GlobalPat) -> T {
        self.exit_pat(value.into())
    }

    fn exit_constructor_pat(&mut self, value: ConstructorPat) -> T {
        self.exit_pat(value.into())
    }

    fn exit_list_pat(&mut self, value: ListPat) -> T {
        self.exit_pat(value.into())
    }

    fn exit_wildcard_pat(&mut self, value: WildcardPat) -> T {
        self.exit_pat(value.into())
    }

    fn exit_spread_pat(&mut self, value: SpreadPat) -> T {
        self.exit_pat(value.into())
    }
}
