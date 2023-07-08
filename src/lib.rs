#![feature(iter_next_chunk)]

pub mod parse;
pub mod scan;

use std::path::PathBuf;

use clap::Parser;
pub use parse::*;
pub use scan::*;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    source_file: PathBuf,
}

lazy_static::lazy_static! {
    pub static ref ARGS: Args = Args::parse();
    pub static ref SOURCE: &'static str = {
        let Ok(source) = std::fs::read_to_string(ARGS.source_file.clone()) else {
            panic!("{}", miette::miette!("Could not read source file"))
        };

        Box::leak(source.into_boxed_str())
    };
}
