use pest::Parser as PestParser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use thiserror::Error;

use crate::ast::*;
use crate::ast;

#[derive(Parser)]
#[grammar = "./parser.pest"] // relative to src/
pub struct HighRustParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Pest error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
    #[error("Unknown parse error")]
    Unknown,
    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),
    #[error("Parse error: {0}")]
    Custom(String),
}

/// Entry point for parsing HighRust source code.
pub fn parse(source: &str) -> Result<Module, ParseError> {
    let mut pairs = HighRustParser::parse(Rule::root, source)?;
    let module_pair = pairs.next().ok_or(ParseError::Unknown)?;
    build_module(module_pair)
}

fn build_module(pair: Pair<Rule>) -> Result<Module, ParseError> {
    assert_eq!(pair.as_rule(), Rule::root);
    let mut items = Vec::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::module => {
                for item in inner.into_inner() {
                    if let Some(module_item) = build_module_item(item)? {
                        items.push(module_item);
                    }
                }
            }
            _ => {}
        }
    }
    // For now, use dummy span
    Ok(Module {
        items,
        span: Span { start: 0, end: 0 },
    })
}

fn build_module_item(pair: Pair<Rule>) -> Result<Option<ModuleItem>, ParseError> {
    let span = get_span(&pair);
    match pair.as_rule() {
        Rule::import_stmt => Ok(Some(ModuleItem::Import(build_import(pair)?))),
        Rule::export_stmt => Ok(Some(ModuleItem::Export(build_export(pair)?))),
        Rule::data_def => Ok(Some(ModuleItem::Data(build_data_def(pair)?))),
        Rule::function_def => Ok(Some(ModuleItem::Function(build_function_def(pair)?))),
        Rule::embedded_rust_block => Ok(Some(ModuleItem::EmbeddedRust(build_embedded_rust(pair)?))),
        Rule::EOI => Ok(None),
        _ => Err(ParseError::UnexpectedRule(pair.as_rule())),
    }
}

// --- Helper functions for building AST nodes (stubs for now) ---

fn build_import(_pair: Pair<Rule>) -> Result<Import, ParseError> {
    Err(ParseError::Custom("Import parsing not yet implemented".to_string()))
}

fn build_export(_pair: Pair<Rule>) -> Result<Export, ParseError> {
    Err(ParseError::Custom("Export parsing not yet implemented".to_string()))
}

fn build_data_def(_pair: Pair<Rule>) -> Result<DataDef, ParseError> {
    Err(ParseError::Custom("DataDef parsing not yet implemented".to_string()))
}

fn build_function_def(_pair: Pair<Rule>) -> Result<FunctionDef, ParseError> {
    Err(ParseError::Custom("FunctionDef parsing not yet implemented".to_string()))
}

fn build_embedded_rust(_pair: Pair<Rule>) -> Result<EmbeddedRustBlock, ParseError> {
    Err(ParseError::Custom("EmbeddedRust parsing not yet implemented".to_string()))
}

// Utility: get span from pest Pair
fn get_span(pair: &Pair<Rule>) -> Span {
    let span = pair.as_span();
    Span {
        start: span.start(),
        end: span.end(),
    }
}
