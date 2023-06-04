use std::path::PathBuf;

use miette::Result;
use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    source_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Ok(source) = std::fs::read_to_string(args.source_file) {
        let tokens = easl::scan(source)?;
    
        println!("{:?}", tokens.collect::<Vec<easl::Token>>());
    } else {
        return Err(miette::Report::msg("Could not read source file"));
    }

    Ok(())
}
