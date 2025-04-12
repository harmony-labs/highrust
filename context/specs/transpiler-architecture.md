# HighRust Transpiler Architecture

**Version:** 1.0  
**Date:** 2024-06-19  
**Owner:** HighRust Core Engineering

---

## 1. Overview

The HighRust transpiler is a modular, production-grade tool that converts HighRust source code (.hrs) into idiomatic Rust (.rs), supporting full-stack development (native, server, WASM). It is designed for extensibility, robust diagnostics, and seamless integration with the Rust toolchain.

---

## 2. Architectural Goals

- **Deterministic Output:** Given the same input and config, output is always the same.
- **Performance:** Target 10kloc/sec transpile speed; efficient dev/watch mode.
- **Extensibility:** Plugin API for syntax, type system, and codegen extensions.
- **Fault Tolerance:** Graceful error recovery for IDE/dev server use.
- **Developer-Centric:** Inline source maps, actionable diagnostics, and clear CLI UX.

---

## 3. Compiler Pipeline

### 3.1 Pipeline Stages

1. **Source Loader**
   - Resolves multi-file roots, imports, and inline mixes.

2. **Tokenizer/Lexer**
   - Custom lex spec using pest or lalrpop.
   - k-token lookahead, robust error recovery.

3. **Parser**
   - Full production rules for HighRust.
   - Outputs AST with position mapping.

4. **Semantic Analyzer**
   - Variable/module/import resolution (shadowing, re-exports).
   - Hindley-Milner type inference.
   - Ownership/borrowing analysis (move/borrow/clone).
   - Lifetime inference (emitted only if necessary).
   - Async/blocking context detection.
   - Trait/impl resolution.

5. **Desugaring/Lowering**
   - Expands shorthands (pattern, group-by, filter, etc.) to canonical IR.
   - Lowers async/await, try/catch, nullability to Rust forms.

6. **Embed/Transplant Rust Blocks**
   - Inlines explicit Rust code as blocks/functions/modules.
   - Binds HighRust variables as borrowed/owned as needed.
   - Maps errors to correct source.

7. **Code Generation**
   - Emits idiomatic .rs code, minimal boilerplate.
   - Configurable comments for inferred mut, clone, etc.
   - Places files for cargo/wasm builds; generates Cargo manifests if needed.

8. **Source Maps**
   - Maintains .hrs:line to .rs:line mapping for diagnostics and panics.

9. **Error/Warning Reporting**
   - All errors point to original HighRust source.
   - Suggests fixes for ambiguous clones/borrows or missing type hints.

10. **CLI/Build Integration**
    - Modular output for Cargo/WASM.
    - Hot reload for dev.
    - Prints transpile and rustc errors side-by-side.
    - Exits with correct codes for CI/CD.

---

## 4. Language Feature Handling

- **Ergonomics:** No explicit mut, clone, &, or lifetimes unless in explicit Rust.
- **Async:** Auto-lowered async/await, runtime selection/config.
- **Pattern Matching:** List destructuring, guards, shorthands.
- **Option/Result:** Nullables, try/catch, early return propagation.
- **Interop:** Embedded Rust blocks, @rust functions, Rust module imports.
- **Shared Types:** Canonical output for WASM/native compatibility.

---

## 5. Extensibility

- **Plugin API:** Syntax, type system, codegen, lints, and formatting hooks.
- **Polyfills:** Standard library helpers for group-by, map, filter, etc.
- **Config:** highrust.toml for entrypoints, stdlib toggles, output paths, plugins.

---

## 6. Error Handling & Diagnostics

- **Compile-Time:** Friendly, prescriptive errors with code snippets.
- **Runtime:** No effect at transpile stage; handled by Rust.
- **Source Maps:** Maintains mapping for debugging and error reporting.
- **Verbose Mode:** Optionally documents all mut/clone/&/lifetime transformations.

---

## 7. Testing & Validation

- **Golden Files:** .hrs → .rs output, validated with rustc/cargo.
- **Runtime Integration:** Server & WASM test harnesses.
- **Coverage:** 100% test coverage for core syntax; output matches golden files.

---

## 8. Technical Choices

- **Implementation Language:** Rust.
- **Parser:** pest or lalrpop.
- **Codegen:** syn crate or direct string-building for MVP.
- **Testing:** Test-driven, golden files, runtime samples.

---

## 9. Open Questions

- Generalization/documentation of ownership analysis.
- Configurability for advanced Rust devs.
- Ergonomic syntax for Rust escape hatches.

---

**For implementation phases and roadmap, see the Implementation Roadmap document.**