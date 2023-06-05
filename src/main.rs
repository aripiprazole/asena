#![feature(box_patterns)]
#![feature(concat_idents)]
#![feature(try_trait_v2)]
#![feature(lazy_cell)]
#![feature(downcast_unchecked)]

pub mod ast;
pub mod graph;
pub mod incremental;
pub mod lexer;
pub mod parser;
pub mod report;

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
pub struct EvalArgs {
    /// Enables the verbose mode on Command Line Interface.
    #[clap(short = 'v', long, default_value = "false")]
    pub verbose: bool,

    /// A "file.brex" to evaluate
    #[clap(short = 'f', long)]
    pub file: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Rename(RenameArgs),
    Search(SearchArgs),
    Eval(EvalArgs),
}

pub fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Command::Rename(..) => todo!(),
        Command::Search(..) => todo!(),
        Command::Eval(args) => {
            let path = args.file;
            let file = std::fs::read_to_string(path).unwrap();
            let lexer = lexer::Lexer::new(&file);
            let mut parser = parser::Parser::from(lexer);
            parser::grammar::expr(&mut parser);
            let tree = parser.build_tree();
            println!("{:#?}", tree.data());
        }
    }
}

fn main() {
    run_cli();
}
