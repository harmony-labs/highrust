//! Tests for automatic string conversion in the HighRust transpiler.
//!
//! These tests verify that the transpiler automatically inserts appropriate
//! .to_string() conversions where needed.

use highrust_transpiler::{
    ast::{Block, Expr, FunctionDef, Literal, Module, ModuleItem, Pattern, Span, Stmt, Type},
    lowering::lower_module,
    codegen::{generate_rust_code, CodegenContext},
    ownership::OwnershipInference,
};

/// Helper function to create a span for testing.
fn test_span() -> Span {
    Span { start: 0, end: 0 }
}

#[test]
fn test_string_literal_to_string_conversion() {
    // Create AST for a function that needs string conversion:
    // fn test_string_conversion() {
    //     let s: String = "hello";  // Should convert to "hello".to_string()
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_string_conversion".to_string(),
        params: vec![],
        ret_type: None,
        body: Block {
            stmts: vec![
                Stmt::Let {
                    pattern: Pattern::Variable("s".to_string(), span.clone()),
                    value: Expr::Literal(Literal::String("hello".to_string()), span.clone()),
                    ty: Some(Type::Named("String".to_string(), vec![])),
                    span: span.clone(),
                },
            ],
            span: span.clone(),
        },
        is_async: false,
        is_rust: false,
        span,
    };
    
    // Create a module with our function
    let module = Module {
        items: vec![ModuleItem::Function(func)],
        span: test_span(),
    };
    
    // Run ownership inference
    let ownership_inference = OwnershipInference::new();
    let analysis_result = ownership_inference.analyze_module(&module);
    
    // Lower the AST to IR
    let lowered = lower_module(&module).unwrap();
    
    // Generate Rust code with the ownership analysis
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // Verify that the generated code includes .to_string()
    assert!(code.contains(".to_string()"), 
            "Generated code should include .to_string() conversion, but got: {}", code);
}

#[test]
fn test_string_concat_conversion() {
    // Create AST for a function that concatenates strings:
    // fn test_concat() {
    //     let name = "World";  
    //     let greeting = "Hello, " + name;  // Should convert both sides
    // }
    
    let span = test_span();
    
    // Create a module with our function
    let module = Module {
        items: vec![
            ModuleItem::Function(FunctionDef {
                name: "test_concat".to_string(),
                params: vec![],
                ret_type: None,
                body: Block {
                    stmts: vec![
                        // let name = "World";
                        Stmt::Let {
                            pattern: Pattern::Variable("name".to_string(), span.clone()),
                            value: Expr::Literal(Literal::String("World".to_string()), span.clone()),
                            ty: None,
                            span: span.clone(),
                        },
                        // let greeting = "Hello, " + name;
                        Stmt::Let {
                            pattern: Pattern::Variable("greeting".to_string(), span.clone()),
                            // In our AST, binary operations are represented as function calls
                            value: Expr::Call {
                                func: Box::new(Expr::Variable("+".to_string(), span.clone())),
                                args: vec![
                                    Expr::Literal(Literal::String("Hello, ".to_string()), span.clone()),
                                    Expr::Variable("name".to_string(), span.clone())
                                ],
                                span: span.clone(),
                            },
                            ty: None,
                            span: span.clone(),
                        },
                    ],
                    span: span.clone(),
                },
                is_async: false,
                is_rust: false,
                span,
            })
        ],
        span: test_span(),
    };
    
    // Run ownership inference
    let ownership_inference = OwnershipInference::new();
    let analysis_result = ownership_inference.analyze_module(&module);
    
    // Lower the AST to IR
    let lowered = lower_module(&module).unwrap();
    
    // Generate Rust code with the ownership analysis
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // Verify that the generated code includes .to_string() for string concatenation
    assert!(code.contains(".to_string()"), 
            "Generated code should include .to_string() conversion for string concatenation, but got: {}", code);
}