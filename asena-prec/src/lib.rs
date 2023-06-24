use asena_ast::{command::CommandWalker, walker::Reporter, *};
use asena_derive::ast_step;
use asena_report::InternalError;
use commands::Entry;
use im::HashMap;

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
    prec_table: &'a HashMap<FunctionId, Entry>,
    reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaPrecStep<'a, R> {
    fn walk_expr_infix(&mut self, value: &Infix) {
        self.impl_reorder_prec(value);
    }

    fn walk_expr_accessor(&mut self, value: &Accessor) {
        self.impl_reorder_prec(value);
    }

    fn walk_expr_ann(&mut self, value: &Ann) {
        self.impl_reorder_prec(value);
    }

    fn walk_expr_qual(&mut self, value: &Qual) {
        self.impl_reorder_prec(value);
    }
}

impl<'a, R: Reporter> AsenaPrecStep<'a, R> {
    pub fn new(reporter: &'a mut R, prec_table: &'a HashMap<FunctionId, Entry>) -> Self {
        Self {
            reporter,
            prec_table,
        }
    }

    fn impl_reorder_prec(&mut self, binary: &impl Binary) {
        println!("current prec table  {:?}", self.prec_table);

        let lhs = binary.find_lhs();
        let rhs = binary.find_rhs();

        let new_rhs = rhs.as_new_node();

        rhs.set(lhs.clone());
        lhs.set(new_rhs);
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
    use asena_ast::*;
    use asena_grammar::{asena_expr, asena_file};
    use asena_leaf::ast::*;

    use crate::{commands::*, AsenaPrecStep};

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();
        let mut tree = asena_file! {
            #infixr "+", (10 + 3);

            Main {
                let x = 4 + 1;
                Println "hello world"
            }
        };

        let file = AsenaFile::new(tree.clone().unwrap())
            .walks(AsenaInfixCommandStep::new(&mut tree, &mut prec_table))
            .walks(AsenaPrecStep::new(&mut tree, &prec_table));

        tree.reporter.dump();

        println!("{file:#?}")
    }

    #[test]
    fn expr_works() {
        let tree = asena_expr! { 1 + 1 };
        let expr = Expr::from(Infix::new(tree.unwrap()));

        println!("{expr:#?}")
    }
}
