use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::{Result, ErrReport};
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

            let ast = easl::parser::parse(&source).map_err(|err|  <easl::parser::ParserError as Into<ErrReport>>::into(err))?;
            println!("{:#?}", ast);
            
            easl::interpreter::interpret(ast, &source).map_err(|err| <easl::interpreter::InterpreterError as Into<ErrReport>>::into(err))?;
        }
    }

    Ok(())
}
