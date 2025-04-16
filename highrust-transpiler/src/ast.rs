//! Abstract Syntax Tree (AST) definitions for HighRust.
//!
//! This module defines the core AST node types for the HighRust language,
//! as described in the language specification and transpiler architecture.
//! The AST is the output of the parser and the input to semantic analysis and lowering.
//!
//! Each node includes documentation and, where appropriate, source position information
//! for diagnostics and source mapping.


/// Represents a span in the source file for diagnostics and source mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// The root of a HighRust AST: a module (source file).
#[derive(Debug, Clone)]
pub struct Module {
    pub items: Vec<ModuleItem>,
    pub span: Span,
}

/// Top-level items in a module.
#[derive(Debug, Clone)]
pub enum ModuleItem {
    Import(Import),
    Export(Export),
    Data(DataDef),
    Function(FunctionDef),
    EmbeddedRust(EmbeddedRustBlock),
}

/// Import statement (e.g., `import foo::bar` or `import rust "foo.rs"`).
#[derive(Debug, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub is_rust: bool,
    pub span: Span,
}

/// Export statement (e.g., `export foo`).
#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub span: Span,
}

/// Data type definition: struct, enum, or tagged union.
#[derive(Debug, Clone)]
pub struct DataDef {
    pub name: String,
    pub kind: DataKind,
    pub generics: Vec<TypeParam>,
    pub span: Span,
}

/// Kinds of data types.
#[derive(Debug, Clone)]
pub enum DataKind {
    Struct(Vec<Field>),
    Enum(Vec<EnumVariant>),
    TaggedUnion(Vec<TaggedVariant>),
}

/// Field in a struct or record.
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Enum variant.
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
}

/// Tagged union variant.
#[derive(Debug, Clone)]
pub struct TaggedVariant {
    pub tag: String,
    pub ty: Type,
    pub span: Span,
}

/// Type parameter for generics.
#[derive(Debug, Clone)]
pub struct TypeParam {
    pub name: String,
    pub span: Span,
}

/// Function definition (sync or async).
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: Option<Type>,
    pub body: Block,
    pub is_async: bool,
    pub is_rust: bool, // true if @rust function
    pub span: Span,
}

/// Function parameter.
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<Type>,
    pub span: Span,
}

/// Block of statements.
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// Statements in HighRust.
#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        pattern: Pattern,
        value: Expr,
        ty: Option<Type>,
        span: Span,
    },
    Expr(Expr),
    Return(Option<Expr>, Span),
    If {
        cond: Expr,
        then_branch: Block,
        else_branch: Option<Block>,
        span: Span,
    },
    While {
        cond: Expr,
        body: Block,
        span: Span,
    },
    For {
        pattern: Pattern,
        iterable: Expr,
        body: Block,
        span: Span,
    },
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
        span: Span,
    },
    Try {
        block: Block,
        catch: Option<Block>,
        span: Span,
    },
    EmbeddedRust(EmbeddedRustBlock),
}

/// Expressions in HighRust.
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal, Span),
    Variable(String, Span),
    Wildcard(Span),
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    FieldAccess {
        base: Box<Expr>,
        field: String,
        span: Span,
    },
    Block(Block),
    Await {
        expr: Box<Expr>,
        span: Span,
    },
    Comprehension {
        pattern: Pattern,
        iterable: Box<Expr>,
        body: Box<Expr>,
        span: Span,
    },
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
        span: Span,
    },
    Try(Box<Expr>, Span),
    // Add more as needed (e.g., binary ops, unary ops)
}

/// Pattern for let/match destructuring.
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard(Span),
    Variable(String, Span),
    Tuple(Vec<Pattern>, Span),
    TuplePair(Box<Pattern>, Box<Pattern>, Span),
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
        span: Span,
    },
    Enum {
        name: String,
        variant: String,
        inner: Option<Box<Pattern>>,
        span: Span,
    },
    Literal(Literal, Span),
}

/// Match arm for match statements.
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<Expr>>,
    pub expr: Box<Expr>,
    pub span: Span,
}

/// Literal values.
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

/// Embedded Rust block or @rust function.
#[derive(Debug, Clone)]
pub struct EmbeddedRustBlock {
    pub code: String,
    pub span: Span,
}

/// Type annotation.
#[derive(Debug, Clone)]
pub enum Type {
    Named(String, Vec<Type>), // e.g., Foo, Option<T>, Result<T, E>
    Option(Box<Type>),        // Option<T>
    Result(Box<Type>, Box<Type>), // Result<T, E>
    Tuple(Vec<Type>),
    Array(Box<Type>),
    // TODO: Function types, generics, etc.
}