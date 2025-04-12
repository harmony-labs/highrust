//! Integration tests for the HighRust transpiler
//!
//! These tests run the full transpiler pipeline from HighRust code to Rust code,
//! testing the integration between components.

mod test_utils;

use highrust_transpiler::{
    parser::parse,
    // These will be uncommented as the transpiler components are implemented
    // ast::{Module},  
    // lowering::{lower_module}, 
    // codegen::{generate_rust_code, CodegenContext, LoweredIr},
};
use test_utils::{get_fixture_files, get_expected_path, read_file_content};

/// A basic integration test that will eventually run the full transpiler pipeline
#[test]
#[ignore] // Ignoring until the transpiler is more fully implemented
fn test_transpiler_pipeline() {
    let fixtures_dir = "tests/fixtures/basic";
    let expected_dir = "tests/expected/basic";
    
    // Get all .hrs files in the fixtures directory
    let fixture_files = get_fixture_files(fixtures_dir, "hrs");
    
    for fixture_path in fixture_files {
        let source = read_file_content(&fixture_path);
        
        // Step 1: Parse the HighRust code
        let parse_result = parse(&source);
        assert!(parse_result.is_ok(), "Failed to parse fixture: {}", fixture_path.display());
        
        // The remaining steps will be implemented as the transpiler components are built:
        
        // Step 2: Generate AST from parse result
        // Currently parse() doesn't return an AST yet
        
        // Step 3: Lower AST to IR
        // let lowered = lower_module(&ast);
        
        // Step 4: Generate Rust code from IR
        // let ctx = CodegenContext::new();
        // let rust_code = generate_rust_code(&lowered, &ctx);
        
        // Step 5: Compare with expected output
        let expected_path = get_expected_path(&fixture_path, fixtures_dir, expected_dir);
        
        if expected_path.exists() {
            let expected = read_file_content(&expected_path);
            
            // Placeholder for actual result
            // Compare once codegen is implemented:
            // assert_eq!(expected.trim(), rust_code.trim());
            
            // For now, just verify we can read the expected file
            assert!(!expected.is_empty(), "Expected file is empty: {}", expected_path.display());
        } else {
            // Skip if expected file doesn't exist
            println!("Expected file does not exist: {}", expected_path.display());
        }
    }
}

/// Test that fixtures are valid and readable
#[test]
fn test_fixtures_are_valid() {
    let fixtures_dir = "tests/fixtures/basic";
    
    // Get all .hrs files in the fixtures directory
    let fixture_files = get_fixture_files(fixtures_dir, "hrs");
    
    // Verify we have test fixtures
    assert!(!fixture_files.is_empty(), "No fixture files found in {}", fixtures_dir);
    
    // Verify each fixture is readable
    for fixture_path in fixture_files {
        let source = read_file_content(&fixture_path);
        assert!(!source.is_empty(), "Fixture file is empty: {}", fixture_path.display());
    }
}