# HighRust Language Specification

**Version:** 1.0  
**Date:** 2024-06-19  
**Owner:** HighRust Core Engineering

---

## 1. Overview

HighRust is a high-level, ergonomic dialect of Rust designed to maximize developer productivity and code clarity while preserving Rust’s safety and performance. It enables rapid full-stack development (native, server, WASM) with TypeScript-like velocity, reducing the need for explicit mutability, lifetimes, and low-level conversions. HighRust code is transpiled to idiomatic Rust, ensuring full compatibility with the Rust ecosystem.

---

## 2. Language Philosophy

- **Ergonomics First:** Minimize boilerplate and explicitness (e.g., mut, clone, lifetimes) unless required.
- **Safety by Default:** All code is safe unless explicitly marked as raw/unsafe Rust.
- **Progressive Disclosure:** Advanced features and low-level control are available via escape hatches.
- **Full-Stack Ready:** Designed for seamless code sharing between server, client, and WASM targets.
- **Interop:** Directly import and embed Rust code where needed.

---

## 3. Syntax and Semantics

### 3.1 Syntax

- **Rust-like Syntax:** Familiar to Rust developers, but with reduced explicitness.
- **No Explicit `mut`, `clone`, `&`, or lifetimes** (unless inside explicit Rust blocks).
- **Data Declarations:** Use `data` for struct-like and enum types.
- **Pattern Matching:** Enhanced destructuring, guards, and shorthands.
- **Async/Await:** Native async functions and auto-lowered `await`.
- **Top-Level Code:** Modules, imports, and exports.
- **Embedded Rust:** Use `rust { ... }` blocks or `@rust` functions for explicit Rust.

### 3.2 Semantics

- **Type Inference:** Hindley-Milner style, explicit annotations optional.
- **Ownership/Borrowing:** Inferred; explicit only in embedded Rust.
- **Option/Result Handling:** Ergonomic sugars for nullables, try/catch, and propagation.
- **Implicit Conversions:** Automatic `.to_string()`, Vec/slice, and type conversions where safe.
- **Error Handling:** Try/catch blocks map to Rust’s Result/Option.

---

## 4. Core Features

- **Functions:** Sync and async, with inferred types and ergonomic signatures.
- **Data Types:** Structs, enums, tagged unions via `data`.
- **Pattern Matching:** List/struct destructuring, guards, default cases.
- **Loops and Iterators:** For, while, map/filter/group-by comprehensions.
- **Option/Result Propagation:** Early returns, nullables, and error handling.
- **Code Sharing:** Canonical output for shared types (WASM/native).
- **Testing/Docs:** Doctest-style inline tests, auto-generated docs.

---

## 5. Escape Hatches and Interop

- **Embedded Rust Blocks:** `rust { ... }` for verbatim Rust code.
- **@rust Functions:** Mark functions to be included as-is.
- **Rust Module Imports:** `import rust "foo.rs"` for external Rust modules.
- **Explicit Control:** Drop to raw Rust for performance or edge cases.

---

## 6. Example

**HighRust:**
```rust
fn fetch_users() => async [User] {
    let data = await http_get("/api/users");
    parse_users(data)
}
```

**Transpiled Rust:**
```rust
async fn fetch_users() -> Result<Vec<User>, SomeErrorType> {
    let data = http_get("/api/users").await?;
    let users = parse_users(data)?;
    Ok(users)
}
```

---

## 7. Non-Goals

- Not a replacement for Rust; always transpiles to Rust, never directly to binaries.
- No procedural macros or full IDE integration in v1.
- No GC by default; escape hatches for advanced memory management.

---

## 8. Precedents

Inspired by TypeScript, Elm, Kotlin, Scala, ReasonML, and other high-level-to-systems transpiled languages.

---

## 9. Open Questions

- Best model for mutability/borrowing hints.
- Default async runtime and configuration.
- Strict/dynamic build modes for clone/borrow enforcement.
- Ergonomic syntax for embedded/external Rust code.

---

**For further details, see the Transpiler Architecture and Implementation Roadmap.**