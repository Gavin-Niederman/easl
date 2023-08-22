use std::path::PathBuf;

use clap::Parser;
use easl::{Args, Commands};
use miette::{Result, ErrReport};

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Run { source_file } => {
            let Ok(source) = std::fs::read_to_string(source_file) else {
                return Err(miette::miette!("Could not read source file"));
            };

            let (statements, ident_map) =
                match easl::parser::parse(&source).map_err(|errs| -> Vec<ErrReport> {
                    errs.into_iter().map(|err| err.into()).collect()
                }) {
                    Ok((statements, ident_map)) => (statements, ident_map),
                    Err(errs) => {
                        for error in errs {
                            return Err(error);
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
