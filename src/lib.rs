//! UBASIC Rust - A modern implementation of UBASIC
//! 
//! This crate provides a complete BASIC interpreter with advanced mathematical capabilities,
//! graphics support, and modern Rust features.
//! 
//! # Features
//! 
//! - **Arbitrary precision arithmetic** using `rug`
//! - **Complex numbers** and mathematical functions
//! - **Graphics support** with ggez and egui
//! - **Interactive console** with syntax highlighting
//! - **File I/O** and data persistence
//! - **Concurrent execution** with async/await
//! 
//! # Quick Start
//! 
//! ```rust
//! use ubasic_rust::UBasicEngine;
//! 
//! let mut engine = UBasicEngine::new();
//! let result = engine.run("PRINT 2 + 2").unwrap();
//! ```
//! 
//! # Examples
//! 
//! ## Mathematical calculations
//! 
//! ```rust
//! let code = r#"
//! LET x = 3.14159
//! LET y = SIN(x)
//! PRINT "sin(π) =", y
//! "#;
//! 
//! engine.run(code).unwrap();
//! ```
//! 
//! ## Graphics
//! 
//! ```rust
//! let code = r#"
//! SCREEN 640, 480
//! LINE 0, 0, 100, 100
//! CIRCLE 200, 200, 50
//! "#;
//! 
//! engine.run(code).unwrap();
//! ```
//! 
//! ## Interactive mode
//! 
//! ```rust
//! use ubasic_rust::UBasic;
//! 
//! let mut ubasic = UBasic::new();
//! ubasic.run_interactive().unwrap();
//! ```
//! 
//! ## Error handling
//! 
//! ```rust
//! use ubasic_rust::{UBasic, UBasicResult};
//! 
//! let mut ubasic = UBasic::new();
//! 
//! match ubasic.run("LET x = 1 / 0") {
//!     Ok(result) => println!("Result: {}", result),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//! 
//! ## Custom precision
//! 
//! ```rust
//! let mut ubasic = UBasic::with_precision(128);
//! let result = ubasic.run("PRINT PI").unwrap();
//! ```
//! 
//! ## Memory management
//! 
//! ```rust
//! let mut ubasic = UBasic::new();
//! 
//! // Create variables
//! ubasic.run("LET x = 42").unwrap();
//! ubasic.run("LET y = 3.14").unwrap();
//! 
//! // Check memory usage
//! let stats = ubasic.memory_stats();
//! println!("Variables: {}", stats.total_variables);
//! 
//! // Clear memory
//! ubasic.clear();
//! ```

pub mod types;
pub mod parser;
pub mod interpreter;
pub mod math;
pub mod memory;
pub mod graphics;
pub mod console;
pub mod errors;
pub mod utils;

// Re-export main types for convenience
pub use types::{UBasicValue, UBasicType};
pub use interpreter::UBasicEngine;
pub use errors::{UBasicError, UBasicResult};
pub use math::MathEngine;
pub use memory::MemoryManager;

/// Main UBASIC engine that coordinates all components
/// 
/// This is the primary interface for running UBASIC programs.
/// It combines the parser, interpreter, math engine, and memory manager
/// into a single, easy-to-use interface.
/// 
/// # Examples
/// 
/// ```rust
/// use ubasic_rust::UBasic;
/// 
/// let mut ubasic = UBasic::new();
/// let result = ubasic.run("PRINT \"Hello, World!\"").unwrap();
/// ```
/// 
/// ```rust
/// // With custom precision
/// let mut ubasic = UBasic::with_precision(128);
/// let result = ubasic.run("PRINT PI").unwrap();
/// ```
pub struct UBasic {
    engine: UBasicEngine,
    math: MathEngine,
    memory: MemoryManager,
}

impl UBasic {
    /// Create a new UBASIC instance with default settings
    /// 
    /// Uses 64-bit precision for mathematical calculations.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ubasic_rust::UBasic;
    /// 
    /// let ubasic = UBasic::new();
    /// ```
    pub fn new() -> Self {
        Self {
            engine: UBasicEngine::new(),
            math: MathEngine::new(),
            memory: MemoryManager::new(),
        }
    }

    /// Create a new UBASIC instance with custom precision
    /// 
    /// # Arguments
    /// 
    /// * `precision` - The number of bits of precision for mathematical calculations
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ubasic_rust::UBasic;
    /// 
    /// let ubasic = UBasic::with_precision(128);
    /// ```
    pub fn with_precision(precision: u32) -> Self {
        Self {
            engine: UBasicEngine::new(),
            math: MathEngine::with_precision(precision),
            memory: MemoryManager::new(),
        }
    }

    /// Run BASIC code and return the result
    /// 
    /// # Arguments
    /// 
    /// * `code` - The BASIC code to execute
    /// 
    /// # Returns
    /// 
    /// Returns the result of the last expression evaluated, or an error if execution fails.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ubasic_rust::UBasic;
    /// 
    /// let mut ubasic = UBasic::new();
    /// let result = ubasic.run("LET x = 2 + 2").unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The code contains syntax errors
    /// - A runtime error occurs (e.g., division by zero)
    /// - A variable is not found
    /// - Memory allocation fails
    pub fn run(&mut self, code: &str) -> UBasicResult<UBasicValue> {
        self.engine.run(code)
    }

    /// Run BASIC code in interactive mode
    /// 
    /// This starts an interactive REPL (Read-Eval-Print Loop) where users can
    /// enter BASIC commands one at a time.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use ubasic_rust::UBasic;
    /// 
    /// let mut ubasic = UBasic::new();
    /// ubasic.run_interactive().unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The console cannot be initialized
    /// - Input/output operations fail
    pub fn run_interactive(&mut self) -> UBasicResult<()> {
        self.engine.run_interactive()
    }

    /// Get the current memory usage statistics
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ubasic_rust::UBasic;
    /// 
    /// let mut ubasic = UBasic::new();
    /// ubasic.run("LET x = 42").unwrap();
    /// 
    /// let stats = ubasic.memory_stats();
    /// println!("Total variables: {}", stats.total_variables);
    /// ```
    pub fn memory_stats(&self) -> memory::MemoryStats {
        self.memory.stats()
    }

    /// Clear all variables and reset the interpreter
    /// 
    /// This removes all variables, arrays, and functions from memory,
    /// effectively resetting the interpreter to its initial state.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ubasic_rust::UBasic;
    /// 
    /// let mut ubasic = UBasic::new();
    /// ubasic.run("LET x = 42").unwrap();
    /// 
    /// // Clear all variables
    /// ubasic.clear();
    /// 
    /// // x is no longer defined
    /// assert!(ubasic.run("PRINT x").is_err());
    /// ```
    pub fn clear(&mut self) {
        self.memory.clear();
        self.engine.clear();
    }
}

impl Default for UBasic {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_creation() {
        let ubasic = UBasic::new();
        assert!(ubasic.memory_stats().total_variables == 0);
    }

    #[test]
    fn test_precision_creation() {
        let ubasic = UBasic::with_precision(128);
        assert!(ubasic.memory_stats().total_variables == 0);
    }

    #[test]
    fn test_clear_functionality() {
        let mut ubasic = UBasic::new();
        ubasic.clear();
        assert!(ubasic.memory_stats().total_variables == 0);
    }
} 