//! UBASIC Rust - Main executable
//! 
//! This is the main entry point for the UBASIC Rust interpreter.

use clap::{Parser, Subcommand};
use ubasic_rust::UBasic;

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
}

#[derive(Subcommand)]
enum Commands {
    /// Run in interactive mode
    Interactive,
    
    /// Execute a file
    Run {
        /// File to execute
        file: String,
    },
    
    /// Execute code directly
    Eval {
        /// Code to execute
        code: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    env_logger::init();

    let mut ubasic = UBasic::new();

    match &cli.command {
        Some(Commands::Interactive) => {
            run_interactive(&mut ubasic);
        }
        Some(Commands::Run { file }) => {
            run_file(&mut ubasic, file);
        }
        Some(Commands::Eval { code }) => {
            run_code(&mut ubasic, code);
        }
        None => {
            // Handle direct arguments
            if cli.interactive {
                run_interactive(&mut ubasic);
            } else if let Some(file) = cli.file {
                run_file(&mut ubasic, &file);
            } else if let Some(code) = cli.code {
                run_code(&mut ubasic, &code);
            } else {
                // Default to interactive mode
                run_interactive(&mut ubasic);
            }
        }
    }
}

fn run_interactive(ubasic: &mut UBasic) {
    println!("UBASIC Rust Interactive Mode");
    println!("Type 'exit' to quit, 'help' for help");
    println!();

    // Simple interactive loop
    loop {
        print!("> ");
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        if input == "help" {
            print_help();
            continue;
        }

        if input == "clear" {
            ubasic.clear();
            println!("Memory cleared");
            continue;
        }

        match ubasic.run(input) {
            Ok(result) => {
                if result != ubasic_rust::UBasicValue::Null {
                    println!("= {}", result);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn run_file(ubasic: &mut UBasic, filename: &str) {
    match std::fs::read_to_string(filename) {
        Ok(content) => {
            match ubasic.run(&content) {
                Ok(result) => {
                    if result != ubasic_rust::UBasicValue::Null {
                        println!("= {}", result);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
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

fn run_code(ubasic: &mut UBasic, code: &str) {
    match ubasic.run(code) {
        Ok(result) => {
            if result != ubasic_rust::UBasicValue::Null {
                println!("= {}", result);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
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