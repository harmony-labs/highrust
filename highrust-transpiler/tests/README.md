# HighRust Transpiler Test Suite

This directory contains the test suite for the HighRust transpiler. The tests are designed to validate the transpiler's functionality at different levels, from unit tests of individual components to full end-to-end integration tests.

## Test Structure

The test suite is structured as follows:

### 1. Unit Tests (`unit_tests.rs`)

These tests focus on validating individual components of the transpiler, such as:
- AST construction
- Parser functionality
- Lowering logic
- Code generation

Unit tests ensure that each component works correctly in isolation.

### 2. Golden File Tests (`golden_tests.rs`)

Golden file tests validate the end-to-end transpilation of HighRust code to Rust code by comparing the output against expected "golden" files. The testing process:

1. Takes HighRust source files from the `fixtures` directory
2. Runs them through the transpiler
3. Compares the output with the expected Rust code in the `expected` directory

To update the golden files when the transpiler output changes intentionally:
- Run the `generate_expected_outputs` test with the `--ignored` flag

### 3. Integration Tests (`integration_tests.rs`)

Integration tests validate the interactions between different components of the transpiler, ensuring they work correctly together in the full pipeline:

1. Parsing HighRust source files
2. Generating AST
3. Lowering to IR
4. Generating Rust code

## Test Fixtures

The `fixtures` directory contains HighRust source files used for testing. Each fixture has a corresponding expected output in the `expected` directory.

### Basic Fixtures

- `basic/hello_world.hrs`: A simple Hello World program
- `basic/variables_and_expressions.hrs`: Demonstrates variables and expressions
- `basic/pattern_matching.hrs`: Demonstrates pattern matching

## Running Tests

To run all the tests:
```
cargo test -p highrust-transpiler
```

To run ignored tests (such as those waiting for full implementation):
```
cargo test -p highrust-transpiler -- --ignored
```

To run a specific test:
```
cargo test -p highrust-transpiler <test_name>
```

## Test Coverage Goals

The test suite aims to achieve comprehensive coverage of the transpiler:

1. **Parser**: All valid HighRust syntax constructs
2. **AST**: All node types and their combinations
3. **Lowering**: All transformation and desugaring rules
4. **Code Generation**: All generated Rust patterns

As the transpiler implementation progresses, the test suite will be expanded to cover more complex language features and edge cases.

---

## 4. Runtime Tests (`run_runtime_tests.sh`)

Runtime tests validate that the transpiler not only generates correct Rust code, but that the code can be compiled and executed successfully.

### Purpose

These tests ensure that sample HighRust programs can be transpiled, compiled, and run without errors, providing end-to-end validation of the transpiler's output.

### How to Run

From the `highrust-transpiler/tests/` directory, execute:

```bash
./run_runtime_tests.sh
```

This script will:
1. Iterate over each `.hrs` file in `fixtures/basic/`
2. Transpile it to Rust using the CLI (`highrust`)
3. Optionally compare the generated Rust code to the expected output
4. Compile the generated Rust code with `rustc`
5. Run the resulting binary and check for successful execution

### Interpreting Results

- If all tests pass, the script will print "All runtime tests passed."
- If any test fails (compilation or runtime error), the script will print "Some runtime tests failed." and exit with a nonzero status.
- Warnings are shown if the generated Rust code differs from the expected output.

### Notes

- The script uses a temporary directory for generated files and binaries.
- The runtime tests provide an additional layer of confidence beyond golden file and integration tests, ensuring that the transpiler's output is valid Rust code that runs as expected.