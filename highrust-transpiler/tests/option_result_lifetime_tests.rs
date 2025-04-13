//! Tests for Option/Result mapping and lifetime inference in HighRust.

use highrust_transpiler::{
    ast::{Block, Expr, FunctionDef, Literal, Module, ModuleItem, Pattern, Span, Stmt, Type, Param},
    ownership::OwnershipInference,
    lowering::lower_module,
    codegen::{generate_rust_code, CodegenContext},
};

fn test_span() -> Span {
    Span { start: 0, end: 0 }
}

#[test]
fn test_option_mapping() {
    // fn test_option(x: i32?) -> Option<i32> { x }
    let span = test_span();
    let func = FunctionDef {
        name: "test_option".to_string(),
        params: vec![
            Param {
                name: "x".to_string(),
                ty: Some(Type::Named("Option".to_string(), vec![Type::Named("i32".to_string(), vec![])])),
                span: span.clone(),
            }
        ],
        ret_type: Some(Type::Named("Option".to_string(), vec![Type::Named("i32".to_string(), vec![])])),
        body: Block {
            stmts: vec![
                Stmt::Return(Some(Expr::Variable("x".to_string(), span.clone())), span.clone())
            ],
            span: span.clone(),
        },
        is_async: false,
        is_rust: false,
        span,
    };
    let module = Module {
        items: vec![ModuleItem::Function(func)],
        span: test_span(),
    };
    let ownership_inference = OwnershipInference::new();
    let analysis_result = ownership_inference.analyze_module(&module);
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    assert!(code.contains("Option"), "Generated code should use Option: {}", code);
}

#[test]
fn test_result_mapping() {
    // fn test_result(x: i32) -> Result<i32, String> { if x > 0 { Ok(x) } else { Err(\"bad\") } }
    let span = test_span();
    let func = FunctionDef {
        name: "test_result".to_string(),
        params: vec![
            Param {
                name: "x".to_string(),
                ty: Some(Type::Named("i32".to_string(), vec![])),
                span: span.clone(),
            }
        ],
        ret_type: Some(Type::Named("Result".to_string(), vec![
            Type::Named("i32".to_string(), vec![]),
            Type::Named("String".to_string(), vec![]),
        ])),
        body: Block {
            stmts: vec![
                Stmt::If {
                    cond: Expr::Call {
                        func: Box::new(Expr::Variable(">".to_string(), span.clone())),
                        args: vec![
                            Expr::Variable("x".to_string(), span.clone()),
                            Expr::Literal(Literal::Int(0), span.clone()),
                        ],
                        span: span.clone(),
                    },
                    then_branch: Block {
                        stmts: vec![
                            Stmt::Return(
                                Some(Expr::Call {
                                    func: Box::new(Expr::Variable("Ok".to_string(), span.clone())),
                                    args: vec![Expr::Variable("x".to_string(), span.clone())],
                                    span: span.clone(),
                                }),
                                span.clone()
                            )
                        ],
                        span: span.clone(),
                    },
                    else_branch: Some(Block {
                        stmts: vec![
                            Stmt::Return(
                                Some(Expr::Call {
                                    func: Box::new(Expr::Variable("Err".to_string(), span.clone())),
                                    args: vec![Expr::Literal(Literal::String("bad".to_string()), span.clone())],
                                    span: span.clone(),
                                }),
                                span.clone()
                            )
                        ],
                        span: span.clone(),
                    }),
                    span: span.clone(),
                }
            ],
            span: span.clone(),
        },
        is_async: false,
        is_rust: false,
        span,
    };
    let module = Module {
        items: vec![ModuleItem::Function(func)],
        span: test_span(),
    };
    let ownership_inference = OwnershipInference::new();
    let analysis_result = ownership_inference.analyze_module(&module);
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    assert!(code.contains("Result"), "Generated code should use Result: {}", code);
    assert!(code.contains("Ok(") && code.contains("Err("), "Generated code should use Ok/Err: {}", code);
}

#[test]
fn test_lifetime_inference() {
    // fn get_ref<'a>(x: &'a i32) -> &'a i32 { x }
    let span = test_span();
    let func = FunctionDef {
        name: "get_ref".to_string(),
        params: vec![
            Param {
                name: "x".to_string(),
                ty: Some(Type::Named("&".to_string(), vec![Type::Named("i32".to_string(), vec![])])),
                span: span.clone(),
            }
        ],
        ret_type: Some(Type::Named("&".to_string(), vec![Type::Named("i32".to_string(), vec![])])),
        body: Block {
            stmts: vec![
                Stmt::Return(Some(Expr::Variable("x".to_string(), span.clone())), span.clone())
            ],
            span: span.clone(),
        },
        is_async: false,
        is_rust: false,
        span,
    };
    let module = Module {
        items: vec![ModuleItem::Function(func)],
        span: test_span(),
    };
    let ownership_inference = OwnershipInference::new();
    let analysis_result = ownership_inference.analyze_module(&module);
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    assert!(code.contains("fn get_ref<'a>"), "Generated code should have lifetime parameter: {}", code);
    assert!(code.contains("&'a i32"), "Generated code should use lifetime in type: {}", code);
}