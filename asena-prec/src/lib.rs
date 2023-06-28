use asena_ast::{command::CommandWalker, walker::Reporter, *};
use asena_derive::{ast_reporter, ast_step, Reporter};
use asena_leaf::node::TreeKind;
use asena_report::InternalError;
use commands::Entry;
use im::HashMap;

pub mod commands;

#[derive(Reporter)]
#[ast_step(
    CommandWalker,
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker,
    FileWalker
)]
pub struct AsenaPrecStep<'a, R: Reporter> {
    prec_table: &'a HashMap<FunctionId, Entry>,
    #[ast_reporter]
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
    /// Reorder the precedence of the binary expression.
    fn impl_reorder_prec(&mut self, binary: &impl Binary) -> Option<()> {
        let lhs = binary.lhs();
        let fn_id = binary.fn_id();
        let rhs = binary.rhs().as_binary()?;

        let op1 = self.prec_table.get(&fn_id)?;
        let op2 = self.prec_table.get(&rhs.fn_id())?;

        if op1.order > op2.order {
            let new_lhs = Infix::from(TreeKind::ExprBinary);
            new_lhs.set_lhs(lhs);
            new_lhs.set_fn_id(fn_id);
            new_lhs.set_rhs(rhs.find_lhs().as_new_node().as_leaf());

            binary.set_lhs(new_lhs.into());
            binary.set_fn_id(rhs.fn_id());
            binary.set_rhs(rhs.rhs());
        }

        Some(())
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

        let file = AsenaFile::new(tree.clone())
            .walks(AsenaInfixCommandStep::new(&mut tree, &mut prec_table))
            .walks(AsenaPrecStep {
                prec_table: &prec_table,
                reporter: &mut tree,
            });

        tree.reporter.dump();

        println!("{file:#?}")
    }

    #[test]
    fn expr_works() {
        let prec_table = default_prec_table();
        let mut tree = asena_expr!(1 * 2 + 5 * 4 + 3);
        let expr = Expr::new(tree.unwrap()).walks(AsenaPrecStep {
            prec_table: &prec_table,
            reporter: &mut tree,
        });

        tree.reporter.dump();

        println!("{expr:#?}")
    }
}
