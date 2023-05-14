pub mod ast;
pub mod graph;
pub mod lexer;
pub mod parser;

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
    #[clap(short = 'v', long, default_value = "auto")]
    pub verbose: bool,
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
        Command::Eval(..) => todo!(),
    }
}

fn main() {
    run_cli();
}
