# HighRust Project Task Checklist

## Phase 1: Core Transpiler MVP

**Percent complete: 28% (2.5/9 tasks)**

- [ ] Implement parser for HighRust syntax
- [x] Build Abstract Syntax Tree (AST) generation
- [ ] Implement basic lowering from AST to intermediate representation
- [ ] Develop working code generator: transpile .hrs files to .rs files
  - [x] Initial codegen module skeleton
- [x] Create minimal CLI for transpilation
- [ ] Implement file watcher for automatic transpilation
- [ ] Develop initial test suite
  - [ ] Add golden file tests
  - [ ] Add basic runtime tests

## Phase 2: Ownership and Conversion Inference

- [ ] Implement dataflow-based mutability, borrow, and clone inference
- [ ] Insert .to_string() conversions where required
- [ ] Implement Option and Result type mapping
- [ ] Add full pattern matching support
- [ ] Implement source mapping for diagnostics

## Phase 3: Async, Polyfills, Build Integration

- [ ] Implement full async lowering and runtime autonomy
- [ ] Develop standard helper modules
  - [ ] Implement map helper
  - [ ] Implement filter helper
  - [ ] Implement group-by helper
  - [ ] Implement additional standard helpers as needed
- [ ] Develop development server with hot reload capability
- [ ] Integrate full cargo build pipeline
- [ ] Integrate full WASM build pipeline

## Phase 4: IDE Extensions, Plugin System, Community Release

- [ ] Implement plugin hooks for syntax extension
- [ ] Implement plugin hooks for type system extension
- [ ] Implement plugin hooks for code generation extension
- [ ] Add support for external lints
- [ ] Add support for external formatters
- [ ] Write documentation and guides
- [ ] Create example repository
- [ ] Write community contribution documentation