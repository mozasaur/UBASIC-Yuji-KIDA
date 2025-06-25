//! Memory management for UBASIC

use crate::types::{UBasicValue, ProgramState};
use std::collections::HashMap;

/// Memory manager for UBASIC
pub struct MemoryManager {
    state: ProgramState,
    max_variables: usize,
    max_arrays: usize,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub variables_count: usize,
    pub arrays_count: usize,
    pub total_memory_used: usize,
    pub max_variables: usize,
    pub max_arrays: usize,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            state: ProgramState::new(),
            max_variables: 1000,
            max_arrays: 100,
        }
    }

    /// Get memory usage statistics
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            variables_count: self.state.variables.len(),
            arrays_count: self.state.arrays.len(),
            total_memory_used: self.state.variables.len() + self.state.arrays.len(),
            max_variables: self.max_variables,
            max_arrays: self.max_arrays,
        }
    }

    /// Clear all memory
    pub fn clear(&mut self) {
        self.state.clear();
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
} 