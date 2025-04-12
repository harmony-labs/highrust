//! Test utilities for the HighRust transpiler
//!
//! This module provides common utilities for testing the HighRust transpiler,
//! including functions for running golden file tests.

use std::fs;
use std::path::{Path, PathBuf};

/// Get a list of all fixture files in a directory with a specific extension, recursively
pub fn get_fixture_files(dir: &str, extension: &str) -> Vec<PathBuf> {
    fn visit_dirs(dir: &Path, extension: &str, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, extension, files)?;
                } else if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    let fixtures_dir = Path::new(dir);
    let mut files = Vec::new();
    visit_dirs(fixtures_dir, extension, &mut files)
        .expect(&format!("Failed to read fixtures directory: {}", dir));
    files
}

/// Get the expected output path for a fixture file
pub fn get_expected_path(fixture_path: &Path, input_dir: &str, output_dir: &str) -> PathBuf {
    let stem = fixture_path.file_stem().unwrap();
    
    // Create path to expected output file, changing extension from .hrs to .rs
    Path::new(output_dir)
        .join(fixture_path.strip_prefix(input_dir).unwrap().parent().unwrap_or(Path::new("")))
        .join(format!("{}.rs", stem.to_string_lossy()))
}

/// Read the content of a file
pub fn read_file_content(path: &Path) -> String {
    fs::read_to_string(path).expect(&format!("Failed to read file: {}", path.display()))
}

/// Write content to a file, creating parent directories if needed
pub fn write_file_content(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect(&format!("Failed to create directory: {}", parent.display()));
    }
    fs::write(path, content).expect(&format!("Failed to write file: {}", path.display()));
}