//! Unit tests for the HighRust transpiler components
//!
//! These tests focus on testing individual components of the transpiler,
//! such as the parser, AST, lowering, and code generation.

use highrust_transpiler::{
    ast::{Module, Span, FunctionDef, Block, Stmt, Expr, Literal},
    parser::parse,
    lowering::{lower_module, lower_function, lower_stmt, lower_expr},
    codegen::{CodegenContext, LoweredIr},
};

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
    
    // Call lowering functions and ensure they return something
    let _lowered_module = lower_module(&module);
    let _lowered_function = lower_function(&func);
    let _lowered_stmt = lower_stmt(&stmt);
    let _lowered_expr = lower_expr(&expr);
    
    // Currently nothing to assert beyond they don't panic,
    // as the lowering functions return stub implementations
}

#[test]
fn test_codegen_context() {
    // Test that we can create a codegen context
    let ctx = CodegenContext::new();
    
    // Create a stubbed IR and generate code
    let ir = LoweredIr {};
    let code = highrust_transpiler::codegen::generate_rust_code(&ir, &ctx);
    
    // Currently this returns an empty string
    assert_eq!(code, "");
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