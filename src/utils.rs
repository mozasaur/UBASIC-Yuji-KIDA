//! Utility functions for UBASIC

use crate::types::UBasicValue;
use rug::Integer;

/// Convert a string to a UBASIC value
pub fn string_to_value(s: &str) -> Option<UBasicValue> {
    // Try to parse as integer first
    if let Ok(i) = s.parse::<i64>() {
        return Some(UBasicValue::Integer(Integer::from(i)));
    }
    
    // Try to parse as float
    if let Ok(f) = s.parse::<f64>() {
        return Some(UBasicValue::Float(rug::Float::with_val(64, f)));
    }
    
    // Return as string
    Some(UBasicValue::String(s.to_string()))
}

/// Format a UBASIC value as a string
pub fn value_to_string(value: &UBasicValue) -> String {
    value.to_string()
} 