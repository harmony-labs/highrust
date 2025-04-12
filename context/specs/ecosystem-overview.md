# HighRust Ecosystem Overview

**Version:** 1.0  
**Date:** 2024-06-19  
**Owner:** HighRust Core Engineering

---

## 1. Introduction

The HighRust ecosystem is designed to provide a seamless, productive, and extensible environment for full-stack Rust development. It includes the HighRust language, transpiler, CLI tooling, standard library polyfills, documentation generation, and community resources, all aimed at maximizing developer velocity and code quality.

---

## 2. Core Components

### 2.1 Transpiler CLI

- **Command:** `highrustc` or `highrust transpile`
- Converts `.hrs` files to idiomatic `.rs` files.
- Supports modular builds, source maps, and robust error reporting.

### 2.2 Build Daemon & Dev Tooling

- **Live Reload:** Watches for file changes, triggers rebuilds.
- **Integration:** Works with Cargo, wasm-pack, trunk for native and WASM targets.
- **Hot Reload:** For rapid development cycles.

### 2.3 Standard Library Polyfills

- Provides HighRust-specific helpers (group-by, map, filter, safe Option/Result, async utilities).
- Bundled as Rust modules or via crate imports.

### 2.4 Documentation Generation

- **Command:** `highrust doc`
- Generates API documentation, mirroring Rustdoc expectations.
- Auto-links to Rust docs where applicable.

### 2.5 Error Reporting & Source Maps

- Inline mapping from HighRust to Rust for diagnostics and panics.
- All errors reference original HighRust code with actionable suggestions.

### 2.6 Testing Suite

- Golden file tests: .hrs input, .rs output, expected behavior.
- Runtime integration tests for server and WASM.
- CLI test runner for doctest-style inline tests.

---

## 3. Developer Experience

- **Ergonomic CLI:** Intuitive commands for transpile, build, dev, test, doc, and format.
- **Verbose Mode:** Optionally documents all mut/clone/&/lifetime transformations.
- **Contributor Guide:** Onboarding documentation for new developers.
- **Community Resources:** Example apps, guides, and open-source repository.

---

## 4. Configuration

- **highrust.toml:** Entrypoints, include/exclude globs, stdlib toggles, output paths, dependency and plugin overrides.

---

## 5. Extensibility

- **Plugin System:** Syntax, type system, codegen, lints, and formatting hooks.
- **Polyfills:** Users can write custom HighRust polyfills or use Rust crates directly.
- **Shared Types:** Canonical transpilation for WASM/native compatibility.

---

## 6. CLI Reference

| Command                       | Description                                                     |
|-------------------------------|-----------------------------------------------------------------|
| `highrust transpile`          | Transpile .hrs to .rs, preserving folder structure              |
| `highrust build [--server]`   | Build server for native, with cargo integration                 |
| `highrust build --client`     | Build client WASM, run asset tools                              |
| `highrust dev`                | Live reload, watches .hrs, (re)builds on change, runs project   |
| `highrust test`               | Run transpile and runtime tests across targets                  |
| `highrust doc`                | Generate API docs                                               |
| `highrust fmt`                | Format .hrs files (planned)                                     |

---

## 7. Community & Governance

- **Open Source:** Community-driven development, issue tracker, milestones.
- **Contribution Guide:** Clear onboarding, code of conduct, and roadmap.
- **Metrics:** >98% transpile success, <60s for 100kloc, 100% test coverage for core syntax.

---

## 8. Security & Compliance

- No unsafe code in transpiled output by default (except in explicit Rust blocks).
- Optional linter for forbidden constructs.
- Sanitization of embedded Rust blocks if security is set to strict.

---

**For technical details, see the Language Specification and Transpiler Architecture documents.**