use asena_ast::{command::CommandWalker, decl::command::Result, walker::Reporter, *};
use asena_derive::{ast_command, ast_step};
use asena_leaf::ast::Lexeme;
use asena_report::InternalError;
use im::HashMap;

#[ast_step(
    FileWalker,
    BodyWalker,
    PropertyWalker,
    ExprWalker,
    PatWalker,
    StmtWalker
)]
pub struct AsenaInfixCommandStep<'a, R: Reporter> {
    pub prec_table: &'a mut HashMap<FunctionId, Entry>,
    pub reporter: &'a mut R,
}

#[ast_command(infixl, infixr)]
impl<'a, R: Reporter> CommandWalker for AsenaInfixCommandStep<'a, R> {
    fn on_command(&mut self, command: &Command) -> Result {
        let name = command.at::<Lexeme<Literal>>(0)?.contents();
        let order = command
            .at::<Lexeme<Literal>>(1)?
            .to_u8()
            .unwrap_or_default();
        let mut entry = Entry::new(Assoc::Left, order);

        if command.is_command("infixr") {
            entry = Entry::new(Assoc::Right, order);
        }

        self.prec_table.insert(FunctionId::new(&name), entry);

        Ok(())
    }
}

impl<'a, R: Reporter> Reporter for AsenaInfixCommandStep<'a, R> {
    fn diagnostic<E: InternalError, T>(&mut self, error: E, at: asena_span::Spanned<T>)
    where
        E: 'static,
    {
        self.reporter.diagnostic(error, at)
    }
}

impl<'a, R: Reporter> AsenaInfixCommandStep<'a, R> {
    pub fn new(reporter: &'a mut R, prec_table: &'a mut HashMap<FunctionId, Entry>) -> Self {
        Self {
            prec_table,
            reporter,
        }
    }
}

pub fn default_prec_table() -> HashMap<FunctionId, Entry> {
    let mut table = HashMap::new();
    table.insert(FunctionId::new("=="), Entry::new(Assoc::Right, 0));
    table
}

#[derive(Debug, Clone)]
pub enum Assoc {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub assoc: Assoc,
    pub order: u8,
}

impl Entry {
    pub fn new(assoc: Assoc, order: u8) -> Self {
        Self { assoc, order }
    }
}
