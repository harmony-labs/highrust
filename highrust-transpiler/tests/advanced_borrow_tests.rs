//! Advanced borrow inference tests for the HighRust transpiler.
//!
//! These tests verify that the ownership inference system correctly handles more
//! complex borrowing and ownership patterns.

use highrust_transpiler::{
    ast::{Block, Expr, FunctionDef, Literal, Module, ModuleItem, Pattern, Span, Stmt, Type, Param},
    ownership::OwnershipInference,
    lowering::lower_module,
    codegen::{generate_rust_code, CodegenContext},
};

/// Helper function to create a span for testing.
fn test_span() -> Span {
    Span { start: 0, end: 0 }
}

#[test]
fn test_nested_borrow_inference() {
    // Create AST for a function that uses nested borrows:
    // fn test_nested_borrows() {
    //     let data = vec![1, 2, 3];
    //     let view = &data;            // immutable borrow
    //     let first = &view[0];        // nested immutable borrow
    //     println!("{}", first);
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_nested_borrows".to_string(),
        params: vec![],
        ret_type: None,
        body: Block {
            stmts: vec![
                // let data = vec![1, 2, 3];
                Stmt::Let {
                    pattern: Pattern::Variable("data".to_string(), span.clone()),
                    value: Expr::Call {
                        func: Box::new(Expr::Variable("vec".to_string(), span.clone())),
                        args: vec![
                            Expr::Literal(Literal::Int(1), span.clone()),
                            Expr::Literal(Literal::Int(2), span.clone()),
                            Expr::Literal(Literal::Int(3), span.clone()),
                        ],
                        span: span.clone(),
                    },
                    ty: None,
                    span: span.clone(),
                },
                
                // let view = &data;
                Stmt::Let {
                    pattern: Pattern::Variable("view".to_string(), span.clone()),
                    value: Expr::Call {
                        func: Box::new(Expr::Variable("ref".to_string(), span.clone())),
                        args: vec![Expr::Variable("data".to_string(), span.clone())],
                        span: span.clone(),
                    },
                    ty: None,
                    span: span.clone(),
                },
                
                // let first = &view[0];
                Stmt::Let {
                    pattern: Pattern::Variable("first".to_string(), span.clone()),
                    value: Expr::Call {
                        func: Box::new(Expr::Variable("ref".to_string(), span.clone())),
                        args: vec![
                            Expr::Call {
                                func: Box::new(Expr::Variable("index".to_string(), span.clone())),
                                args: vec![
                                    Expr::Variable("view".to_string(), span.clone()),
                                    Expr::Literal(Literal::Int(0), span.clone()),
                                ],
                                span: span.clone(),
                            },
                        ],
                        span: span.clone(),
                    },
                    ty: None,
                    span: span.clone(),
                },
                
                // println!("{}", first);
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Variable("println".to_string(), span.clone())),
                    args: vec![
                        Expr::Literal(Literal::String("{}".to_string()), span.clone()),
                        Expr::Variable("first".to_string(), span.clone()),
                    ],
                    span: span.clone(),
                }),
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
    
    // Verify "view" and "first" were identified as immutable borrows
    assert!(analysis_result.immut_borrowed_vars.contains("view"),
            "Variable 'view' should be identified as an immutable borrow");
    assert!(analysis_result.immut_borrowed_vars.contains("first"),
            "Variable 'first' should be identified as an immutable borrow");
    
    // Additionally, verify the generated code has appropriate references
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // Check that the code contains "&data" and refer to "view" and "first" correctly
    assert!(code.contains("&data"), "Generated code should have reference to 'data'");
}

#[test]
fn test_temporary_borrow_inference() {
    // Create AST for a function that uses temporary borrows:
    // fn test_temporary_borrow() {
    //     let data = "hello".to_string();
    //     process(&data);             // temporary borrow
    //     data.push_str(" world");    // data can be modified after borrow ends
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_temporary_borrow".to_string(),
        params: vec![],
        ret_type: None,
        body: Block {
            stmts: vec![
                // let data = "hello".to_string();
                Stmt::Let {
                    pattern: Pattern::Variable("data".to_string(), span.clone()),
                    value: Expr::Call {
                        func: Box::new(Expr::FieldAccess {
                            base: Box::new(Expr::Literal(Literal::String("hello".to_string()), span.clone())),
                            field: "to_string".to_string(),
                            span: span.clone(),
                        }),
                        args: vec![],
                        span: span.clone(),
                    },
                    ty: Some(Type::Named("String".to_string(), vec![])),
                    span: span.clone(),
                },
                
                // process(&data);
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Variable("process".to_string(), span.clone())),
                    args: vec![
                        Expr::Call {
                            func: Box::new(Expr::Variable("ref".to_string(), span.clone())),
                            args: vec![Expr::Variable("data".to_string(), span.clone())],
                            span: span.clone(),
                        },
                    ],
                    span: span.clone(),
                }),
                
                // data.push_str(" world");
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::FieldAccess {
                        base: Box::new(Expr::Variable("data".to_string(), span.clone())),
                        field: "push_str".to_string(),
                        span: span.clone(),
                    }),
                    args: vec![
                        Expr::Literal(Literal::String(" world".to_string()), span.clone()),
                    ],
                    span: span.clone(),
                }),
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
    
    // Verify "data" is mutable (for push_str)
    assert!(analysis_result.mutable_vars.contains("data"),
            "Variable 'data' should be identified as mutable");
    
    // Verify "data" was temporarily borrowed
    assert!(analysis_result.immut_borrowed_vars.contains("data"),
            "Variable 'data' should be identified as having an immutable borrow");
    
    // Generate code and verify
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // Check that the code contains "let mut data" and "&data"
    assert!(code.contains("let mut data"), "Generated code should mark 'data' as mutable");
    assert!(code.contains("&data"), "Generated code should have reference to 'data'");
}