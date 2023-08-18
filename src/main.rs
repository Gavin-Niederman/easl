use std::path::PathBuf;

use clap::{Parser, Subcommand};
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

            let (mut statements, mut ident_map) = easl::parser::parse(&source)
                .map_err(<easl::parser::ParserError as Into<ErrReport>>::into)?;
            println!("{:#?}", statements);


            // Complicated include logic. I would love to simplify this.
            let includes = statements.clone().into_iter().filter_map(|statement| match statement {
                easl::parser::ast::Statement::Include { source } => Some(source),
                _ => None,
            }).map(|source| {
                easl::parser::parse(&source)
                    .map_err(<easl::parser::ParserError as Into<ErrReport>>::into)
            });

            for include in includes {
                let (mut include_statements, include_ident_map) = include?;
                statements.append(&mut include_statements);
                ident_map.map.extend(include_ident_map.map);
            }

            easl::interpreter::interpret(statements, &source, ident_map)
                .map_err(<easl::interpreter::InterpreterError as Into<ErrReport>>::into)?;
        }
    }

    Ok(())
}
