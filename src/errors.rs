//! Error types for UBASIC Rust
//! 
//! This module defines all error types that can occur during UBASIC execution,
//! including parsing errors, runtime errors, mathematical errors, and system errors.

use thiserror::Error;
use std::fmt;

/// Result type for UBASIC operations
pub type UBasicResult<T> = Result<T, UBasicError>;

/// Main error type for UBASIC
#[derive(Error, Debug, Clone)]
pub enum UBasicError {
    /// Syntax error during parsing
    #[error("Syntax error: {message} at line {line}, column {column}")]
    Syntax {
        message: String,
        line: usize,
        column: usize,
    },

    /// Runtime error during execution
    #[error("Runtime error: {message}")]
    Runtime {
        message: String,
        line: Option<usize>,
    },

    /// Mathematical error (division by zero, overflow, etc.)
    #[error("Math error: {message}")]
    Math {
        message: String,
        operation: Option<String>,
    },

    /// Memory allocation or management error
    #[error("Memory error: {message}")]
    Memory {
        message: String,
        requested: Option<usize>,
        available: Option<usize>,
    },

    /// Variable not found
    #[error("Variable '{name}' not found")]
    VariableNotFound {
        name: String,
    },

    /// Type mismatch error
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        expected: String,
        actual: String,
    },

    /// File I/O error
    #[error("File error: {message}")]
    File {
        message: String,
        path: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Graphics error
    #[error("Graphics error: {message}")]
    Graphics {
        message: String,
        operation: Option<String>,
    },

    /// System error (OS-level errors)
    #[error("System error: {message}")]
    System {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Stack overflow
    #[error("Stack overflow: maximum depth {max_depth} exceeded")]
    StackOverflow {
        max_depth: usize,
        current_depth: usize,
    },

    /// Recursion limit exceeded
    #[error("Recursion limit exceeded: maximum {max_depth} levels")]
    RecursionLimit {
        max_depth: usize,
        current_depth: usize,
    },

    /// Invalid function call
    #[error("Invalid function call: {function_name}({args})")]
    InvalidFunctionCall {
        function_name: String,
        args: String,
        expected_args: usize,
        actual_args: usize,
    },

    /// Array bounds error
    #[error("Array bounds error: index {index} out of bounds [0, {max_index})")]
    ArrayBounds {
        index: isize,
        max_index: usize,
        array_name: String,
    },

    /// Division by zero
    #[error("Division by zero")]
    DivisionByZero,

    /// Overflow error
    #[error("Overflow error: {operation}")]
    Overflow {
        operation: String,
    },

    /// Underflow error
    #[error("Underflow error: {operation}")]
    Underflow {
        operation: String,
    },

    /// Invalid number format
    #[error("Invalid number format: '{input}'")]
    InvalidNumber {
        input: String,
    },

    /// Invalid string operation
    #[error("Invalid string operation: {message}")]
    InvalidString {
        message: String,
    },

    /// Compilation error
    #[error("Compilation error: {message}")]
    Compilation {
        message: String,
        line: Option<usize>,
    },

    /// Linking error
    #[error("Linking error: {message}")]
    Linking {
        message: String,
    },

    /// Internal error (should not occur in normal operation)
    #[error("Internal error: {message}")]
    Internal {
        message: String,
    },
}

impl UBasicError {
    /// Create a syntax error
    pub fn syntax(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::Syntax {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create a runtime error
    pub fn runtime(message: impl Into<String>, line: Option<usize>) -> Self {
        Self::Runtime {
            message: message.into(),
            line,
        }
    }

    /// Create a math error
    pub fn math(message: impl Into<String>, operation: Option<String>) -> Self {
        Self::Math {
            message: message.into(),
            operation,
        }
    }

    /// Create a variable not found error
    pub fn variable_not_found(name: impl Into<String>) -> Self {
        Self::VariableNotFound {
            name: name.into(),
        }
    }

    /// Create a type mismatch error
    pub fn type_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::TypeMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Create a file error
    pub fn file(message: impl Into<String>, path: Option<String>) -> Self {
        Self::File {
            message: message.into(),
            path,
            source: None,
        }
    }

    /// Create a graphics error
    pub fn graphics(message: impl Into<String>, operation: Option<String>) -> Self {
        Self::Graphics {
            message: message.into(),
            operation,
        }
    }

    /// Create a system error
    pub fn system(message: impl Into<String>) -> Self {
        Self::System {
            message: message.into(),
            source: None,
        }
    }

    /// Create a stack overflow error
    pub fn stack_overflow(max_depth: usize, current_depth: usize) -> Self {
        Self::StackOverflow {
            max_depth,
            current_depth,
        }
    }

    /// Create a recursion limit error
    pub fn recursion_limit(max_depth: usize, current_depth: usize) -> Self {
        Self::RecursionLimit {
            max_depth,
            current_depth,
        }
    }

    /// Create an invalid function call error
    pub fn invalid_function_call(
        function_name: impl Into<String>,
        args: impl Into<String>,
        expected_args: usize,
        actual_args: usize,
    ) -> Self {
        Self::InvalidFunctionCall {
            function_name: function_name.into(),
            args: args.into(),
            expected_args,
            actual_args,
        }
    }

    /// Create an array bounds error
    pub fn array_bounds(index: isize, max_index: usize, array_name: impl Into<String>) -> Self {
        Self::ArrayBounds {
            index,
            max_index,
            array_name: array_name.into(),
        }
    }

    /// Create an overflow error
    pub fn overflow(operation: impl Into<String>) -> Self {
        Self::Overflow {
            operation: operation.into(),
        }
    }

    /// Create an underflow error
    pub fn underflow(operation: impl Into<String>) -> Self {
        Self::Underflow {
            operation: operation.into(),
        }
    }

    /// Create an invalid number error
    pub fn invalid_number(input: impl Into<String>) -> Self {
        Self::InvalidNumber {
            input: input.into(),
        }
    }

    /// Create an invalid string error
    pub fn invalid_string(message: impl Into<String>) -> Self {
        Self::InvalidString {
            message: message.into(),
        }
    }

    /// Create a compilation error
    pub fn compilation(message: impl Into<String>, line: Option<usize>) -> Self {
        Self::Compilation {
            message: message.into(),
            line,
        }
    }

    /// Create a linking error
    pub fn linking(message: impl Into<String>) -> Self {
        Self::Linking {
            message: message.into(),
        }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        match self {
            Self::Syntax { message, .. } => message,
            Self::Runtime { message, .. } => message,
            Self::Math { message, .. } => message,
            Self::Memory { message, .. } => message,
            Self::VariableNotFound { name } => name,
            Self::TypeMismatch { expected, actual } => {
                // Return a combined message
                expected
            }
            Self::File { message, .. } => message,
            Self::Graphics { message, .. } => message,
            Self::System { message, .. } => message,
            Self::StackOverflow { .. } => "Stack overflow",
            Self::RecursionLimit { .. } => "Recursion limit exceeded",
            Self::InvalidFunctionCall { function_name, .. } => function_name,
            Self::ArrayBounds { .. } => "Array bounds error",
            Self::DivisionByZero => "Division by zero",
            Self::Overflow { operation } => operation,
            Self::Underflow { operation } => operation,
            Self::InvalidNumber { input } => input,
            Self::InvalidString { message } => message,
            Self::Compilation { message, .. } => message,
            Self::Linking { message } => message,
            Self::Internal { message } => message,
        }
    }

    /// Check if this is a fatal error (cannot be recovered from)
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::Internal { .. } | Self::System { .. } | Self::Memory { .. }
        )
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        !self.is_fatal()
    }
}

impl fmt::Display for UBasicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, actual)
            }
            _ => write!(f, "{}", self.message()),
        }
    }
}

/// Error context for better error reporting
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub source_line: Option<String>,
    pub stack_trace: Vec<String>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            line: None,
            column: None,
            source_line: None,
            stack_trace: Vec::new(),
        }
    }

    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_source_line(mut self, source_line: String) -> Self {
        self.source_line = Some(source_line);
        self
    }

    pub fn add_stack_frame(mut self, frame: String) -> Self {
        self.stack_trace.push(frame);
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let syntax_error = UBasicError::syntax("Unexpected token", 10, 5);
        assert!(matches!(syntax_error, UBasicError::Syntax { .. }));

        let runtime_error = UBasicError::runtime("Variable not found", Some(15));
        assert!(matches!(runtime_error, UBasicError::Runtime { .. }));

        let math_error = UBasicError::math("Division by zero", Some("DIV".to_string()));
        assert!(matches!(math_error, UBasicError::Math { .. }));
    }

    #[test]
    fn test_error_properties() {
        let internal_error = UBasicError::internal("Critical failure");
        assert!(internal_error.is_fatal());
        assert!(!internal_error.is_recoverable());

        let syntax_error = UBasicError::syntax("Missing semicolon", 1, 1);
        assert!(!syntax_error.is_fatal());
        assert!(syntax_error.is_recoverable());
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new()
            .with_line(10)
            .with_column(5)
            .with_source_line("LET x = 2 + 3".to_string())
            .add_stack_frame("main()".to_string());

        assert_eq!(context.line, Some(10));
        assert_eq!(context.column, Some(5));
        assert_eq!(context.stack_trace.len(), 1);
    }
} 