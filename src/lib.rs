pub mod compiler;
pub mod interpreter;
pub mod parser;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Run { source_file: PathBuf },
}

// This is needed because of stupid shit that chumsky does with errors
// I Really Really want this to be gone forever
lazy_static::lazy_static! {
    pub static ref SOURCE: Option<String> = match Args::parse().command {
        Commands::Run { source_file } => {
            // We unwrap here because if this lazy static is ever evaluated the file has to exists.
            // See? Bad code that I want to die.
            Some(std::fs::read_to_string(source_file).unwrap())
        }
        // Do this for when there is more than one command
        _ => None
    };
}