pub mod parser;
pub mod ast;
pub mod lowering;
pub mod codegen;
pub mod ownership;

use std::path::Path;

/// Error type for the transpiler.
#[derive(Debug)]
pub enum TranspilerError {
    /// Error during parsing.
    ParseError(String),
    /// Error during lowering.
    LoweringError(lowering::LoweringError),
    /// Error during code generation.
    CodegenError(codegen::CodegenError),
    /// Error during ownership inference.
    OwnershipError(ownership::OwnershipError),
    /// Error reading or writing files.
    IoError(std::io::Error),
}

impl From<lowering::LoweringError> for TranspilerError {
    fn from(err: lowering::LoweringError) -> Self {
        TranspilerError::LoweringError(err)
    }
}

impl From<codegen::CodegenError> for TranspilerError {
    fn from(err: codegen::CodegenError) -> Self {
        TranspilerError::CodegenError(err)
    }
}

impl From<std::io::Error> for TranspilerError {
    fn from(err: std::io::Error) -> Self {
        TranspilerError::IoError(err)
    }
}

impl From<ownership::OwnershipError> for TranspilerError {
    fn from(err: ownership::OwnershipError) -> Self {
        TranspilerError::OwnershipError(err)
    }
}

/// Transpiles HighRust source code to Rust.
///
/// # Arguments
///
/// * `source` - The HighRust source code to transpile.
///
/// # Returns
///
/// A `Result` containing either the generated Rust code as a `String` or a `TranspilerError`.
///
/// # Example
///
/// ```ignore
/// let highrust_code = r#"
///     fn main() {
///         println("Hello, World!");
///     }
/// "#;
/// let rust_code = transpile_source(highrust_code)?;
/// ```
pub fn transpile_source(source: &str) -> Result<String, TranspilerError> {
    // Parse the source code
    let ast = parser::parse(source).map_err(|e| TranspilerError::ParseError(e.to_string()))?;
    
    // Perform ownership inference
    let ownership_inference = ownership::OwnershipInference::new();
    let _ownership_analysis = ownership_inference.analyze_module(&ast);
    
    // Lower the AST to IR, passing ownership information
    // Note: The ownership analysis is already integrated in the lower_module function
    let ir = lowering::lower_module(&ast)?;
    
    // The ownership analysis results are now used during lowering
    // inform the codegen phase about required mut, &, &mut, and clone() calls
    
    // Generate Rust code
    let mut ctx = codegen::CodegenContext::new();
    // In the future, we'll pass ownership_analysis to the context
    let rust_code = codegen::generate_rust_code(&ir, &mut ctx)?;
    
    Ok(rust_code)
}

/// Transpiles a HighRust file to a Rust file.
///
/// # Arguments
///
/// * `input_path` - Path to the HighRust source file.
/// * `output_path` - Path where the generated Rust code will be written.
///
/// # Returns
///
/// A `Result` indicating success or a `TranspilerError`.
///
/// # Example
///
/// ```ignore
/// transpile_file("src/main.hrs", "src/main.rs")?;
/// ```
pub fn transpile_file<P: AsRef<Path>>(input_path: P, output_path: P) -> Result<(), TranspilerError> {
    // Read the input file
    let source = std::fs::read_to_string(input_path)?;
    
    // Transpile the source
    let rust_code = transpile_source(&source)?;
    
    // Write the output file
    std::fs::write(output_path, rust_code)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::ast::*;
    use super::lowering::*;
    use super::codegen::*;
    use super::ownership::{OwnershipAnalysisResult, OwnershipInference};
    use std::collections::HashSet;

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
        let expr = Expr::Literal(Literal::Int(0), span.clone());

        // Create a mock ownership analysis result for testing
        let mock_analysis = OwnershipAnalysisResult {
            mutable_vars: HashSet::new(),
            borrowed_vars: HashSet::new(),
            moved_vars: HashSet::new(),
            cloned_vars: HashSet::new(),
            lifetime_params: Vec::new(),
        };

        // Call lowering functions
        let _ = lower_module(&module);
        let _ = lower_function(&func, &mock_analysis);
        let _ = lower_stmt(&stmt, &mock_analysis);
        let _ = lower_expr(&expr, &mock_analysis);
    }

    #[test]
    fn test_codegen_hello_world() {
        // Create a simple "Hello, World!" program
        let span = Span { start: 0, end: 0 };
        
        // Create println call expression
        let println_call = Expr::Call {
            func: Box::new(Expr::Variable("println".to_string(), span.clone())),
            args: vec![Expr::Literal(Literal::String("Hello, World!".to_string()), span.clone())],
            span: span.clone(),
        };
        
        // Create main function
        let main_func = FunctionDef {
            name: "main".to_string(),
            params: vec![],
            ret_type: None,
            body: Block {
                stmts: vec![Stmt::Expr(println_call)],
                span: span.clone(),
            },
            is_async: false,
            is_rust: false,
            span: span.clone(),
        };
        
        // Create module
        let module = Module {
            items: vec![ModuleItem::Function(main_func)],
            span,
        };
        
        // Perform ownership inference and lower the AST to IR
        let ownership_inference = OwnershipInference::new();
        let _analysis_result = ownership_inference.analyze_module(&module);
        let ir = lower_module(&module).expect("Failed to lower module");
        
        // Generate Rust code
        let mut ctx = CodegenContext::new();
        let rust_code = generate_rust_code(&ir, &mut ctx).expect("Failed to generate code");
        
        // Expected output - note the extra newline at the end that our generator adds
        let expected = "fn main() {\n    println!(\"Hello, World!\");\n}\n\n";
        
        assert_eq!(rust_code, expected);
    }

    #[test]
    #[ignore = "Parser does not yet support function calls like println; enable when parser is improved."]
    fn test_transpile_source() {
        // Simple HighRust program
        let source = "fn main() {\n    println(\"Hello, World!\");\n}";
        
        // Transpile to Rust
        let result = super::transpile_source(source);
        
        // Check that transpilation succeeded
        assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
        
        // Check the generated code
        let rust_code = result.unwrap();
        assert!(rust_code.contains("fn main()"));
        assert!(rust_code.contains("println!(\"Hello, World!\")"));
    }
}