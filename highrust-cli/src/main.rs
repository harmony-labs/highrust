use clap::{Parser, Subcommand};

mod watcher;

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
            // TODO: Call into highrust-transpiler library to perform transpilation
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