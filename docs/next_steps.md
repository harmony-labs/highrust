# Next Steps: Implementing Borrow Inference

## Overview
After successfully implementing mutability inference, the next phase is to add borrow inference to the ownership system. This will allow HighRust to automatically determine when a variable should be borrowed (using `&` or `&mut`) versus moved.

## Implementation Plan

### 1. Enhance Variable Tracking
- Extend `VariableInfo` to track borrow state and lifetime
- Add fields to track whether a variable is currently borrowed and where
- Implement tracking for borrow lifetimes and overlap detection

### 2. Analyze Usage Patterns 
- Detect when a variable is used multiple times (candidate for borrowing)
- Check if a variable is used after being passed to a function (may need borrowing)
- Determine if a value is used for read-only operations (immutable borrow)
- Identify when a value is modified through a reference (mutable borrow)

### 3. Function Call Analysis
- Parse function signatures to determine if parameters need ownership
- Infer if arguments should be passed by reference or by value
- Handle standard library functions appropriately

### 4. Lifetime Analysis
- Track variable lifetimes to ensure borrowed references remain valid
- Implement lifetime conflict detection 
- Generate appropriate lifetime annotations

### 5. Update Lowering Module
- Add borrow operators to expressions in the IR
- Pass borrow information to code generation
- Handle return values and lifetime annotations

### 6. Update Code Generation
- Generate correct `&` and `&mut` operators
- Add lifetime annotations where necessary
- Ensure proper dereferencing when needed

### 7. Testing
- Add unit tests for borrow detection
- Create test cases for common patterns
- Test lifetime conflicts and validations

## Key Challenges

1. **Lifetime Inference**: Determining appropriate lifetimes for references without explicit annotations
2. **Lifetime Conflicts**: Detecting when borrows might create dangling references
3. **Optimization**: Finding the right balance between borrowing and moving
4. **Function Boundaries**: Analyzing functions without full context of their callers

## Expected Outcome
When implemented, the borrow inference system will allow HighRust code to be written without explicit borrow operators (`&`, `&mut`), while still generating correct and memory-safe Rust code with the appropriate borrowing semantics.