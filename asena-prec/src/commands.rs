use asena_ast::{command::CommandWalker, *};
use asena_derive::{ast_command, ast_step};
use im::HashMap;

#[ast_step(BodyWalker, PropertyWalker, ExprWalker, PatWalker, StmtWalker)]
pub struct AsenaInfixCommandStep<'a> {
    pub prec_table: &'a mut HashMap<FunctionId, PrecedenceEntry>,
}

#[ast_command(infixl, infixr)]
impl CommandWalker for AsenaInfixCommandStep<'_> {
    fn on_command(&mut self, command: &Command) -> asena_ast::decl::command::Result {
        let name = command.at::<QualifiedPath>(0)?.to_fn_id();
        let order = command.at::<Literal>(0)?.to_u8().unwrap_or_default();
        let mut entry = PrecedenceEntry::new(Assoc::Left, order);

        if command.is_command("infixr") {
            entry = PrecedenceEntry::new(Assoc::Right, order);
        }

        self.prec_table.insert(name, entry);

        Ok(())
    }
}

impl<'a> AsenaInfixCommandStep<'a> {
    pub fn new(prec_table: &'a mut HashMap<FunctionId, PrecedenceEntry>) -> Self {
        Self { prec_table }
    }

    pub fn default_prec_table() -> HashMap<FunctionId, PrecedenceEntry> {
        let mut table = HashMap::new();
        table.insert(FunctionId::new("=="), PrecedenceEntry::new(Assoc::Right, 0));
        table
    }
}

#[derive(Debug, Clone)]
pub enum Assoc {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct PrecedenceEntry {
    pub assoc: Assoc,
    pub order: u8,
}

impl PrecedenceEntry {
    pub fn new(assoc: Assoc, order: u8) -> Self {
        Self { assoc, order }
    }
}
