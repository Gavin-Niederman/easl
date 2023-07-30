use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "easl.pest"]
pub struct EaslParser;

pub fn parse(source: &str) -> Result<Pair<'_, Rule>, pest::error::Error<Rule>> {
    // EaslParser::parse(Rule::main, &source).expect("Failed to parse file").next().unwrap()

    match EaslParser::parse(Rule::file, source) {
        Ok(mut pairs) => Ok(pairs.next().unwrap()),
        Err(err) => Err(err),
    }
}
