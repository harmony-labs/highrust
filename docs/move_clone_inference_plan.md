# Task Plan: Per-Statement Move/Clone Inference for HighRust

## Overview

This document outlines the plan to implement correct per-statement move/clone inference for the HighRust transpiler, enabling the system to insert `.clone()` only when necessary and to properly track ownership and moves in a Rust-like way.

---

## 1. Refactor IR for Clone Annotation

- Update the IR (LoweredStmt or a new wrapper) to allow each let statement to be annotated with "needs_clone" for the right-hand side.

## 2. Implement Move State Tracking in Ownership Inference

- During ownership analysis, maintain a map: `HashMap<String, bool>` where the value is "moved" (true if the variable has been moved).
- As you process each statement:
  - For `let x = y;`:
    - If `y` is not moved, mark `y` as moved and annotate this statement as Normal.
    - If `y` is already moved, annotate this statement as NeedsClone.

## 3. Propagate Clone Annotation to Lowering

- When lowering AST to IR, propagate the "needs_clone" annotation to the IR for each let statement.

## 4. Update Codegen to Use Clone Annotation

- In codegen, for each let statement:
  - If "needs_clone" is true, emit `let x = y.clone();`
  - Otherwise, emit `let x = y;`

## 5. Expand Test Suite

- Add/expand tests for:
  - Multiple moves from the same variable (should insert .clone() for all but the first)
  - Moves interleaved with other statements
  - Moves in branches and loops
  - Moves of non-cloneable types (should error or warn)
- Ensure the clone_inference_tests and other relevant tests pass.

## 6. Documentation

- Update TASKS.md and PROGRESS.md to reflect this new move/clone inference milestone.
- Document the move/clone inference logic in DESIGN.md or a new architecture note.

---

## Mermaid Diagram: Move/Clone Inference Flow

```mermaid
flowchart TD
    A[Start: AST Statements] --> B[Ownership Inference: Track Move State]
    B --> C{Is RHS variable moved?}
    C -- No --> D[Mark as moved, annotate as Normal]
    C -- Yes --> E[Annotate as NeedsClone]
    D & E --> F[Lowering: Propagate annotation to IR]
    F --> G[Codegen: Emit .clone() if NeedsClone]
    G --> H[Output Rust Code]
```

---

**This plan will enable precise, Rust-like move/clone inference and correct .clone() insertion in the transpiler.**