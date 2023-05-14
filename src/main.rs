pub mod ast;
pub mod graph;
pub mod lexer;
pub mod parser;
use std::ops::Range;

use clap::{Args, Parser, Subcommand};

use crate::lexer::lexer;

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
    file: String,
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
            use ariadne::{Color, Label, Report, ReportKind, Source};
            use chumsky::Parser;

            let path = args.file;
            let file = std::fs::read_to_string(path).unwrap();
            let (tokens, errs) = lexer().parse(&file).into_output_errors();

            for err in errs {
                Report::<Range<usize>>::build(ReportKind::Error, (), 0)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(err.span().into_range())
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print(Source::from(file.clone()))
                    .unwrap();
            }

            println!("{:?}", tokens);
        }
    }
}

fn main() {
    run_cli();
}
