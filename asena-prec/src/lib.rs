use asena_ast::{Accessor, Ann, Binary, ExprWalker, Infix, PatWalker, Qual, StmtWalker};
use asena_derive::ast_step;

#[ast_step(PatWalker, StmtWalker)]
pub struct AsenaPrecStep;

impl ExprWalker for AsenaPrecStep {
    fn walk_expr_infix(&mut self, value: &Infix) {
        impl_reorder_prec(value);
    }

    fn walk_expr_accessor(&mut self, value: &Accessor) {
        impl_reorder_prec(value);
    }

    fn walk_expr_ann(&mut self, value: &Ann) {
        impl_reorder_prec(value);
    }

    fn walk_expr_qual(&mut self, value: &Qual) {
        impl_reorder_prec(value);
    }
}

fn impl_reorder_prec(binary: &impl Binary) {
    let lhs = binary.find_lhs();
    let rhs = binary.find_rhs();

    let new_rhs = rhs.as_new_node();

    rhs.set(lhs.clone());
    lhs.set(new_rhs);
}

#[cfg(test)]
mod tests {
    use asena_ast::{Expr, Infix};
    use asena_leaf::ast::Walkable;
    use asena_lexer::Lexer;
    use asena_parser::Parser;

    use crate::AsenaPrecStep;

    #[test]
    fn it_works() {
        let tree = Parser::from(Lexer::new("(2 + 3) + 1"))
            .run(asena_grammar::expr)
            .build_tree()
            .unwrap();

        let tree = Expr::from(Infix::new(tree));
        tree.run(AsenaPrecStep);

        println!("{tree:#?}")
    }
}
