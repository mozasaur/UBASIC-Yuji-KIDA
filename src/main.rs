//! UBASIC Rust - Main executable
//! 
//! This is the main entry point for the UBASIC Rust interpreter.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use ubasic_rust::{UBasic, UBasicResult};

#[derive(Parser)]
#[command(name = "ubasic-rust")]
#[command(about = "A modern Rust implementation of UBASIC")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input file to execute
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Code to evaluate
    #[arg(short, long)]
    eval: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Set precision for calculations
    #[arg(short, long, default_value = "64")]
    precision: u32,

    /// History file path
    #[arg(long, default_value = "~/.ubasic_history")]
    history_file: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive mode
    Interactive {
        /// Enable syntax highlighting
        #[arg(long)]
        syntax_highlighting: bool,
    },
    /// Run tests
    Test {
        /// Test file to run
        file: PathBuf,
    },
}

fn main() -> UBasicResult<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        env_logger::init();
    }

    // Create UBASIC engine with specified precision
    let mut ubasic = UBasic::with_precision(cli.precision);

    match &cli.command {
        Some(Commands::Interactive { syntax_highlighting }) => {
            println!("UBASIC Rust Interactive Mode");
            println!("Type 'help' for commands, 'exit' to quit");
            if *syntax_highlighting {
                println!("Syntax highlighting enabled");
            }
            
            // Run interactive mode
            ubasic.run_interactive()?;
        }
        Some(Commands::Test { file }) => {
            println!("Running tests from: {}", file.display());
            // TODO: Implement test runner
            println!("Test runner not yet implemented");
        }
        None => {
            // Handle file execution or code evaluation
            if let Some(file_path) = cli.file {
                println!("Executing file: {}", file_path.display());
                let code = std::fs::read_to_string(file_path)?;
                let result = ubasic.run(&code)?;
                println!("Result: {}", result);
            } else if let Some(code) = cli.eval {
                println!("Evaluating code: {}", code);
                let result = ubasic.run(&code)?;
                println!("Result: {}", result);
            } else {
                // Default to interactive mode
                println!("UBASIC Rust Interactive Mode");
                println!("Type 'help' for commands, 'exit' to quit");
                ubasic.run_interactive()?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_creation() {
        let args = vec!["ubasic-rust", "--precision", "128"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.precision, 128);
    }

    #[test]
    fn test_interactive_command() {
        let args = vec!["ubasic-rust", "interactive"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(cli.command, Some(Commands::Interactive { .. })));
    }
} 