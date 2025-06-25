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
pub struct UBasic {
    engine: UBasicEngine,
    math: MathEngine,
    memory: MemoryManager,
}

impl UBasic {
    /// Create a new UBASIC instance with default settings
    pub fn new() -> Self {
        Self {
            engine: UBasicEngine::new(),
            math: MathEngine::new(),
            memory: MemoryManager::new(),
        }
    }

    /// Create a new UBASIC instance with custom precision
    pub fn with_precision(precision: u32) -> Self {
        Self {
            engine: UBasicEngine::new(),
            math: MathEngine::with_precision(precision),
            memory: MemoryManager::new(),
        }
    }

    /// Run BASIC code and return the result
    pub fn run(&mut self, code: &str) -> UBasicResult<UBasicValue> {
        self.engine.run(code)
    }

    /// Run BASIC code in interactive mode
    pub fn run_interactive(&mut self) -> UBasicResult<()> {
        self.engine.run_interactive()
    }

    /// Get the current memory usage statistics
    pub fn memory_stats(&self) -> memory::MemoryStats {
        self.memory.stats()
    }

    /// Clear all variables and reset the interpreter
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
    fn test_basic_arithmetic() {
        let mut ubasic = UBasic::new();
        let result = ubasic.run("LET x = 2 + 3 * 4").unwrap();
        assert_eq!(result, UBasicValue::Integer(14.into()));
    }

    #[test]
    fn test_complex_math() {
        let mut ubasic = UBasic::new();
        let result = ubasic.run("LET z = (1 + 2i) * (3 + 4i)").unwrap();
        // Should be -5 + 10i
        assert!(matches!(result, UBasicValue::Complex(_)));
    }

    #[test]
    fn test_variables() {
        let mut ubasic = UBasic::new();
        ubasic.run("LET x = 42").unwrap();
        ubasic.run("LET y = x * 2").unwrap();
        let result = ubasic.run("PRINT y").unwrap();
        assert_eq!(result, UBasicValue::Integer(84.into()));
    }
} 