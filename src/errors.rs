//! Error types for UBASIC Rust
//! 
//! This module defines all error types that can occur during UBASIC execution,
//! including parsing errors, runtime errors, mathematical errors, and system errors.

use std::fmt;
use std::error::Error;

/// Result type for UBASIC operations
pub type UBasicResult<T> = Result<T, UBasicError>;

/// UBASIC error types
#[derive(Debug)]
pub enum UBasicError {
    /// Syntax error in the source code
    Syntax {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },
    /// Runtime error during execution
    Runtime {
        message: String,
        line: Option<usize>,
    },
    /// Type mismatch error
    TypeMismatch {
        expected: String,
        got: String,
        line: Option<usize>,
    },
    /// Variable not found error
    VariableNotFound {
        name: String,
        line: Option<usize>,
    },
    /// Function not found error
    FunctionNotFound {
        name: String,
        line: Option<usize>,
    },
    /// Division by zero error
    DivisionByZero {
        line: Option<usize>,
    },
    /// Overflow error
    Overflow {
        operation: String,
        line: Option<usize>,
    },
    /// Underflow error
    Underflow {
        operation: String,
        line: Option<usize>,
    },
    /// Mathematical error
    Math {
        message: String,
        line: Option<usize>,
    },
    /// File I/O error
    Io {
        message: String,
        line: Option<usize>,
    },
    /// Memory allocation error
    Memory {
        message: String,
        line: Option<usize>,
    },
    /// Invalid argument error
    InvalidArgument {
        message: String,
        line: Option<usize>,
    },
    /// Stack overflow error
    StackOverflow {
        line: Option<usize>,
    },
    /// Recursion limit exceeded
    RecursionLimit {
        line: Option<usize>,
    },
    /// Validation error
    Validation {
        message: String,
        line: Option<usize>,
    },
}

impl UBasicError {
    /// Create a syntax error
    pub fn syntax(message: impl Into<String>, line: Option<usize>, column: Option<usize>) -> Self {
        UBasicError::Syntax {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create a runtime error
    pub fn runtime(message: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::Runtime {
            message: message.into(),
            line,
        }
    }

    /// Create a type mismatch error
    pub fn type_mismatch(expected: impl Into<String>, got: impl Into<String>) -> Self {
        UBasicError::TypeMismatch {
            expected: expected.into(),
            got: got.into(),
            line: None,
        }
    }

    /// Create a variable not found error
    pub fn variable_not_found(name: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::VariableNotFound {
            name: name.into(),
            line,
        }
    }

    /// Create a function not found error
    pub fn function_not_found(name: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::FunctionNotFound {
            name: name.into(),
            line,
        }
    }

    /// Create a division by zero error
    pub fn division_by_zero(line: Option<usize>) -> Self {
        UBasicError::DivisionByZero { line }
    }

    /// Create an overflow error
    pub fn overflow(operation: impl Into<String>) -> Self {
        UBasicError::Overflow {
            operation: operation.into(),
            line: None,
        }
    }

    /// Create an underflow error
    pub fn underflow(operation: impl Into<String>) -> Self {
        UBasicError::Underflow {
            operation: operation.into(),
            line: None,
        }
    }

    /// Create a mathematical error
    pub fn math(message: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::Math {
            message: message.into(),
            line,
        }
    }

    /// Create an I/O error
    pub fn io(message: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::Io {
            message: message.into(),
            line,
        }
    }

    /// Create a memory error
    pub fn memory(message: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::Memory {
            message: message.into(),
            line,
        }
    }

    /// Create an invalid argument error
    pub fn invalid_argument(message: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::InvalidArgument {
            message: message.into(),
            line,
        }
    }

    /// Create a stack overflow error
    pub fn stack_overflow(line: Option<usize>) -> Self {
        UBasicError::StackOverflow { line }
    }

    /// Create a recursion limit error
    pub fn recursion_limit(line: Option<usize>) -> Self {
        UBasicError::RecursionLimit { line }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>, line: Option<usize>) -> Self {
        UBasicError::Validation {
            message: message.into(),
            line,
        }
    }

    /// Get the line number where the error occurred
    pub fn line(&self) -> Option<usize> {
        match self {
            UBasicError::Syntax { line, .. } => *line,
            UBasicError::Runtime { line, .. } => *line,
            UBasicError::TypeMismatch { line, .. } => *line,
            UBasicError::VariableNotFound { line, .. } => *line,
            UBasicError::FunctionNotFound { line, .. } => *line,
            UBasicError::DivisionByZero { line } => *line,
            UBasicError::Overflow { line, .. } => *line,
            UBasicError::Underflow { line, .. } => *line,
            UBasicError::Math { line, .. } => *line,
            UBasicError::Io { line, .. } => *line,
            UBasicError::Memory { line, .. } => *line,
            UBasicError::InvalidArgument { line, .. } => *line,
            UBasicError::StackOverflow { line } => *line,
            UBasicError::RecursionLimit { line } => *line,
            UBasicError::Validation { line, .. } => *line,
        }
    }

    /// Get the column number where the error occurred (only for syntax errors)
    pub fn column(&self) -> Option<usize> {
        match self {
            UBasicError::Syntax { column, .. } => *column,
            _ => None,
        }
    }
}

impl fmt::Display for UBasicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UBasicError::Syntax { message, line, column } => {
                if let (Some(line), Some(col)) = (line, column) {
                    write!(f, "Syntax error at line {}, column {}: {}", line, col, message)
                } else if let Some(line) = line {
                    write!(f, "Syntax error at line {}: {}", line, message)
                } else {
                    write!(f, "Syntax error: {}", message)
                }
            }
            UBasicError::Runtime { message, line } => {
                if let Some(line) = line {
                    write!(f, "Runtime error at line {}: {}", line, message)
                } else {
                    write!(f, "Runtime error: {}", message)
                }
            }
            UBasicError::TypeMismatch { expected, got, line } => {
                if let Some(line) = line {
                    write!(f, "Type mismatch at line {}: expected {}, got {}", line, expected, got)
                } else {
                    write!(f, "Type mismatch: expected {}, got {}", expected, got)
                }
            }
            UBasicError::VariableNotFound { name, line } => {
                if let Some(line) = line {
                    write!(f, "Variable '{}' not found at line {}", name, line)
                } else {
                    write!(f, "Variable '{}' not found", name)
                }
            }
            UBasicError::FunctionNotFound { name, line } => {
                if let Some(line) = line {
                    write!(f, "Function '{}' not found at line {}", name, line)
                } else {
                    write!(f, "Function '{}' not found", name)
                }
            }
            UBasicError::DivisionByZero { line } => {
                if let Some(line) = line {
                    write!(f, "Division by zero at line {}", line)
                } else {
                    write!(f, "Division by zero")
                }
            }
            UBasicError::Overflow { operation, line } => {
                if let Some(line) = line {
                    write!(f, "Overflow in {} at line {}", operation, line)
                } else {
                    write!(f, "Overflow in {}", operation)
                }
            }
            UBasicError::Underflow { operation, line } => {
                if let Some(line) = line {
                    write!(f, "Underflow in {} at line {}", operation, line)
                } else {
                    write!(f, "Underflow in {}", operation)
                }
            }
            UBasicError::Math { message, line } => {
                if let Some(line) = line {
                    write!(f, "Mathematical error at line {}: {}", line, message)
                } else {
                    write!(f, "Mathematical error: {}", message)
                }
            }
            UBasicError::Io { message, line } => {
                if let Some(line) = line {
                    write!(f, "I/O error at line {}: {}", line, message)
                } else {
                    write!(f, "I/O error: {}", message)
                }
            }
            UBasicError::Memory { message, line } => {
                if let Some(line) = line {
                    write!(f, "Memory error at line {}: {}", line, message)
                } else {
                    write!(f, "Memory error: {}", message)
                }
            }
            UBasicError::InvalidArgument { message, line } => {
                if let Some(line) = line {
                    write!(f, "Invalid argument at line {}: {}", line, message)
                } else {
                    write!(f, "Invalid argument: {}", message)
                }
            }
            UBasicError::StackOverflow { line } => {
                if let Some(line) = line {
                    write!(f, "Stack overflow at line {}", line)
                } else {
                    write!(f, "Stack overflow")
                }
            }
            UBasicError::RecursionLimit { line } => {
                if let Some(line) = line {
                    write!(f, "Recursion limit exceeded at line {}", line)
                } else {
                    write!(f, "Recursion limit exceeded")
                }
            }
            UBasicError::Validation { message, line } => {
                if let Some(line) = line {
                    write!(f, "Validation error at line {}: {}", line, message)
                } else {
                    write!(f, "Validation error: {}", message)
                }
            }
        }
    }
}

impl Error for UBasicError {}

/// Error context for providing additional information about errors
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
    fn test_syntax_error() {
        let error = UBasicError::syntax("Unexpected token", Some(10), Some(5));
        assert_eq!(error.line(), Some(10));
        assert_eq!(error.column(), Some(5));
        assert!(error.to_string().contains("Syntax error"));
    }

    #[test]
    fn test_runtime_error() {
        let error = UBasicError::runtime("Variable not initialized", Some(15));
        assert_eq!(error.line(), Some(15));
        assert!(error.to_string().contains("Runtime error"));
    }

    #[test]
    fn test_type_mismatch_error() {
        let error = UBasicError::type_mismatch("Integer", "String");
        assert!(error.to_string().contains("Type mismatch"));
        assert!(error.to_string().contains("Integer"));
        assert!(error.to_string().contains("String"));
    }

    #[test]
    fn test_division_by_zero_error() {
        let error = UBasicError::division_by_zero(Some(20));
        assert_eq!(error.line(), Some(20));
        assert!(error.to_string().contains("Division by zero"));
    }
} 
} 