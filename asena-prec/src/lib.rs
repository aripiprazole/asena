use asena_ast::reporter::{Reporter, Reports};
use asena_ast::*;

use im::HashMap;

pub mod commands;

pub use commands::*;

pub struct PrecReorder<'a> {
    pub prec_table: &'a HashMap<FunctionId, Entry>,
    pub reporter: &'a mut Reporter,
}

impl Reports for PrecReorder<'_> {
    fn reports(&mut self) -> &mut Reporter {
        self.reporter
    }
}

impl AsenaVisitor<()> for PrecReorder<'_> {
    fn visit_infix(&mut self, value: Infix) {
        self.impl_reorder_prec(&value);
    }

    fn visit_accessor(&mut self, value: Accessor) {
        self.impl_reorder_prec(&value);
    }

    fn visit_ann(&mut self, value: Ann) {
        self.impl_reorder_prec(&value);
    }

    fn visit_qual(&mut self, value: Qual) {
        self.impl_reorder_prec(&value);
    }
}

impl PrecReorder<'_> {
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

    use crate::{commands::*, PrecReorder};

    #[test]
    fn it_works() {
        let mut prec_table = default_prec_table();
        let mut tree = asena_file! {
            #infixr "@", 1;

            Main {
                let x = 2 @ 2 * 4 + 1;
                Println "hello world"
            }
        };

        let file = AsenaFile::new(tree.clone())
            .walks(InfixHandler {
                prec_table: &mut prec_table,
                reporter: &mut tree.reporter,
            })
            .walks(PrecReorder {
                prec_table: &prec_table,
                reporter: &mut tree.reporter,
            });

        tree.reporter.dump();

        println!("{file:#?}")
    }

    #[test]
    fn expr_works() {
        let prec_table = default_prec_table();
        let mut tree = asena_expr!(foo(1 * 2 + 4));
        let expr = Expr::new(tree.unwrap()).walks(PrecReorder {
            prec_table: &prec_table,
            reporter: &mut tree.reporter,
        });

        tree.reporter.dump();

        println!("{expr:#?}")
    }

    #[test]
    fn stmt_works() {
        let prec_table = default_prec_table();
        let mut tree = asena_stmt!(bar(foo(1 * 2 + 4)));
        let stmt = Stmt::new(tree.unwrap()).walks(PrecReorder {
            prec_table: &prec_table,
            reporter: &mut tree.reporter,
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
        let decl = Decl::new(tree.unwrap()).walks(PrecReorder {
            prec_table: &prec_table,
            reporter: &mut tree.reporter,
        });

        tree.reporter.dump();

        println!("{decl:#?}")
    }
}
