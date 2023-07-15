use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

use asena_derive::*;

use asena_ast::command::{CommandHandler, Result};
use asena_ast::*;

use asena_report::{Diagnostic, Reports};
use im::HashMap;

use crate::PrecDatabase;

pub struct InfixHandler<'db> {
    pub db: &'db dyn PrecDatabase,
}

impl<'db> InfixHandler<'db> {
    pub fn new(db: &'db dyn PrecDatabase) -> Self {
        Self { db }
    }
}

impl<'db> Reports for InfixHandler<'db> {
    fn errors(&self) -> Arc<RwLock<Vec<Diagnostic<asena_report::BoxInternalError>>>> {
        todo!()
    }
}

#[ast_command(infixl, infixr)]
impl<'db> CommandHandler for InfixHandler<'db> {
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

        let prec_table = self.db.prec_table();
        let mut prec_table = prec_table.write().unwrap();
        prec_table.insert(FunctionId::new(&name), entry);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Assoc {
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct PrecTable(Arc<RwLock<HashMap<FunctionId, Entry>>>);

impl PartialEq for PrecTable {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for PrecTable {}

impl Default for PrecTable {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(default_prec_table())))
    }
}

impl DerefMut for PrecTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for PrecTable {
    type Target = Arc<RwLock<HashMap<FunctionId, Entry>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
