use asena_derive::*;

use asena_ast::walker::Reporter;
use asena_ast::*;
use asena_leaf::ast::Walkable;

use crate::validation::AsenaTypeValidator;

#[derive(Default)]
pub struct ClassEnvironment {}

#[derive(Default)]
pub struct TypeEnvironment {}

#[derive(Reporter)]
#[ast_step(
    FileWalker,
    BodyWalker,
    PropertyWalker,
    PatWalker,
    StmtWalker,
    FileWalker
)]
pub struct AsenaTyperStep<'a, R: Reporter> {
    pub type_env: &'a mut TypeEnvironment,
    pub class_env: &'a mut ClassEnvironment,

    #[ast_reporter]
    pub reporter: &'a mut R,
}

impl<'a, R: Reporter> DeclWalker for AsenaTyperStep<'a, R> {
    fn walk_decl_signature(&mut self, value: &Signature) {
        // Check the return type of the signature.
        value.return_type().walk(&mut AsenaTypeValidator {
            reporter: self.reporter,
        });
    }
}

impl<'a, R: Reporter> ExprWalker for AsenaTyperStep<'a, R> {}
