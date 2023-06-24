use asena_ast::{command::CommandWalker, *};
use asena_derive::ast_step;

pub mod commands;

#[ast_step(
    CommandWalker,
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker
)]
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
    use asena_ast::AsenaFile;
    use asena_grammar::asena_file;
    use asena_leaf::ast::Walkable;

    use crate::{commands::AsenaInfixCommandStep, AsenaPrecStep};

    #[test]
    fn it_works() {
        let mut prec_table = AsenaInfixCommandStep::default_prec_table();

        let file = AsenaFile::new(asena_file! {
            #infixr +, 10;

            Main {
                Println "hello world"
            }
        })
        .walks(AsenaInfixCommandStep::new(&mut prec_table))
        .walks(AsenaPrecStep);

        println!("{file:#?}")
    }
}
