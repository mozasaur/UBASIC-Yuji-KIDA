//! Console interface for UBASIC

use crate::errors::{UBasicError, UBasicResult};

/// Console interface for UBASIC
pub struct Console {
    prompt: String,
}

impl Console {
    /// Create a new console
    pub fn new() -> Self {
        Self {
            prompt: "> ".to_string(),
        }
    }

    /// Set the prompt
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    /// Print a message
    pub fn print(&self, message: &str) {
        println!("{}", message);
    }

    /// Print with newline
    pub fn println(&self, message: &str) {
        println!("{}", message);
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
} 