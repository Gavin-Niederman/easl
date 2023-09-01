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
            

        }
    }

    Ok(())
}
