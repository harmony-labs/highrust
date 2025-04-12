# HighRust Project Design

**Version:** 1.0  
**Date:** 2024-06-19  
**Owner:** HighRust Core Engineering

---

## 1. Introduction & Technical Goals

HighRust is a high-level, ergonomic dialect of Rust designed to maximize developer productivity and code clarity while preserving Rust’s safety and performance. The project aims to enable rapid, full-stack development (native, server, WASM) with TypeScript-like velocity, reducing the need for explicit mutability, lifetimes, and low-level conversions. HighRust code is transpiled to idiomatic Rust, ensuring full compatibility with the Rust ecosystem.

**Major Design Objectives:**
- **Ergonomics:** Minimize boilerplate and explicitness, making code more concise and readable.
- **Safety:** All code is safe by default, with explicit escape hatches for raw/unsafe Rust.
- **Extensibility:** Modular architecture with plugin APIs for language and tooling extensions.
- **Performance:** Fast transpilation and build cycles, targeting 10kloc/sec and efficient dev/watch modes.
- **Full-Stack Readiness:** Seamless code sharing between server, client, and WASM targets.

---

## 2. Architecture Overview

The HighRust project is composed of several tightly integrated components:

- **HighRust Language:** A Rust-like syntax with reduced explicitness, ergonomic sugars, and progressive disclosure of advanced features.
- **Transpiler:** Converts HighRust (.hrs) code to idiomatic Rust (.rs), supporting robust diagnostics, source maps, and extensibility via plugins.
- **CLI Tooling:** The `highrust` CLI provides commands for transpilation, building, testing, documentation, and formatting.
- **Ecosystem Components:** Standard library polyfills, documentation generation, testing suite, and community resources for onboarding and collaboration.

**Key Technical Choices:**
- Implementation in Rust for performance and ecosystem alignment.
- Use of parser generators (pest or lalrpop) and the syn crate for code generation.
- Plugin system for extensibility at multiple stages (syntax, type system, codegen, lints, formatting).
- Source maps for precise diagnostics and debugging.
- Canonical output for shared types to support WASM/native compatibility.

---

## 3. Language Design Philosophy

HighRust is guided by the following principles:

- **Ergonomics First:** Reduce the need for explicit `mut`, `clone`, lifetimes, and references unless required for advanced use cases.
- **Safety by Default:** All code is safe unless explicitly marked as raw/unsafe Rust.
- **Progressive Disclosure:** Advanced features and low-level control are available via escape hatches (e.g., embedded Rust blocks).
- **Full-Stack Ready:** Designed for seamless code sharing across server, client, and WASM targets.
- **Interop:** Directly import and embed Rust code where needed.

**Core Language Features:**
- Rust-like syntax with reduced explicitness.
- Data declarations via `data` for structs/enums.
- Enhanced pattern matching and destructuring.
- Native async/await and ergonomic error handling.
- Type inference (Hindley-Milner style).
- Implicit conversions and ergonomic Option/Result propagation.
- Embedded Rust via `rust { ... }` blocks or `@rust` functions.

---

## 4. Transpiler Pipeline & Technical Choices

The transpiler is a modular, deterministic toolchain with the following pipeline:

1. **Source Loader:** Resolves multi-file roots, imports, and inline mixes.
2. **Tokenizer/Lexer:** Custom lexing with robust error recovery.
3. **Parser:** Full production rules for HighRust, outputting an AST with position mapping.
4. **Semantic Analyzer:** Variable/module/import resolution, type inference, ownership/borrowing analysis, lifetime inference, async/blocking context detection, and trait/impl resolution.
5. **Desugaring/Lowering:** Expands ergonomic sugars and lowers advanced constructs to canonical IR and Rust forms.
6. **Embed/Transplant Rust Blocks:** Inlines explicit Rust code, binding variables as needed and mapping errors to the correct source.
7. **Code Generation:** Emits idiomatic Rust code with minimal boilerplate, places files for cargo/wasm builds, and generates manifests as needed.
8. **Source Maps:** Maintains mapping from HighRust to Rust for diagnostics and debugging.
9. **Error/Warning Reporting:** All errors reference original HighRust code with actionable suggestions.
10. **CLI/Build Integration:** Modular output for Cargo/WASM, hot reload for development, and side-by-side error reporting.

**Technical Implementation:**
- Written in Rust for performance and ecosystem compatibility.
- Parser built with pest or lalrpop.
- Code generation via syn crate or direct string-building.
- Test-driven development with golden files and runtime samples.

---

## 5. Extensibility Mechanisms

HighRust is designed for extensibility at every layer:

- **Plugin API:** Extends syntax, type system, codegen, lints, and formatting.
- **Polyfills:** Standard library helpers for common patterns (group-by, map, filter, async utilities), bundled as Rust modules or via crate imports.
- **Configuration:** `highrust.toml` for entrypoints, stdlib toggles, output paths, dependency and plugin overrides.
- **Custom Polyfills:** Users can write custom HighRust polyfills or use Rust crates directly.
- **Shared Types:** Canonical transpilation ensures compatibility between WASM and native targets.

---

## 6. Developer Experience

- **Ergonomic CLI:** Intuitive commands for transpile, build, dev, test, doc, and format.
- **Live/Hot Reload:** Watches for file changes and triggers rebuilds for rapid development.
- **Integration:** Works seamlessly with Cargo, wasm-pack, and trunk for native and WASM targets.
- **Verbose Mode:** Optionally documents all mut/clone/&/lifetime transformations for transparency.
- **Testing Suite:** Golden file tests, runtime integration tests, and CLI test runner for doctest-style inline tests.
- **Documentation Generation:** `highrust doc` generates API documentation, mirroring Rustdoc and auto-linking to Rust docs.
- **Contributor Guide & Community Resources:** Onboarding documentation, example apps, guides, and open-source repository.

---

## 7. Integration with the Rust Ecosystem

- **Output:** Always transpiles to idiomatic Rust, never directly to binaries.
- **Build Integration:** Output is compatible with Cargo, wasm-pack, and other Rust tooling.
- **Interop:** Embedded Rust blocks, @rust functions, and Rust module imports allow direct use of Rust code and crates.
- **Source Maps:** Maintains mapping for debugging and error reporting in Rust tools.
- **Security:** No unsafe code in transpiled output by default (except in explicit Rust blocks); optional linter for forbidden constructs; sanitization of embedded Rust blocks in strict mode.

---

**HighRust is inspired by TypeScript, Elm, Kotlin, Scala, ReasonML, and other high-level-to-systems transpiled languages, but is uniquely focused on maximizing Rust’s safety, performance, and ecosystem compatibility while delivering a modern, ergonomic developer experience.**