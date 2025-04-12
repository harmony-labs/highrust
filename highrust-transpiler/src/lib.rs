pub mod parser;
pub mod ast;
pub mod lowering;
pub mod codegen;

#[cfg(test)]
mod tests {
    use super::ast::*;
    use super::lowering::*;

    #[test]
    fn lowering_entry_points_compile() {
        // Minimal dummy AST nodes
        let span = Span { start: 0, end: 0 };
        let module = Module { items: vec![], span: span.clone() };
        let func = FunctionDef {
            name: "f".to_string(),
            params: vec![],
            ret_type: None,
            body: Block { stmts: vec![], span: span.clone() },
            is_async: false,
            is_rust: false,
            span: span.clone(),
        };
        let stmt = Stmt::Expr(Expr::Literal(Literal::Int(0), span.clone()));
        let expr = Expr::Literal(Literal::Int(0), span);

        // Call lowering functions
        let _ = lower_module(&module);
        let _ = lower_function(&func);
        let _ = lower_stmt(&stmt);
        let _ = lower_expr(&expr);
    }
}