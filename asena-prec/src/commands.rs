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
        let mut entry = Entry::new(FunctionId::new(&name), Assoc::Left, order);

        if command.is_command("infixr") {
            entry = Entry::new(FunctionId::new(&name), Assoc::Right, order);
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
    table.insert("+".into(), Entry::new("+", Assoc::Right, 2));
    table.insert("-".into(), Entry::new("-", Assoc::Right, 2));

    // `->`, `=>`
    table.insert("->".into(), Entry::new("->", Assoc::Right, 1));
    table.insert("=>".into(), Entry::new("=>", Assoc::Right, 1));

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
