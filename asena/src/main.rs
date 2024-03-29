#![feature(panic_info_message)]
#![feature(box_patterns)]
#![feature(concat_idents)]
#![feature(try_trait_v2)]
#![feature(lazy_cell)]
#![feature(downcast_unchecked)]

use std::path::PathBuf;

use asena_grammar::Linebreak;
use asena_highlight::{Annotator, VirtualFile};
use asena_lexer::Lexer;
use clap::{Args, Parser, Subcommand};

pub mod imp;
pub mod panik;

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
    env_logger::init();
    panik::install_asena_panic_hook();

    run_cli();
}

#[cfg(test)]
mod tests {
    use asena_ast_db::{db::AstDatabase, package::*, vfs::*};
    use asena_hir_lowering::LlirConfig;
    use std::sync::Arc;

    #[test]
    fn pipeline_works() {
        env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Error)
            .try_init()
            .unwrap();

        crate::panik::install_asena_panic_hook();

        let db = crate::imp::DatabaseImpl::default();

        let local_pkg = Package::new(&db, "Local", "0.0.0", Arc::new(Default::default()));
        let file = VfsFileData::new(&db, "Test", "./Test.ase".into(), local_pkg);
        VfsFileData::new(&db, "Nat", "./Nat.ase".into(), local_pkg);
        VfsFileData::new(&db, "IO", "./IO.ase".into(), local_pkg);

        db.global_scope().write().unwrap().import(&db, file, None);
        db.run_pipeline_catching(file, LlirConfig::default());
        db.lookup_intern_package(local_pkg).print_diagnostics(&db);
    }
}
