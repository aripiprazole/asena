use asena_derive::*;

use asena_ast::command::{CommandHandler, Result};
use asena_ast::reporter::{Reporter, Reports};
use asena_ast::*;

use im::HashMap;

pub struct InfixHandler<'a> {
    pub prec_table: &'a mut HashMap<FunctionId, Entry>,
    pub reporter: &'a mut Reporter,
}

impl Reports for InfixHandler<'_> {
    fn reports(&mut self) -> &mut Reporter {
        self.reporter
    }
}

#[ast_command(infixl, infixr)]
impl CommandHandler for InfixHandler<'_> {
    fn on_command(&mut self, command: Command) -> Result {
        let name = command.at::<LiteralExpr>(0)?.literal().contents();
        let order = command
            .at::<LiteralExpr>(1)?
            .literal()
            .to_u8()
            .unwrap_or_default();
        let mut entry = Entry::new(FunctionId::new(&name), Assoc::Left, order);

        if command.is_command("infixr") {
            entry = Entry::new(FunctionId::new(&name), Assoc::Right, order);
        }

        self.prec_table.insert(FunctionId::new(&name), entry);

        Ok(())
    }
}

/// The default precedence table for Asena Language in the Standard Library.
///   - `->`, `=>`
///   - `^`, `>>`, `<<`, `|`, `&`
///   - `>`, `>=`, `<=`, `<`
///   - `==`, `!=`
///   - `||`, `&&`
///   - `$`, `%`, `=>>`, `@`
///   - `^^`
///   - `*`, `/`
///   - `+`, `-`
pub fn default_prec_table() -> HashMap<FunctionId, Entry> {
    let mut table = HashMap::new();

    // `^`, `>>`, `<<`, `|`, `&`
    table.insert("^".into(), Entry::new("==", Assoc::Right, 9));
    table.insert("|".into(), Entry::new("|", Assoc::Right, 9));
    table.insert("&".into(), Entry::new("&", Assoc::Right, 9));
    table.insert(">>".into(), Entry::new(">>", Assoc::Right, 9));
    table.insert("<<".into(), Entry::new("<<", Assoc::Right, 9));

    // `>`, `>=`, `<=`, `<`
    table.insert(">".into(), Entry::new("<", Assoc::Right, 8));
    table.insert("<".into(), Entry::new("<", Assoc::Right, 8));
    table.insert(">=".into(), Entry::new(">=", Assoc::Right, 8));
    table.insert("<=".into(), Entry::new("<=", Assoc::Right, 8));

    // `==`, `!=`
    table.insert("==".into(), Entry::new("==", Assoc::Right, 7));
    table.insert("!=".into(), Entry::new("!=", Assoc::Right, 7));

    // `||`, `&&`
    table.insert("||".into(), Entry::new("||", Assoc::Right, 6));
    table.insert("&&".into(), Entry::new("&&", Assoc::Right, 6));

    // `$`, `%`, `=>>`, `@`
    table.insert("$".into(), Entry::new("$", Assoc::Right, 5));
    table.insert("%".into(), Entry::new("%", Assoc::Right, 5));
    table.insert("@".into(), Entry::new("@", Assoc::Right, 5));
    table.insert("=>>".into(), Entry::new("=>>", Assoc::Right, 5));

    // `^^`
    table.insert("^^".into(), Entry::new("^^", Assoc::Right, 4));

    // `*`, `/`
    table.insert("*".into(), Entry::new("*", Assoc::Right, 2));
    table.insert("/".into(), Entry::new("/", Assoc::Right, 2));

    // `+`, `-`
    table.insert("+".into(), Entry::new("+", Assoc::Right, 1));
    table.insert("-".into(), Entry::new("-", Assoc::Right, 1));

    table
}

#[derive(Debug, Clone)]
pub enum Assoc {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub fn_id: FunctionId,
    pub assoc: Assoc,
    pub order: u8,
}

impl Entry {
    pub fn new<I: Into<FunctionId>>(fn_id: I, assoc: Assoc, order: u8) -> Self {
        Self {
            fn_id: fn_id.into(),
            assoc,
            order,
        }
    }
}
