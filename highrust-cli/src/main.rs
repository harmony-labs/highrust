use clap::{Parser, Subcommand};

mod watcher;

use highrust_transpiler::{transpile_file, transpile_source, TranspilerError};
use std::process;
use std::fs;

/// HighRust Transpiler CLI
#[derive(Parser)]
#[command(
    name = "highrust",
    version,
    author,
    about = "Command-line interface for the HighRust transpiler"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Transpile a HighRust source file to Rust
    Transpile {
        /// Path to the input .hrs file
        #[arg(short, long)]
        input: String,
        /// Path to the output .rs file
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Print version information
    Version,
    /// Watch source files and trigger transpilation on changes (scaffold)
    Watch {
        /// Path to the source directory or file to watch
        #[arg(short, long)]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Transpile { input, output } => {
            println!(
                "Transpile command invoked. Input: {}, Output: {:?}",
                input, output
            );
                            // Run the transpiler pipeline
                            let input_path = input;
                            match output {
                                Some(output_path) => {
                                    // Output to file
                                    match transpile_file(input_path, output_path) {
                                        Ok(()) => {
                                            println!(
                                                "Transpilation succeeded. Rust code written to '{}'.",
                                                output_path
                                            );
                                        }
                                        Err(e) => {
                                            eprintln!("Transpilation failed: {}", format_transpiler_error(&e));
                                            process::exit(1);
                                        }
                                    }
                                }
                                None => {
                                    // Output to stdout
                                    match fs::read_to_string(input_path) {
                                        Ok(source) => {
                                            match transpile_source(&source) {
                                                Ok(rust_code) => {
                                                    println!("{}", rust_code);
                                                }
                                                Err(e) => {
                                                    eprintln!("Transpilation failed: {}", format_transpiler_error(&e));
                                                    process::exit(1);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to read input file '{}': {}", input_path, e);
                                            process::exit(1);
                                        }
                                    }
                                }
                            }
                            
                            /// Formats a TranspilerError for user-friendly output.
                            fn format_transpiler_error(e: &TranspilerError) -> String {
                                match *e {
                                    TranspilerError::ParseError(ref msg) => format!("Parse error: {}", msg),
                                    TranspilerError::LoweringError(ref le) => format!("Lowering error: {:?}", le),
                                    TranspilerError::CodegenError(ref ce) => format!("Codegen error: {:?}", ce),
                                    TranspilerError::OwnershipError(ref oe) => format!("Ownership error: {:?}", oe),
                                    TranspilerError::IoError(ref ioe) => format!("I/O error: {}", ioe),
                                }
                            }
        }
        Commands::Version => {
            // This will print the version from Cargo.toml via clap
            println!("HighRust CLI version {}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Watch { path } => {
            println!(
                "Watch command scaffold invoked. Path: {}",
                path
            );
            // This is a scaffold for the file watcher.
            // When fully implemented, this will start watching the given path for changes
            // and trigger transpilation as needed.
            // See watcher.rs for the watcher implementation.
            let mut watcher = watcher::Watcher::new(/* In the future: vec![PathBuf::from(path)] */);
            let _ = watcher.watch();
        }
    }
}