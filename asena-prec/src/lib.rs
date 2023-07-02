use asena_derive::*;

use asena_ast::{command::CommandWalker, walker::Reporter, *};

use im::HashMap;

pub mod commands;

pub use commands::*;

#[derive(Reporter)]
#[ast_step(
    WhereWalker,
    BranchWalker,
    VariantWalker,
    CommandWalker,
    FileWalker,
    BodyWalker,
    PatWalker,
    StmtWalker,
    FileWalker
)]
pub struct AsenaPrecReorder<'a, R: Reporter> {
    pub prec_table: &'a HashMap<FunctionId, Entry>,
    #[ast_reporter]
    pub reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaPrecReorder<'a, R> {
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

impl<'a, R: Reporter> AsenaPrecReorder<'a, R> {
    /// Reorder the precedence of the binary expression.
    fn impl_reorder_prec(&mut self, binary: &impl Binary) -> Option<()> {
        let lhs = binary.lhs();
        let fn_id = binary.fn_id();
        let rhs = binary.rhs().as_binary()?;

        let op1 = self.prec_table.get(&fn_id)?;
        let op2 = self.prec_table.get(&rhs.fn_id())?;

        if op1.order > op2.order {
            let new_lhs = binary.as_new_ast::<VirtualBinary>();
            new_lhs.set_lhs(lhs);
            new_lhs.set_fn_id(fn_id);
            new_lhs.set_rhs(rhs.find_lhs().as_new_node().as_leaf());

            binary.set_lhs(new_lhs);
            binary.set_fn_id(rhs.fn_id());
            binary.set_rhs(rhs.rhs());
        }

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use asena_ast::*;
    use asena_grammar::{asena_decl, asena_expr, asena_file, asena_stmt};
    use asena_leaf::ast::*;

    use crate::{commands::*, AsenaPrecReorder};

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();
        let mut tree = asena_file! {
            #infixr "@", 10;

            Main {
                let x = 2 @ 4 + 1;
                Println "hello world"
            }
        };

        let file = AsenaFile::new(tree.clone())
            .walks(AsenaInfixHandler::new(&mut tree, &mut prec_table))
            .walks(AsenaPrecReorder {
                prec_table: &prec_table,
                reporter: &mut tree,
            });

        tree.reporter.dump();

        println!("{file:#?}")
    }

    #[test]
    fn expr_works() {
        let prec_table = default_prec_table();
        let mut tree = asena_expr!(foo(1 * 2 + 4));
        let expr = Expr::new(tree.unwrap()).walks(AsenaPrecReorder {
            prec_table: &prec_table,
            reporter: &mut tree,
        });

        tree.reporter.dump();

        println!("{expr:#?}")
    }

    #[test]
    fn stmt_works() {
        let prec_table = default_prec_table();
        let mut tree = asena_stmt!(bar(foo(1 * 2 + 4)));
        let stmt = Stmt::new(tree.unwrap()).walks(AsenaPrecReorder {
            prec_table: &prec_table,
            reporter: &mut tree,
        });

        tree.reporter.dump();

        println!("{stmt:#?}")
    }

    #[test]
    fn decl_works() {
        let prec_table = default_prec_table();
        let mut tree = asena_decl! {
            #eval 1 * 2 + 4
        };
        let decl = Decl::new(tree.unwrap()).walks(AsenaPrecReorder {
            prec_table: &prec_table,
            reporter: &mut tree,
        });

        tree.reporter.dump();

        println!("{decl:#?}")
    }
}
