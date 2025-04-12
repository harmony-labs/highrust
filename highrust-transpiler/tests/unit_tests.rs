//! Unit tests for the HighRust transpiler components
//!
//! These tests focus on testing individual components of the transpiler,
//! such as the parser, AST, lowering, and code generation.

use highrust_transpiler::{
    ast::{Module, Span, FunctionDef, Block, Stmt, Expr, Literal},
    parser::parse,
    lowering::{lower_module, lower_function, lower_stmt, lower_expr, LoweredModule},
    codegen::CodegenContext,
    ownership::OwnershipAnalysisResult,
};
use std::collections::{HashMap, HashSet};

#[test]
fn test_parser_minimal() {
    // Test basic parsing with the minimal grammar
    // This will evolve as the parser is implemented
    let result = parse("// This is a comment");
    assert!(result.is_ok());
}

#[test]
fn test_ast_construction() {
    // Test that we can create AST nodes correctly
    let span = Span { start: 0, end: 0 };
    
    // Create a simple module
    let module = Module { 
        items: vec![],
        span: span.clone(),
    };
    
    assert_eq!(module.items.len(), 0);
    assert_eq!(module.span.start, 0);
    assert_eq!(module.span.end, 0);
    
    // Create a simple function
    let func = FunctionDef {
        name: "test_function".to_string(),
        params: vec![],
        ret_type: None,
        body: Block { stmts: vec![], span: span.clone() },
        is_async: false,
        is_rust: false,
        span: span.clone(),
    };
    
    assert_eq!(func.name, "test_function");
    assert_eq!(func.params.len(), 0);
    assert!(!func.is_async);
    assert!(!func.is_rust);
}

#[test]
fn test_lowering_entry_points() {
    // Test that the lowering functions don't panic with simple input
    let span = Span { start: 0, end: 0 };
    
    let module = Module {
        items: vec![],
        span: span.clone(),
    };
    
    let func = FunctionDef {
        name: "test_function".to_string(),
        params: vec![],
        ret_type: None,
        body: Block { stmts: vec![], span: span.clone() },
        is_async: false,
        is_rust: false,
        span: span.clone(),
    };
    
    let stmt = Stmt::Expr(Expr::Literal(Literal::Int(42), span.clone()));
    let expr = Expr::Literal(Literal::Int(42), span);
    
    // Create a mock ownership analysis result for testing
    let mock_analysis = OwnershipAnalysisResult {
        mutable_vars: HashSet::new(),
        immut_borrowed_vars: HashSet::new(),
        mut_borrowed_vars: HashSet::new(),
        moved_vars: HashSet::new(),
        cloned_vars: HashSet::new(),
        lifetime_params: Vec::new(),
        borrow_graph: HashMap::new(),
        string_converted_vars: HashSet::new(),
        string_converted_exprs: HashSet::new(),
    };
    
    // Call lowering functions and ensure they return something
    let _lowered_module = lower_module(&module);
    let _lowered_function = lower_function(&func, &mock_analysis);
    let _lowered_stmt = lower_stmt(&stmt, &mock_analysis);
    let _lowered_expr = lower_expr(&expr, &mock_analysis);
    
    // Currently nothing to assert beyond they don't panic,
    // as the lowering functions return stub implementations
}

#[test]
fn test_codegen_context() {
    // Test that we can create a codegen context
    let mut ctx = CodegenContext::new();
    
    // Create a minimal lowered module
    let module = LoweredModule { items: vec![] };
    
    // Generate code
    let result = highrust_transpiler::codegen::generate_rust_code(&module, &mut ctx);
    
    // Check that code generation succeeds
    assert!(result.is_ok());
    
    // Check that the result is an empty string (since our module is empty)
    assert_eq!(result.unwrap(), "");
}

#[test]
fn test_end_to_end_minimal() {
    // A very minimal end-to-end test
    let source = "// HighRust test code";
    
    // Parse the source
    let parse_result = parse(source);
    assert!(parse_result.is_ok());
    
    // Currently the parse function doesn't actually return an AST,
    // so we can't fully test the pipeline yet
}