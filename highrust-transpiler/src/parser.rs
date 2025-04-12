use pest::Parser as PestParser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./parser.pest"] // relative to src/
pub struct HighRustParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Pest error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
    #[error("Unknown parse error")]
    Unknown,
}

/// Entry point for parsing HighRust source code.
/// For now, this is a stub that just attempts to parse the root rule.
pub fn parse(source: &str) -> Result<(), ParseError> {
    let _pairs = HighRustParser::parse(self::Rule::root, source)?;
    // TODO: Build AST from pairs
    Ok(())
}
