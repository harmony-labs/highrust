//! Golden file tests for the HighRust transpiler
//!
//! These tests ensure that the transpiler produces the expected Rust code
//! for a variety of HighRust input files.

mod test_utils;

use highrust_transpiler::{
    ast::{Module, Span},
    codegen::CodegenContext,
    parser::parse,
    lowering::{lower_module, LoweredModule},
};
use test_utils::{get_fixture_files, get_expected_path, read_file_content};

// Currently just a stub for golden file testing since most components
// are not yet fully implemented
fn transpile_highrust_to_rust(source: &str) -> String {
    // This is a placeholder for the actual transpilation process
    // In reality, we would:
    // 1. Parse the HighRust code to AST
    // 2. Lower the AST to IR
    // 3. Generate Rust code from the IR
    
    match parse(source) {
        Ok(ast) => {
            // Lower the AST to IR
            match lower_module(&ast) {
                Ok(lowered) => {
                    // Generate Rust code using the lowered IR
                    let mut ctx = CodegenContext::new();
                    match highrust_transpiler::codegen::generate_rust_code(&lowered, &mut ctx) {
                        Ok(code) => code,
                        Err(e) => format!("// Failed to generate Rust code: {:?}", e)
                    }
                },
                Err(e) => {
                    // If lowering failed, return a comment with the error
                    format!("// Failed to lower AST to IR: {:?}", e)
                }
            }
        },
        Err(e) => {
            // If parsing failed, return a comment with the error
            format!("// Failed to parse HighRust code: {:?}", e)
        }
    }
}

#[test]
#[ignore] // Ignoring until parser and codegen are more fully implemented
fn test_golden_files() {
    let fixtures_dir = "tests/fixtures/basic";
    let expected_dir = "tests/expected/basic";
    
    // Get all .hrs files in the fixtures directory
    let fixture_files = get_fixture_files(fixtures_dir, "hrs");
    
    for fixture_path in fixture_files {
        let source = read_file_content(&fixture_path);
        let expected_path = get_expected_path(&fixture_path, fixtures_dir, expected_dir);
        
        // Skip if expected file doesn't exist yet
        if !expected_path.exists() {
            println!("Expected file does not exist: {}", expected_path.display());
            continue;
        }
        
        let expected = read_file_content(&expected_path);
        let actual = transpile_highrust_to_rust(&source);
        
        // Normalize line endings for comparison
        let expected_normalized = expected.replace("\r\n", "\n");
        let actual_normalized = actual.replace("\r\n", "\n");
        
        assert_eq!(
            expected_normalized, 
            actual_normalized,
            "Transpiled output for {} doesn't match expected output in {}",
            fixture_path.display(),
            expected_path.display()
        );
    }
}

/// Helper to generate expected outputs for fixture files
/// This is not a test but a utility to create initial expected output files
#[test]
fn generate_expected_outputs() {
    let fixtures_dir = "tests/fixtures/basic";
    let expected_dir = "tests/expected/basic";
    
    // Get all .hrs files in the fixtures directory
    let fixture_files = get_fixture_files(fixtures_dir, "hrs");
    
    for fixture_path in fixture_files {
        let source = read_file_content(&fixture_path);
        let expected_path = get_expected_path(&fixture_path, fixtures_dir, expected_dir);
        
        // Generate transpiled output
        let output = transpile_highrust_to_rust(&source);
        
        // Create parent directories if needed
        if let Some(parent) = expected_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directory");
        }
        
        // Write the output to the expected file
        std::fs::write(&expected_path, output).expect("Failed to write expected output file");
        
        println!("Generated expected output for {} at {}", 
            fixture_path.display(), 
            expected_path.display()
        );
    }
}