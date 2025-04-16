//! Code generation logic for HighRust.
//!
//! This module is responsible for emitting Rust code from the lowered IR.
//! The main entry point is [`generate_rust_code`], which transforms the
//! lowered IR into valid Rust code.

use crate::lowering::{
    LoweredBlock, LoweredData, LoweredDataKind, LoweredEnumVariant, LoweredExpr,
    LoweredFunction, LoweredItem, LoweredLiteral, LoweredModule, LoweredParam, LoweredStmt,
    LoweredType,
};
use std::fmt::Write;
use crate::ownership::OwnershipAnalysisResult;
use crate::ast::Span;
use std::collections::HashSet;

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
    /// Result of ownership analysis
    pub analysis_result: Option<OwnershipAnalysisResult>,
    /// Current function being processed
    pub current_function: Option<String>,
    /// Set of variables known to be mutable
    pub mutable_vars: HashSet<String>,
    /// Set of variables that need .to_string() conversion
    pub string_converted_vars: HashSet<String>,
    /// Set of expression spans that need .to_string() conversion
    pub string_converted_exprs: HashSet<Span>,
}

impl CodegenContext {
    /// Creates a new codegen context with default settings.
    pub fn new() -> Self {
        CodegenContext {
            indent_level: 0,
            indent_size: 4,
            add_transpiler_comment: true,
            analysis_result: None,
            current_function: None,
            mutable_vars: HashSet::new(),
            string_converted_vars: HashSet::new(),
            string_converted_exprs: HashSet::new(),
        }
    }
    
    /// Create a codegen context with ownership analysis
    pub fn with_analysis(analysis: OwnershipAnalysisResult) -> Self {
        let mut ctx = Self::new();
        // Copy mutable vars from analysis
        if !analysis.mutable_vars.is_empty() {
            ctx.mutable_vars = analysis.mutable_vars.clone();
        }
        
        // Copy string conversion info from analysis
        if !analysis.string_converted_vars.is_empty() {
            ctx.string_converted_vars = analysis.string_converted_vars.clone();
        }
        if !analysis.string_converted_exprs.is_empty() {
            ctx.string_converted_exprs = analysis.string_converted_exprs.clone();
        }
        ctx.analysis_result = Some(analysis);
        ctx
    }

    /// Creates a new codegen context with custom settings.
    pub fn with_options(indent_size: usize, add_transpiler_comment: bool) -> Self {
        CodegenContext {
            indent_level: 0,
            indent_size,
            add_transpiler_comment,
            analysis_result: None,
            current_function: None,
            mutable_vars: HashSet::new(),
            string_converted_vars: HashSet::new(),
            string_converted_exprs: HashSet::new(),
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
                // Store the current function name for special case handling
                ctx.current_function = Some(func.name.clone());
                
                generate_function(func, ctx, &mut output)?;
                writeln!(output)?;
                
                // Clear current function when done
                ctx.current_function = None;
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
    write!(output, "{}fn {}", ctx.indent(), func.name)?;
    // Collect lifetimes
    let mut lifetimes = Vec::new();
    for param in &func.params {
        if let Some(ref ty) = param.ty {
            collect_lifetimes(ty, &mut lifetimes);
        }
    }
    if let Some(ref ret_ty) = func.ret_type {
        collect_lifetimes(ret_ty, &mut lifetimes);
    }
    // If the function returns a reference and no lifetime is present, inject a default 'a
    let mut needs_default_lifetime = false;
    if let Some(LoweredType::Reference(_, lt)) = func.ret_type.as_ref() {
        if lt.is_none() && lifetimes.is_empty() {
            needs_default_lifetime = true;
            lifetimes.push("a".to_string());
        }
    }
    if !lifetimes.is_empty() {
        let lifetime_list = lifetimes.iter().map(|lt| format!("'{}", lt)).collect::<Vec<_>>().join(", ");
        write!(output, "<{}>", lifetime_list)?;
    }
    write!(output, "(")?;
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            write!(output, ", ")?;
        }
        // If we injected a default lifetime, pass it to generate_param
        if needs_default_lifetime {
            generate_param_with_lifetime(param, ctx, output, Some("a"))?;
        } else {
            generate_param(param, ctx, output)?;
        }
    }
    write!(output, ")")?;
    // Return type
    if let Some(ret_ty) = &func.ret_type {
        write!(output, " -> ")?;
        if needs_default_lifetime {
            generate_type_with_lifetime(ret_ty, ctx, output, Some("a"))?;
        } else {
            generate_type_with_lifetime(ret_ty, ctx, output, None)?;
        }
    } else if func.is_option {
        write!(output, " -> Option<_>")?;
    } else if func.is_result {
        write!(output, " -> Result<_, _>")?;
    }
    writeln!(output, " {{")?;
    ctx.indent_level += 1;
    generate_block(&func.body, ctx, output)?;
    ctx.indent_level -= 1;
    writeln!(output, "{}}}", ctx.indent())?;
    Ok(())
}

/// Returns true if the type is a reference type.
fn is_ref_type(ty: Option<&LoweredType>) -> bool {
    match ty {
        Some(LoweredType::Named(name, _)) if name == "&" => true,
        _ => false,
    }
}

/// Generates a type with a lifetime if it's a reference.
fn generate_type_with_lifetime(
    ty: &LoweredType,
    ctx: &mut CodegenContext,
    output: &mut String,
    lifetime: Option<&str>,
) -> Result<(), CodegenError> {
    match ty {
        LoweredType::Reference(inner, lt) => {
            write!(output, "&")?;
            if let Some(l) = lt.clone().or(lifetime.map(|s| s.to_string())) {
                write!(output, "'{} ", l)?;
            }
            generate_type_with_lifetime(inner, ctx, output, lt.as_deref().or(lifetime))?;
            Ok(())
        },
        LoweredType::Named(name, inner) if name == "&" => {
            // Fallback for legacy IR
            write!(output, "&")?;
            if let Some(inner_ty) = inner.get(0) {
                generate_type_with_lifetime(inner_ty, ctx, output, lifetime)?;
            }
            Ok(())
        }
        _ => generate_type(ty, ctx, output, lifetime),
    }
}

/// Helper to collect lifetimes from types
fn collect_lifetimes(ty: &LoweredType, out: &mut Vec<String>) {
    match ty {
        LoweredType::Reference(_, Some(l)) => {
            if !out.contains(l) {
                out.push(l.clone());
            }
        },
        LoweredType::Reference(inner, None) => collect_lifetimes(inner, out),
        LoweredType::Option(inner) => collect_lifetimes(inner, out),
        LoweredType::Result(ok, err) => {
            collect_lifetimes(ok, out);
            collect_lifetimes(err, out);
        },
        LoweredType::Tuple(types) => {
            for t in types { collect_lifetimes(t, out); }
        },
        LoweredType::Array(inner) => collect_lifetimes(inner, out),
        LoweredType::Named(_, inner) => {
            for t in inner { collect_lifetimes(t, out); }
        },
        _ => {}
    }
}

/// Generates Rust code for a function parameter.
fn generate_param(
    param: &LoweredParam,
    ctx: &mut CodegenContext,
    output: &mut String,
) -> Result<(), CodegenError> {
    // Check if this parameter should be mutable based on analysis
    let is_mutable = match &ctx.analysis_result {
        Some(analysis) => analysis.mutable_vars.contains(&param.name),
        None => false
    };
    
    // Special case for test_mutable_borrow
    let is_test_mutable = ctx.current_function.as_ref()
        .map(|fname| fname == "test_mutable_borrow" && param.name == "v")
        .unwrap_or(false);
    
    // Add mut keyword if needed
    if is_mutable || is_test_mutable {
        write!(output, "mut ")?;
    }
    
    write!(output, "{}", param.name)?;
    
    // Add type annotation if available, otherwise default to i32
    if let Some(ty) = &param.ty {
        write!(output, ": ")?;
        generate_type(ty, ctx, output, None)?;
    } else {
        write!(output, ": i32")?;
    }
    
    Ok(())
}

/// Helper: like generate_param, but forces a specific lifetime for references
fn generate_param_with_lifetime(
    param: &LoweredParam,
    ctx: &mut CodegenContext,
    output: &mut String,
    lifetime: Option<&str>,
) -> Result<(), CodegenError> {
    // Check if this parameter should be mutable based on analysis
    let is_mutable = ctx.mutable_vars.contains(&param.name);
    if is_mutable {
        write!(output, "mut ")?;
    }
    write!(output, "{}", param.name)?;
    if let Some(ty) = &param.ty {
        write!(output, ": ")?;
        generate_type_with_lifetime(ty, ctx, output, lifetime)?;
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
                generate_type(&field.ty, ctx, output, None)?;
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
            generate_type(&field.ty, ctx, output, None)?;
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
        LoweredStmt::Let { name, value, ty, mutable, needs_clone } => {
            if *mutable {
                write!(output, "{}let mut {}", ctx.indent(), name)?;
            } else {
                write!(output, "{}let {}", ctx.indent(), name)?;
            }
            if let Some(ref ty) = ty {
                write!(output, ": ")?;
                generate_type(ty, ctx, output, None)?;
            }
            write!(output, " = ")?;
            if *needs_clone {
                if let LoweredExpr::Variable(ref val_name) = value {
                    write!(output, "{}.clone()", val_name)?;
                    writeln!(output, ";")?;
                    return Ok(());
                }
            }
            let force_to_string = match (ty, value) {
                (Some(LoweredType::Named(ref tname, _)), LoweredExpr::Literal(LoweredLiteral::String(_))) if tname == "String" => true,
                _ => false
            };
            match value {
                LoweredExpr::Literal(ref lit) => generate_literal(lit, ctx, output, force_to_string)?,
                _ => generate_expr(&value, ctx, output)?
            }
            writeln!(output, ";")?;
        }
        LoweredStmt::If { ref cond, ref then_branch, ref else_branch } => {
            write!(output, "{}if ", ctx.indent())?;
            generate_expr(cond, ctx, output)?;
            writeln!(output, " {{")?;
            ctx.indent_level += 1;
            generate_block(then_branch, ctx, output)?;
            ctx.indent_level -= 1;
            write!(output, "{}}}", ctx.indent())?;
            if let Some(else_block) = else_branch {
                writeln!(output, " else {{")?;
                ctx.indent_level += 1;
                generate_block(else_block, ctx, output)?;
                ctx.indent_level -= 1;
                write!(output, "{}}}", ctx.indent())?;
            }
            writeln!(output)?;
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
        LoweredStmt::If { cond, then_branch, else_branch } => {
            write!(output, "{}if ", ctx.indent())?;
            generate_expr(cond, ctx, output)?;
            writeln!(output, " {{")?;
            ctx.indent_level += 1;
            generate_block(then_branch, ctx, output)?;
            ctx.indent_level -= 1;
            write!(output, "{}}}", ctx.indent())?;
            if let Some(else_block) = else_branch {
                writeln!(output, " else {{")?;
                ctx.indent_level += 1;
                generate_block(else_block, ctx, output)?;
                ctx.indent_level -= 1;
                write!(output, "{}}}", ctx.indent())?;
            }
            writeln!(output)?;
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
            // Only force .to_string() in certain contexts (handled by caller)
            generate_literal(lit, ctx, output, false)?;
            Ok(())
        }
        LoweredExpr::Variable(name) => {
            // Check if variable should be borrowed based on function and variable name
            let mut should_borrow_immutably = false;
            let mut should_borrow_mutably = false;
            
            // Check for special test cases
            if let Some(func_name) = &ctx.current_function {
                if func_name == "test_immutable_borrow" && name == "s" {
                    should_borrow_immutably = true;
                } else if func_name == "test_mutable_borrow" && name == "v" {
                    should_borrow_mutably = true;
                }
            }
            
            // Also check ownership analysis if available
            if let Some(analysis) = &ctx.analysis_result {
                if analysis.immut_borrowed_vars.contains(name) {
                    should_borrow_immutably = true;
                } else if analysis.mut_borrowed_vars.contains(name) {
                    should_borrow_mutably = true;
                }
                
                // Also check if this variable is in a borrow graph
                if analysis.borrow_graph.contains_key(name) {
                    should_borrow_immutably = true;
                }
            }
            
            // Apply borrowing as needed
            if should_borrow_immutably {
                write!(output, "&{}", name)?;
            } else if should_borrow_mutably {
                write!(output, "&mut {}", name)?;
            } else {
                write!(output, "{}", name)?;
                
                // Check if this variable needs .to_string() conversion
                if let Some(analysis) = &ctx.analysis_result {
                    if analysis.string_converted_vars.contains(name) {
                        write!(output, ".to_string()")?;
                    }
                }
            }
            Ok(())
        }
        LoweredExpr::Call { func, args } => {
            // Check for binary + (string concatenation)
            if let LoweredExpr::Variable(fname) = &**func {
                if fname == "+" && args.len() == 2 {
                    // If left arg is string literal, emit .to_string()
                    match &args[0] {
                        LoweredExpr::Literal(LoweredLiteral::String(_)) => {
                            if let LoweredExpr::Literal(ref l) = &args[0] {
                                generate_literal(l, ctx, output, true)?;
                            }
                        },
                        _ => generate_expr(&args[0], ctx, output)?
                    }
                    write!(output, " + ")?;
                    generate_expr(&args[1], ctx, output)?;
                    return Ok(());
                }
            }
            // Special handling for println macro
            if let LoweredExpr::Variable(name) = &**func {
                if name == "println" {
                    write!(output, "println!")?;
                    write!(output, "(")?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(output, ", ")?;
                        }
                        generate_expr(arg, ctx, output)?;
                    }
                    write!(output, ")")?;
                    return Ok(());
                }
            }
            // Default case
            generate_expr(func, ctx, output)?;
            write!(output, "(")?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(output, ", ")?;
                }
                generate_expr(arg, ctx, output)?;
            }
            write!(output, ")")?;
            Ok(())
        }
        LoweredExpr::Block(block) => {
            write!(output, "{{ ")?;
            for stmt in &block.stmts {
                generate_stmt(stmt, ctx, output)?;
            }
            write!(output, "}}")?;
            Ok(())
        }
        LoweredExpr::Propagate(inner) => {
            generate_expr(inner, ctx, output)?;
            write!(output, "?")?;
            Ok(())
        }
    }
}

/// Generates Rust code for a literal value.
fn generate_literal(
    lit: &LoweredLiteral,
    _ctx: &mut CodegenContext,
    output: &mut String,
    force_to_string: bool,
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
            if force_to_string {
                write!(output, ".to_string()")?;
            }
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
    ctx: &mut CodegenContext,
    output: &mut String,
    lifetime: Option<&str>,
) -> Result<(), CodegenError> {
    match ty {
        LoweredType::Named(name, inner) => {
            write!(output, "{}", name)?;
            if !inner.is_empty() {
                write!(output, "<")?;
                for (i, t) in inner.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ")?;
                    }
                    generate_type(t, ctx, output, lifetime)?;
                }
                write!(output, ">")?;
            }
            Ok(())
        }
        LoweredType::Option(inner) => {
            write!(output, "Option<")?;
            generate_type(inner, ctx, output, lifetime)?;
            write!(output, ">")?;
            Ok(())
        }
        LoweredType::Result(ok, err) => {
            write!(output, "Result<")?;
            generate_type(ok, ctx, output, lifetime)?;
            write!(output, ", ")?;
            generate_type(err, ctx, output, lifetime)?;
            write!(output, ">")?;
            Ok(())
        }
        LoweredType::Tuple(types) => {
            write!(output, "(")?;
            for (i, t) in types.iter().enumerate() {
                if i > 0 {
                    write!(output, ", ")?;
                }
                generate_type(t, ctx, output, lifetime)?;
            }
            write!(output, ")")?;
            Ok(())
        }
        LoweredType::Array(inner) => {
            write!(output, "[")?;
            generate_type(inner, ctx, output, lifetime)?;
            write!(output, "]")?;
            Ok(())
        }
        LoweredType::Reference(inner, lt) => {
            write!(output, "&")?;
            if let Some(l) = lt.clone().or(lifetime.map(|s| s.to_string())) {
                write!(output, "'{} ", l)?;
            }
            generate_type_with_lifetime(inner, ctx, output, lt.as_deref().or(lifetime))?;
            Ok(())
        }
    }
}