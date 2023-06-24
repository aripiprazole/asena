use asena_ast::{command::CommandWalker, walker::Reporter, *};
use asena_derive::ast_step;
use asena_report::InternalError;

pub mod commands;

#[ast_step(
    CommandWalker,
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker
)]
pub struct AsenaPrecStep<'a, R: Reporter> {
    reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaPrecStep<'a, R> {
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

impl<'a, R: Reporter + Clone> AsenaPrecStep<'a, R> {
    pub fn new(reporter: &'a mut R) -> Self {
        Self { reporter }
    }
}

impl<'a, R: Reporter> Reporter for AsenaPrecStep<'a, R> {
    fn diagnostic<E: InternalError, T>(&mut self, error: E, at: asena_span::Spanned<T>)
    where
        E: 'static,
    {
        self.reporter.diagnostic(error, at);
    }
}

#[cfg(test)]
mod tests {
    use asena_ast::AsenaFile;
    use asena_grammar::asena_file;
    use asena_leaf::ast::Walkable;

    use crate::{
        commands::{default_prec_table, AsenaInfixCommandStep},
        AsenaPrecStep,
    };

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();
        let mut tree = asena_file! {
            #infixr +, 10;

            Main {
                Println "hello world"
            }
        };

        let file = AsenaFile::new(tree.clone().unwrap())
            .walks(AsenaInfixCommandStep::new(&mut tree, &mut prec_table))
            .walks(AsenaPrecStep::new(&mut tree));

        tree.reporter.dump();

        println!("{file:#?}")
    }
}
