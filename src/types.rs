//! UBASIC data types
//! 
//! This module defines all the data types supported by UBASIC,
//! including integers, floats, complex numbers, strings, arrays, and more.

use rug::{Integer, Float, Complex};
use num_rational::Rational;
use nalgebra::{Matrix, Vector};
use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;

/// Main UBASIC value type that can hold any supported data type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UBasicValue {
    /// Integer (arbitrary precision)
    Integer(Integer),
    
    /// Floating point number (arbitrary precision)
    Float(Float),
    
    /// Complex number
    Complex(Complex<f64>),
    
    /// Rational number (fraction)
    Rational(Rational<i64>),
    
    /// String
    String(String),
    
    /// Array of values
    Array(Vec<UBasicValue>),
    
    /// Matrix
    Matrix(Matrix<f64>),
    
    /// Vector
    Vector(Vector<f64>),
    
    /// Boolean value
    Boolean(bool),
    
    /// Null/undefined value
    Null,
}

/// UBASIC data type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UBasicType {
    Integer,
    Float,
    Complex,
    Rational,
    String,
    Array,
    Matrix,
    Vector,
    Boolean,
    Null,
}

impl UBasicValue {
    /// Get the type of this value
    pub fn get_type(&self) -> UBasicType {
        match self {
            Self::Integer(_) => UBasicType::Integer,
            Self::Float(_) => UBasicType::Float,
            Self::Complex(_) => UBasicType::Complex,
            Self::Rational(_) => UBasicType::Rational,
            Self::String(_) => UBasicType::String,
            Self::Array(_) => UBasicType::Array,
            Self::Matrix(_) => UBasicType::Matrix,
            Self::Vector(_) => UBasicType::Vector,
            Self::Boolean(_) => UBasicType::Boolean,
            Self::Null => UBasicType::Null,
        }
    }

    /// Check if the value is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Integer(_) | Self::Float(_) | Self::Complex(_) | Self::Rational(_)
        )
    }

    /// Check if the value is zero
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Integer(i) => i == &Integer::ZERO,
            Self::Float(f) => f == &Float::new(64),
            Self::Complex(c) => c.norm_sqr() == 0.0,
            Self::Rational(r) => r.numer() == &0,
            Self::String(s) => s.is_empty(),
            Self::Array(a) => a.is_empty(),
            Self::Matrix(m) => m.is_empty(),
            Self::Vector(v) => v.is_empty(),
            Self::Boolean(b) => !b,
            Self::Null => true,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            Self::Integer(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Complex(c) => format!("{}+{}i", c.re, c.im),
            Self::Rational(r) => r.to_string(),
            Self::String(s) => s.clone(),
            Self::Array(a) => {
                let elements: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Self::Matrix(m) => format!("Matrix({}x{})", m.nrows(), m.ncols()),
            Self::Vector(v) => format!("Vector({})", v.len()),
            Self::Boolean(b) => b.to_string(),
            Self::Null => "null".to_string(),
        }
    }

    /// Get the length of the value (for arrays, strings, vectors)
    pub fn len(&self) -> Option<usize> {
        match self {
            Self::String(s) => Some(s.len()),
            Self::Array(a) => Some(a.len()),
            Self::Vector(v) => Some(v.len()),
            Self::Matrix(m) => Some(m.nrows() * m.ncols()),
            _ => None,
        }
    }

    /// Check if the value is empty
    pub fn is_empty(&self) -> bool {
        self.len().map(|l| l == 0).unwrap_or(true)
    }
}

impl fmt::Display for UBasicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<i32> for UBasicValue {
    fn from(value: i32) -> Self {
        Self::Integer(Integer::from(value))
    }
}

impl From<i64> for UBasicValue {
    fn from(value: i64) -> Self {
        Self::Integer(Integer::from(value))
    }
}

impl From<f64> for UBasicValue {
    fn from(value: f64) -> Self {
        Self::Float(Float::with_val(64, value))
    }
}

impl From<String> for UBasicValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for UBasicValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<bool> for UBasicValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<Vec<UBasicValue>> for UBasicValue {
    fn from(value: Vec<UBasicValue>) -> Self {
        Self::Array(value)
    }
}

/// Variable storage
#[derive(Debug, Clone)]
pub struct Variable {
    pub value: UBasicValue,
    pub is_constant: bool,
    pub line_defined: Option<usize>,
}

impl Variable {
    pub fn new(value: UBasicValue) -> Self {
        Self {
            value,
            is_constant: false,
            line_defined: None,
        }
    }

    pub fn constant(value: UBasicValue) -> Self {
        Self {
            value,
            is_constant: true,
            line_defined: None,
        }
    }
}

/// Array storage
#[derive(Debug, Clone)]
pub struct Array {
    pub dimensions: Vec<usize>,
    pub data: Vec<UBasicValue>,
}

impl Array {
    pub fn new(dimensions: Vec<usize>) -> Self {
        let total_size: usize = dimensions.iter().product();
        Self {
            dimensions,
            data: vec![UBasicValue::Null; total_size],
        }
    }

    pub fn get_index(&self, indices: &[isize]) -> Option<usize> {
        if indices.len() != self.dimensions.len() {
            return None;
        }

        let mut index = 0;
        let mut multiplier = 1;

        for (i, &dim) in self.dimensions.iter().enumerate().rev() {
            let idx = indices[i];
            if idx < 0 || idx >= dim as isize {
                return None;
            }
            index += (idx as usize) * multiplier;
            multiplier *= dim;
        }

        Some(index)
    }

    pub fn get(&self, indices: &[isize]) -> Option<&UBasicValue> {
        self.get_index(indices).map(|i| &self.data[i])
    }

    pub fn set(&mut self, indices: &[isize], value: UBasicValue) -> bool {
        if let Some(index) = self.get_index(indices) {
            self.data[index] = value;
            true
        } else {
            false
        }
    }
}

/// Function definition
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<String>, // Lines of BASIC code
    pub return_type: Option<UBasicType>,
    pub line_defined: usize,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, body: Vec<String>, line_defined: usize) -> Self {
        Self {
            name,
            parameters,
            body,
            return_type: None,
            line_defined,
        }
    }
}

/// Program state
#[derive(Debug, Clone)]
pub struct ProgramState {
    pub variables: HashMap<String, Variable>,
    pub arrays: HashMap<String, Array>,
    pub functions: HashMap<String, Function>,
    pub current_line: usize,
    pub program_lines: Vec<String>,
    pub call_stack: Vec<String>,
    pub return_values: Vec<UBasicValue>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            functions: HashMap::new(),
            current_line: 0,
            program_lines: Vec::new(),
            call_stack: Vec::new(),
            return_values: Vec::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: UBasicValue) {
        self.variables.insert(name, Variable::new(value));
    }

    pub fn get_variable(&self, name: &str) -> Option<&UBasicValue> {
        self.variables.get(name).map(|v| &v.value)
    }

    pub fn set_array(&mut self, name: String, array: Array) {
        self.arrays.insert(name, array);
    }

    pub fn get_array(&self, name: &str) -> Option<&Array> {
        self.arrays.get(name)
    }

    pub fn get_array_mut(&mut self, name: &str) -> Option<&mut Array> {
        self.arrays.get_mut(name)
    }

    pub fn clear(&mut self) {
        self.variables.clear();
        self.arrays.clear();
        self.functions.clear();
        self.current_line = 0;
        self.program_lines.clear();
        self.call_stack.clear();
        self.return_values.clear();
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_types() {
        let int_val = UBasicValue::Integer(Integer::from(42));
        assert_eq!(int_val.get_type(), UBasicType::Integer);
        assert!(int_val.is_numeric());
        assert!(!int_val.is_zero());

        let float_val = UBasicValue::Float(Float::with_val(64, 3.14));
        assert_eq!(float_val.get_type(), UBasicType::Float);
        assert!(float_val.is_numeric());

        let string_val = UBasicValue::String("hello".to_string());
        assert_eq!(string_val.get_type(), UBasicType::String);
        assert!(!string_val.is_numeric());
        assert_eq!(string_val.len(), Some(5));
    }

    #[test]
    fn test_array_operations() {
        let mut array = Array::new(vec![3, 3]); // 3x3 array
        assert_eq!(array.data.len(), 9);

        // Set value at [1, 2]
        let value = UBasicValue::Integer(Integer::from(42));
        assert!(array.set(&[1, 2], value.clone()));
        assert_eq!(array.get(&[1, 2]), Some(&value));

        // Invalid index
        assert!(array.get(&[5, 5]).is_none());
    }

    #[test]
    fn test_program_state() {
        let mut state = ProgramState::new();
        
        state.set_variable("x".to_string(), UBasicValue::Integer(Integer::from(10)));
        assert_eq!(state.get_variable("x"), Some(&UBasicValue::Integer(Integer::from(10))));
        
        let array = Array::new(vec![2, 2]);
        state.set_array("matrix".to_string(), array);
        assert!(state.get_array("matrix").is_some());
    }

    #[test]
    fn test_value_conversions() {
        let int_val: UBasicValue = 42.into();
        assert!(matches!(int_val, UBasicValue::Integer(_)));

        let float_val: UBasicValue = 3.14.into();
        assert!(matches!(float_val, UBasicValue::Float(_)));

        let string_val: UBasicValue = "hello".into();
        assert!(matches!(string_val, UBasicValue::String(_)));
    }
} 