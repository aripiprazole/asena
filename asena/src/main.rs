#![feature(box_patterns)]
#![feature(concat_idents)]
#![feature(try_trait_v2)]
#![feature(lazy_cell)]
#![feature(downcast_unchecked)]

use asena_highlight::VirtualFile;
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

    match cli.command {
        Command::Rename(..) => todo!(),
        Command::Search(..) => todo!(),
        Command::Highlight(args) if !args.semantic => {
            let path = args.file;
            let file = std::fs::read_to_string(path).unwrap();
            let lexer = asena_lexer::Lexer::new(&file);
            let parser = asena_parser::Parser::from(lexer).run(asena_grammar::file);
            let tree = parser.build_tree();
            let file = VirtualFile::from(tree.data);
            println!("{file}")
        }
        Command::Highlight(args) => {
            let path = args.file;
            let file = std::fs::read_to_string(path).unwrap();
            let lexer = asena_lexer::Lexer::new(&file);
            let parser = asena_parser::Parser::from(lexer).run(asena_grammar::file);
            let tree = parser.build_tree();
            let file = VirtualFile::from(tree.data);
            println!("{file}");
            println!("TODO: not implemented semantic highlighting");
        }
        Command::Eval(args) => {
            let path = args.file;
            let file = std::fs::read_to_string(path).unwrap();
            let lexer = asena_lexer::Lexer::new(&file);
            let mut parser = asena_parser::Parser::from(lexer);
            asena_grammar::expr(&mut parser, asena_grammar::Linebreak::Cont);
            let tree = parser.build_tree();
            println!("{:#?}", tree.data());
        }
    }
}

fn main() {
    run_cli();
}
