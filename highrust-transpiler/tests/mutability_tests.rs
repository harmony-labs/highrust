//! Unit tests for mutability inference in the HighRust transpiler.
//!
//! These tests verify that the ownership inference system correctly identifies
//! when variables need to be marked as mutable based on their usage patterns.

use highrust_transpiler::{
    ast::{Block, Expr, FunctionDef, Literal, Module, ModuleItem, Param, Pattern, Span, Stmt},
    ownership::{OwnershipInference, MutabilityRequirement},
    lowering::lower_module,
    codegen::{generate_rust_code, CodegenContext},
};

/// Helper function to create a span for testing.
fn test_span() -> Span {
    Span { start: 0, end: 0 }
}

#[test]
fn test_variable_reassignment_mutability() {
    // Create AST for a function with variable reassignment:
    // fn test() {
    //     let x = 1;
    //     x = 2;  // requires x to be mutable
    // }
    
    // In our case, we simulate variable reassignment through field access and method call
    // because direct assignment expressions aren't yet implemented
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_reassign".to_string(),
        params: vec![],
        ret_type: None,
        body: Block {
            stmts: vec![
                // let x = 1;
                Stmt::Let {
                    pattern: Pattern::Variable("x".to_string(), span.clone()),
                    value: Expr::Literal(Literal::Int(1), span.clone()),
                    ty: None,
                    span: span.clone(),
                },
                // simulate x = 2 with a method call that implies mutation
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::FieldAccess {
                        base: Box::new(Expr::Variable("x".to_string(), span.clone())),
                        field: "set".to_string(), // "set" should trigger mutability inference
                        span: span.clone(),
                    }),
                    args: vec![Expr::Literal(Literal::Int(2), span.clone())],
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
    
    // Verify "x" was identified as mutable
    assert!(analysis_result.mutable_vars.contains("x"), 
            "Variable 'x' should be identified as mutable");
    
    // Additionally, verify the generated code
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // Check that the code contains "let mut x"
    assert!(code.contains("let mut x"), 
            "Generated code should include 'let mut x', but got: {}", code);
}

#[test]
fn test_method_call_mutability() {
    // Create AST for a function with a method call that mutates:
    // fn test() {
    //     let v = Vec::new();
    //     v.push(1);  // requires v to be mutable
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_method_mutation".to_string(),
        params: vec![],
        ret_type: None,
        body: Block {
            stmts: vec![
                // let v = Vec::new();
                Stmt::Let {
                    pattern: Pattern::Variable("v".to_string(), span.clone()),
                    value: Expr::Call {
                        func: Box::new(Expr::FieldAccess {
                            base: Box::new(Expr::Variable("Vec".to_string(), span.clone())),
                            field: "new".to_string(),
                            span: span.clone(),
                        }),
                        args: vec![],
                        span: span.clone(),
                    },
                    ty: None,
                    span: span.clone(),
                },
                // v.push(1);
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::FieldAccess {
                        base: Box::new(Expr::Variable("v".to_string(), span.clone())),
                        field: "push".to_string(), // "push" should trigger mutability inference
                        span: span.clone(),
                    }),
                    args: vec![Expr::Literal(Literal::Int(1), span.clone())],
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
    
    // Verify "v" was identified as mutable
    assert!(analysis_result.mutable_vars.contains("v"), 
            "Variable 'v' should be identified as mutable");
    
    // Additionally, verify the generated code
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // Check that the code contains "let mut v"
    assert!(code.contains("let mut v"), 
            "Generated code should include 'let mut v', but got: {}", code);
}

#[test]
fn test_branch_mutability() {
    // Create AST for a function with mutation in a conditional branch:
    // fn test(cond: bool) {
    //     let x = 1;
    //     if cond {
    //         x = 2;  // requires x to be mutable even though this is conditional
    //     }
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_branch_mutation".to_string(),
        params: vec![
            Param {
                name: "cond".to_string(),
                ty: None,
                span: span.clone(),
            }
        ],
        ret_type: None,
        body: Block {
            stmts: vec![
                // let x = 1;
                Stmt::Let {
                    pattern: Pattern::Variable("x".to_string(), span.clone()),
                    value: Expr::Literal(Literal::Int(1), span.clone()),
                    ty: None,
                    span: span.clone(),
                },
                // if cond { ... }
                Stmt::If {
                    cond: Expr::Variable("cond".to_string(), span.clone()),
                    then_branch: Block {
                        stmts: vec![
                            // Simulate x = 2 with a method call that implies mutation
                            Stmt::Expr(Expr::Call {
                                func: Box::new(Expr::FieldAccess {
                                    base: Box::new(Expr::Variable("x".to_string(), span.clone())),
                                    field: "set".to_string(), // "set" should trigger mutability inference
                                    span: span.clone(),
                                }),
                                args: vec![Expr::Literal(Literal::Int(2), span.clone())],
                                span: span.clone(),
                            }),
                        ],
                        span: span.clone(),
                    },
                    else_branch: None,
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
    
    // Verify "x" was identified as mutable
    assert!(analysis_result.mutable_vars.contains("x"), 
            "Variable 'x' should be identified as mutable even though it's only modified in a branch");
}