//! Parser for HighRust language.
//!
//! This module defines the parser for HighRust, using the Pest parser generator.
//! It converts the lexical tokens from Pest into an Abstract Syntax Tree (AST).

use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use std::fmt;
use crate::ast::{
    Block, Expr, FunctionDef, Literal, Module, ModuleItem, Param, Span, Stmt,
};

/// Errors that can occur during parsing.
#[derive(Debug)]
pub enum ParseError {
    PestError(Box<pest::error::Error<Rule>>),
    UnexpectedRule(Rule),
    Unknown,
    Custom(String),
}

/// Parse a string of HighRust source code into an AST.
pub fn parse(source: &str) -> Result<Module, ParseError> {
    println!("Parsing source code");
    let mut pairs = HighRustParser::parse(Rule::root, source)?;
    let module_pair = pairs.next().ok_or(ParseError::Unknown)?;
    build_module(module_pair)
}

/// Construct a Module from a Pest parse tree.
fn build_module(pair: Pair<Rule>) -> Result<Module, ParseError> {
    println!("Building module from rule: {:?}", pair.as_rule());
    let mut items = Vec::new();
    for inner in pair.into_inner() {
        println!("Module child rule: {:?}", inner.as_rule());
        if inner.as_rule() == Rule::module {
            println!("Found module rule");
            // Extract function_def rules from the module
            for module_item in inner.into_inner() {
                println!("Module item rule: {:?}", module_item.as_rule());
                if module_item.as_rule() == Rule::function_def {
                    println!("Found function_def rule");
                    items.push(ModuleItem::Function(build_function_def(module_item)?));
                } else {
                    println!("Ignoring module item rule: {:?}", module_item.as_rule());
                }
            }
        } else if inner.as_rule() == Rule::function_def {
            println!("Found direct function_def rule");
            items.push(ModuleItem::Function(build_function_def(inner)?));
        } else {
            println!("Ignoring rule: {:?}", inner.as_rule());
        }
    }
    println!("Module has {} items", items.len());
    Ok(Module {
        items,
        span: Span { start: 0, end: 0 },
    })
}

/// Build a ModuleItem from a Pest pair.
fn build_module_item(pair: Pair<Rule>) -> Result<Option<ModuleItem>, ParseError> {
    println!("Building module item from rule: {:?}", pair.as_rule());
    match pair.as_rule() {
        Rule::function_def => {
            println!("Found function_def rule");
            Ok(Some(ModuleItem::Function(build_function_def(pair)?)))
        },
        _ => {
            println!("Ignoring rule: {:?}", pair.as_rule());
            Ok(None) // Only function_def supported in MVP
        }
    }
}

/// Build a FunctionDef from a Pest pair.
fn build_function_def(pair: Pair<Rule>) -> Result<FunctionDef, ParseError> {
    // function_def = { fn_keyword ~ function_name ~ function_params ~ block_expr }
    let span = get_span(&pair);
    println!("Function def span: {:?}", span);
    println!("Function def text: {}", pair.as_str());
    
    let mut inner = pair.into_inner();
    println!("Function def inner count: {}", inner.clone().count());
    
    // Skip fn_keyword
    let _fn_kw = inner.next();
    
    // Get function_name
    let name_pair = inner.next().ok_or(ParseError::Unknown)?;
    let name_inner = name_pair.into_inner().next().ok_or(ParseError::Unknown)?;
    let name = name_inner.as_str().to_string();
    println!("Parsed function name: {}", name);
    
    // Get function_params
    let params_pair = inner.next().ok_or(ParseError::Unknown)?;
    
    // Extract parameters
    let mut params = Vec::new();
    for param_pair in params_pair.into_inner() {
        if param_pair.as_rule() == Rule::param {
            println!("Found param: {}", param_pair.as_str());
            params.push(Param {
                name: param_pair.as_str().to_string(),
                ty: None,
                span: get_span(&param_pair),
            });
        }
    }
    println!("Parsed {} parameters", params.len());
    
    // Get block_expr
    let body_pair = inner.next().ok_or(ParseError::Unknown)?;
    let body = build_block(body_pair)?;
    println!("Function body has {} statements", body.stmts.len());
    
    Ok(FunctionDef {
        name,
        params,
        ret_type: None,
        body,
        is_async: false,
        is_rust: false,
        span,
    })
}

/// Build a Block from a Pest pair.
fn build_block(pair: Pair<Rule>) -> Result<Block, ParseError> {
    let span = get_span(&pair);
    let mut stmts = Vec::new();
    for part in pair.into_inner() {
        println!("Block child rule: {:?}", part.as_rule());
        match part.as_rule() {
            Rule::stmt => {
                println!("Found stmt rule");
                stmts.push(build_stmt(part)?);
            },
            _ => {
                println!("Ignoring rule in block: {:?}", part.as_rule());
            }
        }
    }
    Ok(Block { stmts, span })
}

/// Build a statement from a Pest pair.
fn build_stmt(pair: Pair<Rule>) -> Result<Stmt, ParseError> {
    // No need to capture span here as it's handled in expr_stmt
    println!("Building stmt from rule: {:?}", pair.as_rule());
    let inner = pair.into_inner().next().ok_or(ParseError::Unknown)?;
    println!("Stmt inner rule: {:?}", inner.as_rule());
    match inner.as_rule() {
        Rule::expr_stmt => {
            println!("Found expr_stmt rule");
            let expr_pair = inner.into_inner().next().ok_or(ParseError::Unknown)?;
            println!("Expr rule: {:?}", expr_pair.as_rule());
            let expr = build_expr(expr_pair)?;
            println!("Built expr: {:?}", expr);
            Ok(Stmt::Expr(expr))
        },
        _ => {
            println!("Unhandled stmt rule: {:?}", inner.as_rule());
            Err(ParseError::UnexpectedRule(inner.as_rule()))
        }
    }
}

/// Build an expression from a Pest pair.
fn build_expr(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let span = get_span(&pair);
    println!("Building expr from rule: {:?}", pair.as_rule());
    match pair.as_rule() {
        Rule::expr => {
            let inner = pair.into_inner().next().ok_or(ParseError::Unknown)?;
            println!("Expr inner rule: {:?}", inner.as_rule());
            build_expr(inner)
        }
        Rule::call_expr => {
            println!("Found call_expr rule");
            build_call_expr(pair)
        },
        Rule::string_literal => {
            println!("Found string_literal rule: {}", pair.as_str());
            // Remove the quotes from the string literal
            let s = pair.as_str();
            let content = &s[1..s.len()-1];
            Ok(Expr::Literal(Literal::String(content.to_string()), span))
        },
        Rule::identifier => {
            println!("Found identifier rule: {}", pair.as_str());
            Ok(Expr::Variable(pair.as_str().to_string(), span))
        },
        _ => {
            println!("Unhandled expr rule: {:?}", pair.as_rule());
            Err(ParseError::UnexpectedRule(pair.as_rule()))
        },
    }
}

/// Build a function call expression from a Pest pair.
fn build_call_expr(pair: Pair<Rule>) -> Result<Expr, ParseError> {
    let span = get_span(&pair);
    let mut inner = pair.into_inner();
    let func_name = inner.next().ok_or(ParseError::Unknown)?;
    let func = Expr::Variable(func_name.as_str().to_string(), get_span(&func_name));
    let mut args = Vec::new();
    for arg in inner {
        args.push(build_expr(arg)?);
    }
    Ok(Expr::Call {
        func: Box::new(func),
        args,
        span,
    })
}

/// Utility: get span from pest Pair
fn get_span(pair: &Pair<Rule>) -> Span {
    let span = pair.as_span();
    Span {
        start: span.start(),
        end: span.end(),
    }
}

/// From trait implementation for converting Pest errors to our ParseError.
impl From<pest::error::Error<Rule>> for ParseError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        ParseError::PestError(Box::new(error))
    }
}

/// Display implementation for ParseError
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::PestError(e) => write!(f, "Parse error: {}", e),
            ParseError::UnexpectedRule(rule) => write!(f, "Parse error: Unexpected rule: {:?}", rule),
            ParseError::Unknown => write!(f, "Parse error: Unknown parse error"),
            ParseError::Custom(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

/// Parser for HighRust generated by Pest.
#[derive(Parser)]
#[grammar = "src/parser.pest"]
struct HighRustParser;
