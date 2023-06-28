use asena_derive::*;

use asena_ast::command::*;
use asena_ast::walker::Reporter;
use asena_ast::*;

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
pub struct AsenaTypingValidationStep<'a, R: Reporter> {
    #[ast_reporter]
    reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaTypingValidationStep<'a, R> {}
