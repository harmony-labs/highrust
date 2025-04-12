//! Code generation logic for HighRust.
//!
//! This module is responsible for emitting Rust code from the lowered IR.
//! The main entry point is [`generate_rust_code`], which transforms the
//! lowered IR into valid Rust code.

use crate::lowering::{
    LoweredBlock, LoweredData, LoweredDataKind, LoweredEnumVariant, LoweredExpr, LoweredField,
    LoweredFunction, LoweredItem, LoweredLiteral, LoweredModule, LoweredParam, LoweredStmt,
    LoweredType,
};
use std::fmt::Write;

/// Error type for code generation failures.
#[derive(Debug)]
pub enum CodegenError {
    /// An unsupported feature was encountered during code generation.
    UnsupportedFeature(&'static str),
    /// An error occurred during string formatting.
    FormatError(std::fmt::Error),
    /// An invalid IR construct was encountered.
    InvalidIr(String),
}

impl From<std::fmt::Error> for CodegenError {
    fn from(err: std::fmt::Error) -> Self {
        CodegenError::FormatError(err)
    }
}

/// Context for code generation.
/// This struct holds configuration, state, and utilities needed during codegen.
pub struct CodegenContext {
    /// Indentation level for pretty-printing
    indent_level: usize,
    /// Size of each indentation step
    indent_size: usize,
    /// Whether to add a comment indicating the code was transpiled
    add_transpiler_comment: bool,
}

impl CodegenContext {
    /// Creates a new codegen context with default settings.
    pub fn new() -> Self {
        CodegenContext {
            indent_level: 0,
            indent_size: 4,
            add_transpiler_comment: true,
        }
    }

    /// Creates a new codegen context with custom settings.
    pub fn with_options(indent_size: usize, add_transpiler_comment: bool) -> Self {
        CodegenContext {
            indent_level: 0,
            indent_size,
            add_transpiler_comment,
        }
    }

    /// Returns the current indentation as a string.
    fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.indent_size)
    }

    /// Increases the indentation level.
    fn increase_indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decreases the indentation level.
    fn decrease_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
}

/// Generates Rust code from the given lowered module using the provided codegen context.
///
/// # Arguments
///
/// * `module` - Reference to the lowered module to be converted into Rust code.
/// * `ctx` - Reference to the code generation context.
///
/// # Returns
///
/// A `Result` containing either the generated Rust code as a `String` or a `CodegenError`.
///
/// # Example
///
/// ```ignore
/// let module = lower_module(&ast_module)?;
/// let ctx = CodegenContext::new();
/// let rust_code = generate_rust_code(&module, &ctx)?;
/// ```
pub fn generate_rust_code(module: &LoweredModule, ctx: &mut CodegenContext) -> Result<String, CodegenError> {
    let mut output = String::new();
    
    // Generate code for each item in the module
    for item in &module.items {
        match item {
            LoweredItem::Function(func) => {
                generate_function(func, ctx, &mut output)?;
                writeln!(output)?;
            }
            LoweredItem::Data(data) => {
                generate_data(data, ctx, &mut output)?;
                writeln!(output)?;
            }
        }
    }
    
    Ok(output)
}

/// Generates Rust code for a function definition.
fn generate_function(
    func: &LoweredFunction,
    ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    // Function signature
    write!(output, "{}fn {}(", ctx.indent(), func.name)?;
    
    // Parameters
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            write!(output, ", ")?;
        }
        generate_param(param, ctx, output)?;
    }
    write!(output, ")")?;
    
    // Return type
    if let Some(ret_type) = &func.ret_type {
        write!(output, " -> ")?;
        generate_type(ret_type, ctx, output)?;
    }
    
    // Function body
    write!(output, " {{\n")?;
    
    ctx.increase_indent();
    generate_block(&func.body, ctx, output)?;
    ctx.decrease_indent();
    
    writeln!(output, "{}}}", ctx.indent())?;
    
    Ok(())
}

/// Generates Rust code for a function parameter.
fn generate_param(
    param: &LoweredParam,
    _ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    write!(output, "{}", param.name)?;
    
    // Add type annotation if available, otherwise default to i32
    if let Some(ty) = &param.ty {
        write!(output, ": ")?;
        generate_type(ty, _ctx, output)?;
    } else {
        write!(output, ": i32")?;
    }
    
    Ok(())
}

/// Generates Rust code for a data type (struct or enum).
fn generate_data(
    data: &LoweredData,
    ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    match &data.kind {
        LoweredDataKind::Struct(fields) => {
            writeln!(output, "{}struct {} {{", ctx.indent(), data.name)?;
            
            ctx.increase_indent();
            for field in fields {
                write!(output, "{}{}: ", ctx.indent(), field.name)?;
                generate_type(&field.ty, ctx, output)?;
                writeln!(output, ",")?;
            }
            ctx.decrease_indent();
            
            writeln!(output, "{}}}", ctx.indent())?;
        }
        LoweredDataKind::Enum(variants) => {
            writeln!(output, "{}enum {} {{", ctx.indent(), data.name)?;
            
            ctx.increase_indent();
            for variant in variants {
                generate_enum_variant(variant, ctx, output)?;
            }
            ctx.decrease_indent();
            
            writeln!(output, "{}}}", ctx.indent())?;
        }
    }
    
    Ok(())
}

/// Generates Rust code for an enum variant.
fn generate_enum_variant(
    variant: &LoweredEnumVariant,
    ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    write!(output, "{}{}", ctx.indent(), variant.name)?;
    
    if !variant.fields.is_empty() {
        write!(output, "(")?;
        
        for (i, field) in variant.fields.iter().enumerate() {
            if i > 0 {
                write!(output, ", ")?;
            }
            generate_type(&field.ty, ctx, output)?;
        }
        
        write!(output, ")")?;
    }
    
    writeln!(output, ",")?;
    
    Ok(())
}

/// Generates Rust code for a block of statements.
fn generate_block(
    block: &LoweredBlock,
    ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    for stmt in &block.stmts {
        generate_stmt(stmt, ctx, output)?;
    }
    
    Ok(())
}

/// Generates Rust code for a statement.
fn generate_stmt(
    stmt: &LoweredStmt,
    ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    match stmt {
        LoweredStmt::Let { name, value, ty } => {
            write!(output, "{}let {}", ctx.indent(), name)?;
            
            if let Some(ty) = ty {
                write!(output, ": ")?;
                generate_type(ty, ctx, output)?;
            }
            
            write!(output, " = ")?;
            generate_expr(value, ctx, output)?;
            writeln!(output, ";")?;
        }
        LoweredStmt::Expr(expr) => {
            write!(output, "{}", ctx.indent())?;
            generate_expr(expr, ctx, output)?;
            writeln!(output, ";")?;
        }
        LoweredStmt::Return(expr_opt) => {
            write!(output, "{}return", ctx.indent())?;
            
            if let Some(expr) = expr_opt {
                write!(output, " ")?;
                generate_expr(expr, ctx, output)?;
            }
            
            writeln!(output, ";")?;
        }
    }
    
    Ok(())
}

/// Generates Rust code for an expression.
fn generate_expr(
    expr: &LoweredExpr,
    ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    match expr {
        LoweredExpr::Literal(lit) => {
            generate_literal(lit, ctx, output)?;
        }
        LoweredExpr::Variable(name) => {
            write!(output, "{}", name)?;
        }
        LoweredExpr::Call { func, args } => {
            generate_expr(func, ctx, output)?;
            
            // Special case for println
            if let LoweredExpr::Variable(name) = &**func {
                if name == "println" {
                    // Convert to println! macro with proper formatting
                    write!(output, "!")?;
                    
                    if args.len() == 1 {
                        if let LoweredExpr::Literal(LoweredLiteral::String(s)) = &args[0] {
                            // Check if the string contains interpolation
                            if s.contains("${") {
                                // Convert string interpolation to Rust format
                                let mut formatted = String::new();
                                let mut parts = Vec::new();
                                let mut current = String::new();
                                let mut i = 0;
                                
                                while i < s.len() {
                                    if i + 1 < s.len() && &s[i..i+2] == "${" {
                                        let start = i + 2;
                                        let mut end = start;
                                        
                                        while end < s.len() && s.chars().nth(end) != Some('}') {
                                            end += 1;
                                        }
                                        
                                        if end < s.len() {
                                            formatted.push_str(&current);
                                            formatted.push_str("{}");
                                            current.clear();
                                            
                                            parts.push(s[start..end].to_string());
                                            i = end + 1;
                                        } else {
                                            current.push_str(&s[i..i+2]);
                                            i += 2;
                                        }
                                    } else {
                                        current.push(s.chars().nth(i).unwrap());
                                        i += 1;
                                    }
                                }
                                
                                formatted.push_str(&current);
                                
                                write!(output, "(\"{}\", ", formatted)?;
                                
                                for (i, part) in parts.iter().enumerate() {
                                    if i > 0 {
                                        write!(output, ", ")?;
                                    }
                                    write!(output, "{}", part)?;
                                }
                                
                                write!(output, ")")?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            
            write!(output, "(")?;
            
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(output, ", ")?;
                }
                generate_expr(arg, ctx, output)?;
            }
            
            write!(output, ")")?;
        }
        LoweredExpr::Block(block) => {
            writeln!(output, "{{")?;
            
            ctx.increase_indent();
            generate_block(block, ctx, output)?;
            ctx.decrease_indent();
            
            write!(output, "{}}}", ctx.indent())?;
        }
    }
    
    Ok(())
}

/// Generates Rust code for a literal value.
fn generate_literal(
    lit: &LoweredLiteral,
    _ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    match lit {
        LoweredLiteral::Int(i) => {
            write!(output, "{}", i)?;
        }
        LoweredLiteral::Float(f) => {
            write!(output, "{}", f)?;
        }
        LoweredLiteral::Bool(b) => {
            write!(output, "{}", b)?;
        }
        LoweredLiteral::String(s) => {
            write!(output, "\"{}\"", s.replace("\"", "\\\""))?;
        }
        LoweredLiteral::Null => {
            write!(output, "None")?;
        }
    }
    
    Ok(())
}

/// Generates Rust code for a type.
fn generate_type(
    ty: &LoweredType,
    _ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    match ty {
        LoweredType::Named(name, params) => {
            write!(output, "{}", name)?;
            
            if !params.is_empty() {
                write!(output, "<")?;
                
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ")?;
                    }
                    generate_type(param, _ctx, output)?;
                }
                
                write!(output, ">")?;
            }
        }
        LoweredType::Tuple(types) => {
            write!(output, "(")?;
            
            for (i, ty) in types.iter().enumerate() {
                if i > 0 {
                    write!(output, ", ")?;
                }
                generate_type(ty, _ctx, output)?;
            }
            
            write!(output, ")")?;
        }
        LoweredType::Array(inner) => {
            write!(output, "Vec<")?;
            generate_type(inner, _ctx, output)?;
            write!(output, ">")?;
        }
    }
    
    Ok(())
}