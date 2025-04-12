use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Transpile { input, output } => {
            println!(
                "Transpile command invoked. Input: {}, Output: {:?}",
                input, output
            );
            // TODO: Call into highrust-transpiler library to perform transpilation
        }
        Commands::Version => {
            // This will print the version from Cargo.toml via clap
            println!("HighRust CLI version {}", env!("CARGO_PKG_VERSION"));
        }
    }
}