//! Tests for automatic .clone() insertion in HighRust.

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
fn test_clone_on_move() {
    // fn test_clone() {
    //     let s = "hello".to_string();
    //     let t = s;
    //     let u = s; // s is moved, so this should insert .clone()
    // }
    let span = test_span();
    let func = FunctionDef {
        name: "test_clone".to_string(),
        params: vec![],
        ret_type: None,
        body: Block {
            stmts: vec![
                Stmt::Let {
                    pattern: Pattern::Variable("s".to_string(), span.clone()),
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
                Stmt::Let {
                    pattern: Pattern::Variable("t".to_string(), span.clone()),
                    value: Expr::Variable("s".to_string(), span.clone()),
                    ty: Some(Type::Named("String".to_string(), vec![])),
                    span: span.clone(),
                },
                Stmt::Let {
                    pattern: Pattern::Variable("u".to_string(), span.clone()),
                    value: Expr::Variable("s".to_string(), span.clone()),
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
    let module = Module {
        items: vec![ModuleItem::Function(func)],
        span: test_span(),
    };
    let ownership_inference = OwnershipInference::new();
    let analysis_result = ownership_inference.analyze_module(&module);
    let lowered = lower_module(&module).unwrap();
    let mut ctx = CodegenContext::with_analysis(analysis_result);
    let code = generate_rust_code(&lowered, &mut ctx).unwrap();
    // The second use of s should be s.clone()
    assert!(code.contains("let u: String = s.clone();"), "Generated code should insert .clone(): {}", code);
}