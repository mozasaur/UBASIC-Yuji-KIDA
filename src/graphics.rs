//! Graphics support for UBASIC

use crate::errors::{UBasicError, UBasicResult};
use crate::types::UBasicValue;

/// Graphics engine for UBASIC
pub struct GraphicsEngine {
    width: u32,
    height: u32,
    color: u32,
}

impl GraphicsEngine {
    /// Create a new graphics engine
    pub fn new() -> Self {
        Self {
            width: 640,
            height: 480,
            color: 0xFFFFFF, // White
        }
    }

    /// Set screen resolution
    pub fn set_screen(&mut self, width: u32, height: u32) -> UBasicResult<()> {
        self.width = width;
        self.height = height;
        Ok(())
    }

    /// Set drawing color
    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }
}

impl Default for GraphicsEngine {
    fn default() -> Self {
        Self::new()
    }
} 