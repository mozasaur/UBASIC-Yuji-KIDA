//! UBASIC Rust - Main executable
//! 
//! This is the main entry point for the UBASIC Rust interpreter.

use clap::{Parser, Subcommand};
use ubasic_rust::UBasic;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ubasic-rust")]
#[command(about = "A modern Rust implementation of UBASIC")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input file to execute
    #[arg(short, long)]
    file: Option<String>,

    /// Code to execute directly
    #[arg(short, long)]
    code: Option<String>,

    /// Run in interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// History file for interactive mode
    #[arg(long, default_value = "~/.ubasic_history")]
    history: Option<String>,

    /// Set precision for calculations
    #[arg(long, default_value = "64")]
    precision: Option<u32>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run in interactive mode
    Interactive {
        /// History file
        #[arg(long, default_value = "~/.ubasic_history")]
        history: Option<String>,
    },
    
    /// Execute a file
    Run {
        /// File to execute
        file: String,
        
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Execute code directly
    Eval {
        /// Code to execute
        code: String,
        
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show version and build information
    Version,
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    env_logger::init();

    // Create UBASIC instance with specified precision
    let precision = cli.precision.unwrap_or(64);
    let mut ubasic = UBasic::with_precision(precision);

    if cli.verbose {
        println!("UBASIC Rust v{}", env!("CARGO_PKG_VERSION"));
        println!("Precision: {} bits", precision);
        println!();
    }

    match &cli.command {
        Some(Commands::Interactive { history }) => {
            let history_file = history.clone().unwrap_or_else(|| "~/.ubasic_history".to_string());
            run_interactive(&mut ubasic, &history_file, cli.verbose);
        }
        Some(Commands::Run { file, verbose }) => {
            run_file(&mut ubasic, file, *verbose);
        }
        Some(Commands::Eval { code, verbose }) => {
            run_code(&mut ubasic, code, *verbose);
        }
        Some(Commands::Version) => {
            show_version();
        }
        None => {
            // Handle direct arguments
            if cli.interactive {
                let history_file = cli.history.unwrap_or_else(|| "~/.ubasic_history".to_string());
                run_interactive(&mut ubasic, &history_file, cli.verbose);
            } else if let Some(file) = cli.file {
                run_file(&mut ubasic, &file, cli.verbose);
            } else if let Some(code) = cli.code {
                run_code(&mut ubasic, &code, cli.verbose);
            } else {
                // Default to interactive mode
                let history_file = cli.history.unwrap_or_else(|| "~/.ubasic_history".to_string());
                run_interactive(&mut ubasic, &history_file, cli.verbose);
            }
        }
    }
}

fn run_interactive(ubasic: &mut UBasic, history_file: &str, verbose: bool) {
    if verbose {
        println!("Starting interactive mode with history file: {}", history_file);
    }

    // Expand tilde in history file path
    let history_path = if history_file.starts_with('~') {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        history_file.replacen('~', &home, 1)
    } else {
        history_file.to_string()
    };

    // Create history directory if it doesn't exist
    if let Some(parent) = PathBuf::from(&history_path).parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Warning: Could not create history directory: {}", e);
            }
        }
    }

    match ubasic.run_interactive() {
        Ok(_) => {
            if verbose {
                println!("Interactive session ended");
            }
        }
        Err(e) => {
            eprintln!("Error in interactive mode: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(ubasic: &mut UBasic, filename: &str, verbose: bool) {
    if verbose {
        println!("Executing file: {}", filename);
    }

    match std::fs::read_to_string(filename) {
        Ok(content) => {
            if verbose {
                println!("File loaded successfully ({} bytes)", content.len());
            }

            match ubasic.run(&content) {
                Ok(result) => {
                    if result != ubasic_rust::UBasicValue::Null {
                        println!("= {}", result);
                    }
                    if verbose {
                        println!("File executed successfully");
                    }
                }
                Err(e) => {
                    eprintln!("Error executing file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}

fn run_code(ubasic: &mut UBasic, code: &str, verbose: bool) {
    if verbose {
        println!("Executing code: {}", code);
    }

    match ubasic.run(code) {
        Ok(result) => {
            if result != ubasic_rust::UBasicValue::Null {
                println!("= {}", result);
            }
            if verbose {
                println!("Code executed successfully");
            }
        }
        Err(e) => {
            eprintln!("Error executing code: {}", e);
            std::process::exit(1);
        }
    }
}

fn show_version() {
    println!("UBASIC Rust v{}", env!("CARGO_PKG_VERSION"));
    println!("A modern Rust implementation of UBASIC");
    println!("Advanced BASIC interpreter with mathematical capabilities");
    println!();
    println!("Features:");
    println!("  • Arbitrary precision arithmetic");
    println!("  • Complex numbers and mathematical functions");
    println!("  • Graphics support (ggez, egui)");
    println!("  • Interactive console with syntax highlighting");
    println!("  • File I/O and data persistence");
    println!("  • Concurrent execution with async/await");
    println!();
    println!("Built with Rust {}", env!("RUST_VERSION"));
    println!("Target: {}", env!("TARGET"));
    println!("Build date: {}", env!("VERGEN_BUILD_TIMESTAMP"));
}

fn print_help() {
    println!("UBASIC Rust Commands:");
    println!("  help     - Show this help");
    println!("  clear    - Clear all variables");
    println!("  exit     - Exit the interpreter");
    println!("  quit     - Exit the interpreter");
    println!();
    println!("Examples:");
    println!("  LET x = 42");
    println!("  PRINT x");
    println!("  LET y = 2 + 3 * 4");
    println!("  PRINT \"Hello, World!\"");
} 