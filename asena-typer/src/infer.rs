use asena_derive::*;

use asena_ast::command::*;
use asena_ast::walker::Reporter;
use asena_ast::*;

pub struct ClassEnvironment {}

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
pub struct AsenaTyperStep<'a, R: Reporter> {
    pub class_env: &'a mut ClassEnvironment,

    #[ast_reporter]
    reporter: &'a mut R,
}

impl<'a, R: Reporter> ExprWalker for AsenaTyperStep<'a, R> {}
