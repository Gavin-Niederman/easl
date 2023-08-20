use std::path::PathBuf;

use clap::{Parser, Subcommand};
use easl::parser::ParserError;
use miette::{ErrReport, Result};
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Run { source_file: PathBuf },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { source_file } => {
            let Ok(source) = std::fs::read_to_string(source_file) else {
                return Err(miette::miette!("Could not read source file"));
            };

            let (mut statements, mut ident_map) =
                match easl::parser::parse(&source).map_err(|errs| -> Vec<ErrReport> {
                    errs.into_iter().map(|err| err.into()).collect()
                }) {
                    Ok((statements, mut ident_map)) => (statements, ident_map),
                    Err(errs) => {
                        for error in errs {
                            println!("{error}");
                        }
                        return Ok(());
                    }
                };
            println!("{:#?}", statements);

            easl::interpreter::interpret(statements, &source, ident_map)
                .map_err(<easl::interpreter::InterpreterError as Into<ErrReport>>::into)?;
        }
    }

    Ok(())
}
