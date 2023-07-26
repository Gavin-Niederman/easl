use std::path::PathBuf;

use clap::Parser;
use miette::Result;
#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
source_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let Ok(source) = std::fs::read_to_string(args.source_file.clone()) else {
        panic!("{}", miette::miette!("Could not read source file"))
    };

   let ast = easl::parse(&source).unwrap();

    println!("{:#?}", ast);

    Ok(())
}
