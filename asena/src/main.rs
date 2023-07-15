#![feature(box_patterns)]
#![feature(concat_idents)]
#![feature(try_trait_v2)]
#![feature(lazy_cell)]
#![feature(downcast_unchecked)]

use std::{path::PathBuf, sync::Mutex};

use asena_ast_db::db::AstDatabaseStorage;
use asena_ast_lowering::db::AstLowerrerStorage;
use asena_grammar::Linebreak;
use asena_highlight::{Annotator, VirtualFile};
use asena_hir::interner::HirStorage;
use asena_lexer::Lexer;
use asena_prec::db::PrecStorage;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug, Clone)]
pub struct RenameArgs {}

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {}

#[derive(Args, Debug, Clone)]
#[clap(aliases = &["hi"])]
#[clap(
    about = "Highlights a `.ase` file with semantic or lexical analysis and print it on the standard output."
)]
pub struct HighlightArgs {
    /// Enables the semantic parsing highlight on Command Line Interface.
    #[clap(short = 's', long, default_value = "false")]
    pub semantic: bool,

    /// A "file.ase" to highlight
    #[clap(short = 'f', long)]
    pub file: String,
}

#[derive(Args, Debug, Clone)]
pub struct EvalArgs {
    /// Enables the verbose mode on Command Line Interface.
    #[clap(short = 'v', long, default_value = "false")]
    pub verbose: bool,

    /// A "file.ase" to evaluate
    #[clap(short = 'f', long)]
    pub file: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Rename(RenameArgs),
    Search(SearchArgs),
    Highlight(HighlightArgs),
    Eval(EvalArgs),
}

pub fn run_cli() {
    let cli = Cli::parse();

    use asena_parser::Parser;
    match cli.command {
        Command::Rename(..) => todo!(),
        Command::Search(..) => todo!(),
        Command::Highlight(args) if !args.semantic => {
            let path = args.file;
            let file = std::fs::read_to_string(path.clone()).unwrap();
            let lexer = Lexer::new(PathBuf::from(path), &file);
            let parser = Parser::from(lexer).run(asena_grammar::file);
            let tree = parser.build_tree();
            let file = VirtualFile::from(tree.data);
            println!("{file}")
        }
        Command::Highlight(args) => {
            let path = args.file;
            let file = std::fs::read_to_string(path.clone()).unwrap();
            let lexer = Lexer::new(PathBuf::from(path), &file);
            let parser = Parser::from(lexer).run(asena_grammar::file);
            let tree = parser.build_tree();
            let annotator = Annotator::new(asena_highlight::VirtualFile {
                contents: tree.data,
            });
            println!("{}", annotator.run_highlight());
        }
        Command::Eval(args) => {
            let path = args.file;
            let file = std::fs::read_to_string(path.clone()).unwrap();
            let lexer = Lexer::new(PathBuf::from(path), &file);
            let parser = Parser::from(lexer).run(|p| {
                asena_grammar::expr(p, Linebreak::Cont);
            });
            let tree = parser.build_tree();
            println!("{:#?}", tree.data());
        }
    }
}

fn main() {
    run_cli();
}

#[salsa::database(PrecStorage, AstDatabaseStorage, AstLowerrerStorage, HirStorage)]
#[derive(Default)]
pub struct DatabaseImpl {
    pub storage: salsa::Storage<DatabaseImpl>,
    pub logs: Mutex<Vec<salsa::Event>>,
}

impl salsa::Database for DatabaseImpl {
    fn salsa_event(&self, event_fn: salsa::Event) {
        self.logs.lock().unwrap().push(event_fn);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use asena_ast_db::{db::AstDatabase, package::*, vfs::*};
    use asena_ast_lowering::db::AstLowerrer;
    use asena_prec::PrecDatabase;

    use crate::DatabaseImpl;

    #[test]
    fn pipeline_works() {
        let db = DatabaseImpl::default();

        let local_pkg = Package::new(&db, "Local", "0.0.0", Arc::new(Default::default()));
        let file = VfsFileData::new(&db, "Test", "./Test.ase".into(), local_pkg);
        VfsFileData::new(&db, "Nat", "./Nat.ase".into(), local_pkg);
        VfsFileData::new(&db, "IO", "./IO.ase".into(), local_pkg);

        db.global_scope().borrow_mut().import(&db, file, None);

        let file = db.ast(file);
        let file = db.infix_commands(file);
        let file = db.ordered_prec(file);
        let hir = db.hir_file(file);

        db.lookup_intern_package(local_pkg).print_diagnostics(&db);
    }
}
