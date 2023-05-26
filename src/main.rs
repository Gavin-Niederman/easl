use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    source_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let args_copy = args.clone();
    let source = std::fs::read_to_string(args.source_file).with_context({
        || format!("Failed to read file {}", args_copy.source_file.display())
    })?;

    let tokens = easl::scan(source).with_context(|| "Failed to parse source file")?;

    println!("{tokens:?}");

    Ok(())
}
