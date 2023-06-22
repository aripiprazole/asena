use asena_ast::{Accessor, Ann, Binary, ExprWalker, Infix, PatWalker, Qual, StmtWalker};
use asena_derive::ast_step;

#[ast_step(PatWalker, StmtWalker)]
pub struct StepAsenaPrec;

impl ExprWalker for StepAsenaPrec {
    fn walk_expr_infix(&self, value: &Infix) {
        impl_reorder_prec(value);
    }

    fn walk_expr_accessor(&self, value: &Accessor) {
        impl_reorder_prec(value);
    }

    fn walk_expr_ann(&self, value: &Ann) {
        impl_reorder_prec(value);
    }

    fn walk_expr_qual(&self, value: &Qual) {
        impl_reorder_prec(value);
    }
}

fn impl_reorder_prec(binary: &impl Binary) {
    binary.lhs();
    binary.fn_id();
    binary.rhs();
}

#[cfg(test)]
mod tests {
    use asena_ast::{Expr, Infix};
    use asena_leaf::ast::Walkable;
    use asena_lexer::Lexer;
    use asena_parser::Parser;

    use crate::StepAsenaPrec;

    #[test]
    fn it_works() {
        let tree = Parser::from(Lexer::new("1 + 1"))
            .run(asena_grammar::expr)
            .build_tree()
            .unwrap();

        let tree = Expr::from(Infix::new(tree));
        tree.walk(&StepAsenaPrec);

        println!("{tree:?}")
    }
}
