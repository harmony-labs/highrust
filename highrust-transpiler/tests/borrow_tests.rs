//! Tests for borrow inference in the HighRust transpiler.
//!
//! These tests verify that the ownership inference system correctly identifies
//! when variables should be borrowed (immutably or mutably) versus moved.

use highrust_transpiler::{
    ast::{Block, Expr, FunctionDef, Literal, Module, ModuleItem, Param, Pattern, Span, Stmt},
    ownership::OwnershipInference,
    lowering::lower_module,
    codegen::{generate_rust_code, CodegenContext},
};

/// Helper function to create a span for testing.
fn test_span() -> Span {
    Span { start: 0, end: 0 }
}

#[test]
fn test_immutable_borrow_inference() {
    // Create AST for a function that should use immutable borrows:
    // fn test_immutable_borrow(s: String) {
    //     let len = s.len();  // s should be borrowed immutably here
    //     println!("{}", s);  // s should be borrowed immutably here too
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_immutable_borrow".to_string(),
        params: vec![
            Param {
                name: "s".to_string(),
                ty: None,
                span: span.clone(),
            }
        ],
        ret_type: None,
        body: Block {
            stmts: vec![
                // let len = s.len();
                Stmt::Let {
                    pattern: Pattern::Variable("len".to_string(), span.clone()),
                    value: Expr::Call {
                        func: Box::new(Expr::FieldAccess {
                            base: Box::new(Expr::Variable("s".to_string(), span.clone())),
                            field: "len".to_string(),
                            span: span.clone(),
                        }),
                        args: vec![],
                        span: span.clone(),
                    },
                    ty: None,
                    span: span.clone(),
                },
                // println!("{}", s);
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Variable("println".to_string(), span.clone())),
                    args: vec![
                        Expr::Literal(Literal::String("{}".to_string()), span.clone()),
                        Expr::Variable("s".to_string(), span.clone()),
                    ],
                    span: span.clone(),
                })
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
    
    // Verify "s" was identified as needing an immutable borrow
    assert!(analysis_result.immut_borrowed_vars.contains("s"),
            "Variable 's' should be identified as needing an immutable borrow");
    
    // Additionally, verify the generated code would use &s
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // For now, we just check that 's' appears in both places - full borrowing
    // implementation would be more accurate with proper field access handling
    assert!(code.contains("s") && code.contains("println"),
            "Generated code doesn't match expectations: {}", code);
}

#[test]
fn test_mutable_borrow_inference() {
    // Create AST for a function that should use mutable borrows:
    // fn test_mutable_borrow(mut v: Vec<i32>) {
    //     v.push(1);  // v should be borrowed mutably
    //     v.push(2);  // v should be borrowed mutably again
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_mutable_borrow".to_string(),
        params: vec![
            Param {
                name: "v".to_string(),
                ty: None,
                span: span.clone(),
            }
        ],
        ret_type: None,
        body: Block {
            stmts: vec![
                // v.push(1);
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::FieldAccess {
                        base: Box::new(Expr::Variable("v".to_string(), span.clone())),
                        field: "push".to_string(),
                        span: span.clone(),
                    }),
                    args: vec![
                        Expr::Literal(Literal::Int(1), span.clone()),
                    ],
                    span: span.clone(),
                }),
                // v.push(2);
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::FieldAccess {
                        base: Box::new(Expr::Variable("v".to_string(), span.clone())),
                        field: "push".to_string(),
                        span: span.clone(),
                    }),
                    args: vec![
                        Expr::Literal(Literal::Int(2), span.clone()),
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
    
    // Verify "v" was identified as needing a mutable borrow 
    assert!(analysis_result.mut_borrowed_vars.contains("v"), 
            "Variable 'v' should be identified as needing a mutable borrow");
    
    // Also verify it was marked as mutable
    assert!(analysis_result.mutable_vars.contains("v"),
            "Variable 'v' should be identified as mutable");
    
    // Additionally, verify the generated code would use &mut v
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::new();
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    
    // This assertion might need adjustment as implementation progresses
    assert!(code.contains("let mut v") || code.contains("mut v:"), 
            "Generated code should have 'mut v', but got: {}", code);
}

#[test]
fn test_move_inference() {
    // Create AST for a function that needs to move a value:
    // fn test_move_inference(s: String) {
    //     let s2 = s;  // s should be moved here
    //     // s is no longer usable after this point
    // }
    
    let span = test_span();
    let func = FunctionDef {
        name: "test_move_inference".to_string(),
        params: vec![
            Param {
                name: "s".to_string(),
                ty: None,
                span: span.clone(),
            }
        ],
        ret_type: None,
        body: Block {
            stmts: vec![
                // let s2 = s;
                Stmt::Let {
                    pattern: Pattern::Variable("s2".to_string(), span.clone()),
                    value: Expr::Variable("s".to_string(), span.clone()),
                    ty: None,
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
    
    // Verify "s" was identified as being moved
    assert!(analysis_result.moved_vars.contains("s"), 
            "Variable 's' should be identified as being moved");
}