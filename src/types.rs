//! UBASIC data types
//! 
//! This module defines all the data types supported by UBASIC,
//! including integers, floats, complex numbers, strings, arrays, and more.

use rug::{Integer, Float, Complex, Assign};
use std::fmt;
use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd, Ordering};

/// Main UBASIC value type that can hold any supported data type
#[derive(Debug, Clone)]
pub enum UBasicValue {
    /// Integer (arbitrary precision)
    Integer(Integer),
    
    /// Floating point number (arbitrary precision)
    Float(Float),
    
    /// Complex number
    Complex(Complex),
    
    /// String
    String(String),
    
    /// Array of values
    Array(Vec<UBasicValue>),
    
    /// Matrix (2D array)
    Matrix(Vec<Vec<UBasicValue>>),
    
    /// Vector (1D array)
    Vector(Vec<UBasicValue>),
    
    /// Boolean value
    Boolean(bool),
    
    /// Null/undefined value
    Null,
}

/// UBASIC data type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UBasicType {
    Integer,
    Float,
    Complex,
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
            Self::Integer(_) | Self::Float(_) | Self::Complex(_)
        )
    }

    /// Check if the value is zero
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Integer(i) => i.is_zero(),
            Self::Float(f) => f.is_zero(),
            Self::Complex(c) => c.is_zero(),
            Self::String(s) => s.is_empty(),
            Self::Array(a) => a.is_empty(),
            Self::Matrix(m) => m.is_empty() || m[0].is_empty(),
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
            Self::Complex(c) => format!("{}+{}i", c.real_ref(), c.imag_ref()),
            Self::String(s) => s.clone(),
            Self::Array(a) => {
                let elements: Vec<String> = a.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Self::Matrix(m) => {
                let rows: Vec<String> = m.iter()
                    .map(|row| {
                        let elements: Vec<String> = row.iter().map(|v| v.to_string()).collect();
                        format!("[{}]", elements.join(", "))
                    })
                    .collect();
                format!("[{}]", rows.join(", "))
            }
            Self::Vector(v) => {
                let elements: Vec<String> = v.iter().map(|val| val.to_string()).collect();
                format!("<{}>", elements.join(", "))
            }
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
            Self::Matrix(m) => Some(m.len()),
            _ => None,
        }
    }

    /// Check if the value is empty
    pub fn is_empty(&self) -> bool {
        self.len().map(|l| l == 0).unwrap_or(true)
    }

    /// Get the type name of the value
    pub fn get_type_name(&self) -> &'static str {
        match self {
            UBasicValue::Integer(_) => "Integer",
            UBasicValue::Float(_) => "Float",
            UBasicValue::Complex(_) => "Complex",
            UBasicValue::String(_) => "String",
            UBasicValue::Boolean(_) => "Boolean",
            UBasicValue::Array(_) => "Array",
            UBasicValue::Matrix(_) => "Matrix",
            UBasicValue::Vector(_) => "Vector",
            UBasicValue::Null => "Null",
        }
    }

    /// Convert to integer if possible
    pub fn to_integer(&self) -> Option<Integer> {
        match self {
            UBasicValue::Integer(i) => Some(i.clone()),
            UBasicValue::Float(f) => f.to_integer().ok(),
            UBasicValue::Complex(c) => {
                if c.imag_ref().is_zero() {
                    c.real_ref().to_integer().ok()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Convert to float if possible
    pub fn to_float(&self) -> Option<Float> {
        match self {
            UBasicValue::Integer(i) => Some(Float::with_val(64, i)),
            UBasicValue::Float(f) => Some(f.clone()),
            UBasicValue::Complex(c) => {
                if c.imag_ref().is_zero() {
                    Some(Float::with_val(64, c.real_ref()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Convert to complex if possible
    pub fn to_complex(&self) -> Option<Complex> {
        match self {
            UBasicValue::Integer(i) => {
                let mut c = Complex::new(64);
                c.assign(i);
                Some(c)
            }
            UBasicValue::Float(f) => {
                let mut c = Complex::new(64);
                c.assign(f);
                Some(c)
            }
            UBasicValue::Complex(c) => Some(c.clone()),
            _ => None,
        }
    }

    /// Convert to boolean if possible
    pub fn to_boolean(&self) -> Option<bool> {
        match self {
            UBasicValue::Boolean(b) => Some(*b),
            UBasicValue::Integer(i) => Some(!i.is_zero()),
            UBasicValue::Float(f) => Some(!f.is_zero()),
            UBasicValue::Complex(c) => Some(!c.is_zero()),
            UBasicValue::String(s) => Some(!s.is_empty()),
            UBasicValue::Array(a) => Some(!a.is_empty()),
            UBasicValue::Matrix(m) => Some(!m.is_empty()),
            UBasicValue::Vector(v) => Some(!v.is_empty()),
            UBasicValue::Null => Some(false),
        }
    }
}

impl fmt::Display for UBasicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for UBasicValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a == b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a == b,
            (UBasicValue::Complex(a), UBasicValue::Complex(b)) => a == b,
            (UBasicValue::String(a), UBasicValue::String(b)) => a == b,
            (UBasicValue::Boolean(a), UBasicValue::Boolean(b)) => a == b,
            (UBasicValue::Array(a), UBasicValue::Array(b)) => a == b,
            (UBasicValue::Matrix(a), UBasicValue::Matrix(b)) => a == b,
            (UBasicValue::Vector(a), UBasicValue::Vector(b)) => a == b,
            (UBasicValue::Null, UBasicValue::Null) => true,
            _ => false,
        }
    }
}

impl PartialOrd for UBasicValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a.partial_cmp(b),
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a.partial_cmp(b),
            (UBasicValue::String(a), UBasicValue::String(b)) => a.partial_cmp(b),
            (UBasicValue::Boolean(a), UBasicValue::Boolean(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

// Arithmetic operations
impl Add for UBasicValue {
    type Output = UBasicValue;

    fn add(self, other: UBasicValue) -> UBasicValue {
        match (self, other) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => UBasicValue::Integer(a + b),
            (UBasicValue::Float(a), UBasicValue::Float(b)) => UBasicValue::Float(a + b),
            (UBasicValue::Complex(a), UBasicValue::Complex(b)) => UBasicValue::Complex(a + b),
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                let fa = Float::with_val(64, a);
                UBasicValue::Float(fa + b)
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                let fb = Float::with_val(64, b);
                UBasicValue::Float(a + fb)
            }
            (UBasicValue::Integer(a), UBasicValue::Complex(b)) => {
                let mut c = Complex::new(64);
                c.assign(a);
                UBasicValue::Complex(c + b)
            }
            (UBasicValue::Complex(a), UBasicValue::Integer(b)) => {
                let mut c = Complex::new(64);
                c.assign(b);
                UBasicValue::Complex(a + c)
            }
            (UBasicValue::Float(a), UBasicValue::Complex(b)) => {
                let mut c = Complex::new(64);
                c.assign(a);
                UBasicValue::Complex(c + b)
            }
            (UBasicValue::Complex(a), UBasicValue::Float(b)) => {
                let mut c = Complex::new(64);
                c.assign(b);
                UBasicValue::Complex(a + c)
            }
            (UBasicValue::String(a), UBasicValue::String(b)) => UBasicValue::String(a + &b),
            _ => UBasicValue::Null,
        }
    }
}

impl Sub for UBasicValue {
    type Output = UBasicValue;

    fn sub(self, other: UBasicValue) -> UBasicValue {
        match (self, other) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => UBasicValue::Integer(a - b),
            (UBasicValue::Float(a), UBasicValue::Float(b)) => UBasicValue::Float(a - b),
            (UBasicValue::Complex(a), UBasicValue::Complex(b)) => UBasicValue::Complex(a - b),
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                let fa = Float::with_val(64, a);
                UBasicValue::Float(fa - b)
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                let fb = Float::with_val(64, b);
                UBasicValue::Float(a - fb)
            }
            (UBasicValue::Integer(a), UBasicValue::Complex(b)) => {
                let mut c = Complex::new(64);
                c.assign(a);
                UBasicValue::Complex(c - b)
            }
            (UBasicValue::Complex(a), UBasicValue::Integer(b)) => {
                let mut c = Complex::new(64);
                c.assign(b);
                UBasicValue::Complex(a - c)
            }
            (UBasicValue::Float(a), UBasicValue::Complex(b)) => {
                let mut c = Complex::new(64);
                c.assign(a);
                UBasicValue::Complex(c - b)
            }
            (UBasicValue::Complex(a), UBasicValue::Float(b)) => {
                let mut c = Complex::new(64);
                c.assign(b);
                UBasicValue::Complex(a - c)
            }
            _ => UBasicValue::Null,
        }
    }
}

impl Mul for UBasicValue {
    type Output = UBasicValue;

    fn mul(self, other: UBasicValue) -> UBasicValue {
        match (self, other) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => UBasicValue::Integer(a * b),
            (UBasicValue::Float(a), UBasicValue::Float(b)) => UBasicValue::Float(a * b),
            (UBasicValue::Complex(a), UBasicValue::Complex(b)) => UBasicValue::Complex(a * b),
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                let fa = Float::with_val(64, a);
                UBasicValue::Float(fa * b)
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                let fb = Float::with_val(64, b);
                UBasicValue::Float(a * fb)
            }
            (UBasicValue::Integer(a), UBasicValue::Complex(b)) => {
                let mut c = Complex::new(64);
                c.assign(a);
                UBasicValue::Complex(c * b)
            }
            (UBasicValue::Complex(a), UBasicValue::Integer(b)) => {
                let mut c = Complex::new(64);
                c.assign(b);
                UBasicValue::Complex(a * c)
            }
            (UBasicValue::Float(a), UBasicValue::Complex(b)) => {
                let mut c = Complex::new(64);
                c.assign(a);
                UBasicValue::Complex(c * b)
            }
            (UBasicValue::Complex(a), UBasicValue::Float(b)) => {
                let mut c = Complex::new(64);
                c.assign(b);
                UBasicValue::Complex(a * c)
            }
            _ => UBasicValue::Null,
        }
    }
}

impl Div for UBasicValue {
    type Output = UBasicValue;

    fn div(self, other: UBasicValue) -> UBasicValue {
        match (self, other) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    UBasicValue::Integer(a / b)
                }
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    UBasicValue::Float(a / b)
                }
            }
            (UBasicValue::Complex(a), UBasicValue::Complex(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    UBasicValue::Complex(a / b)
                }
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let fa = Float::with_val(64, a);
                    UBasicValue::Float(fa / b)
                }
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let fb = Float::with_val(64, b);
                    UBasicValue::Float(a / fb)
                }
            }
            (UBasicValue::Integer(a), UBasicValue::Complex(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let mut c = Complex::new(64);
                    c.assign(a);
                    UBasicValue::Complex(c / b)
                }
            }
            (UBasicValue::Complex(a), UBasicValue::Integer(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let mut c = Complex::new(64);
                    c.assign(b);
                    UBasicValue::Complex(a / c)
                }
            }
            (UBasicValue::Float(a), UBasicValue::Complex(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let mut c = Complex::new(64);
                    c.assign(a);
                    UBasicValue::Complex(c / b)
                }
            }
            (UBasicValue::Complex(a), UBasicValue::Float(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let mut c = Complex::new(64);
                    c.assign(b);
                    UBasicValue::Complex(a / c)
                }
            }
            _ => UBasicValue::Null,
        }
    }
}

impl Rem for UBasicValue {
    type Output = UBasicValue;

    fn rem(self, other: UBasicValue) -> UBasicValue {
        match (self, other) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    UBasicValue::Integer(a % b)
                }
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    UBasicValue::Float(a % b)
                }
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let fa = Float::with_val(64, a);
                    UBasicValue::Float(fa % b)
                }
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                if b.is_zero() {
                    UBasicValue::Null
                } else {
                    let fb = Float::with_val(64, b);
                    UBasicValue::Float(a % fb)
                }
            }
            _ => UBasicValue::Null,
        }
    }
}

impl Neg for UBasicValue {
    type Output = UBasicValue;

    fn neg(self) -> UBasicValue {
        match self {
            UBasicValue::Integer(i) => UBasicValue::Integer(-i),
            UBasicValue::Float(f) => UBasicValue::Float(-f),
            UBasicValue::Complex(c) => UBasicValue::Complex(-c),
            _ => UBasicValue::Null,
        }
    }
}

// Assignment operations
impl AddAssign for UBasicValue {
    fn add_assign(&mut self, other: UBasicValue) {
        *self = self.clone() + other;
    }
}

impl SubAssign for UBasicValue {
    fn sub_assign(&mut self, other: UBasicValue) {
        *self = self.clone() - other;
    }
}

impl MulAssign for UBasicValue {
    fn mul_assign(&mut self, other: UBasicValue) {
        *self = self.clone() * other;
    }
}

impl DivAssign for UBasicValue {
    fn div_assign(&mut self, other: UBasicValue) {
        *self = self.clone() / other;
    }
}

impl RemAssign for UBasicValue {
    fn rem_assign(&mut self, other: UBasicValue) {
        *self = self.clone() % other;
    }
}

// From implementations for easy conversion
impl From<i32> for UBasicValue {
    fn from(value: i32) -> Self {
        UBasicValue::Integer(Integer::from(value))
    }
}

impl From<i64> for UBasicValue {
    fn from(value: i64) -> Self {
        UBasicValue::Integer(Integer::from(value))
    }
}

impl From<f64> for UBasicValue {
    fn from(value: f64) -> Self {
        UBasicValue::Float(Float::with_val(64, value))
    }
}

impl From<String> for UBasicValue {
    fn from(value: String) -> Self {
        UBasicValue::String(value)
    }
}

impl From<&str> for UBasicValue {
    fn from(value: &str) -> Self {
        UBasicValue::String(value.to_string())
    }
}

impl From<bool> for UBasicValue {
    fn from(value: bool) -> Self {
        UBasicValue::Boolean(value)
    }
}

impl From<Vec<UBasicValue>> for UBasicValue {
    fn from(value: Vec<UBasicValue>) -> Self {
        UBasicValue::Array(value)
    }
}

impl From<Vec<Vec<UBasicValue>>> for UBasicValue {
    fn from(value: Vec<Vec<UBasicValue>>) -> Self {
        UBasicValue::Matrix(value)
    }
}

// Default implementation
impl Default for UBasicValue {
    fn default() -> Self {
        UBasicValue::Null
    }
}

/// Variable storage with metadata
#[derive(Debug, Clone)]
pub struct Variable {
    pub value: UBasicValue,
    pub is_constant: bool,
    pub line_defined: Option<usize>,
}

impl Variable {
    /// Create a new variable
    pub fn new(value: UBasicValue) -> Self {
        Self {
            value,
            is_constant: false,
            line_defined: None,
        }
    }

    /// Create a constant variable
    pub fn constant(value: UBasicValue) -> Self {
        Self {
            value,
            is_constant: true,
            line_defined: None,
        }
    }
}

/// Array storage with multi-dimensional support
#[derive(Debug, Clone)]
pub struct Array {
    pub dimensions: Vec<usize>,
    pub data: Vec<UBasicValue>,
}

impl Array {
    /// Create a new array with given dimensions
    pub fn new(dimensions: Vec<usize>) -> Self {
        let total_size = dimensions.iter().product();
        Self {
            dimensions,
            data: vec![UBasicValue::Null; total_size],
        }
    }

    /// Get the linear index from multi-dimensional indices
    pub fn get_index(&self, indices: &[isize]) -> Option<usize> {
        if indices.len() != self.dimensions.len() {
            return None;
        }

        let mut index = 0;
        let mut stride = 1;

        for (i, &dim) in self.dimensions.iter().enumerate().rev() {
            let idx = indices[i] as usize;
            if idx >= dim {
                return None;
            }
            index += idx * stride;
            stride *= dim;
        }

        Some(index)
    }

    /// Get a value from the array
    pub fn get(&self, indices: &[isize]) -> Option<&UBasicValue> {
        self.get_index(indices).map(|i| &self.data[i])
    }

    /// Set a value in the array
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
    /// Create a new function
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

/// Program execution state
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
    /// Create a new program state
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

    /// Set a variable
    pub fn set_variable(&mut self, name: String, value: UBasicValue) {
        self.variables.insert(name, Variable::new(value));
    }

    /// Get a variable
    pub fn get_variable(&self, name: &str) -> Option<&UBasicValue> {
        self.variables.get(name).map(|v| &v.value)
    }

    /// Set an array
    pub fn set_array(&mut self, name: String, array: Array) {
        self.arrays.insert(name, array);
    }

    /// Get an array
    pub fn get_array(&self, name: &str) -> Option<&Array> {
        self.arrays.get(name)
    }

    /// Get a mutable reference to an array
    pub fn get_array_mut(&mut self, name: &str) -> Option<&mut Array> {
        self.arrays.get_mut(name)
    }

    /// Clear all state
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
        let int_val = UBasicValue::from(42);
        let float_val = UBasicValue::from(3.14);
        let string_val = UBasicValue::from("hello");
        let bool_val = UBasicValue::from(true);

        assert!(matches!(int_val, UBasicValue::Integer(_)));
        assert!(matches!(float_val, UBasicValue::Float(_)));
        assert!(matches!(string_val, UBasicValue::String(_)));
        assert!(matches!(bool_val, UBasicValue::Boolean(_)));
    }

    #[test]
    fn test_array_operations() {
        let mut array = Array::new(vec![2, 3]);
        let value = UBasicValue::from(42);
        
        assert!(array.set(&[0, 1], value.clone()));
        assert_eq!(array.get(&[0, 1]), Some(&value));
    }

    #[test]
    fn test_program_state() {
        let mut state = ProgramState::new();
        state.set_variable("x".to_string(), UBasicValue::from(42));
        
        assert_eq!(state.get_variable("x"), Some(&UBasicValue::from(42)));
    }

    #[test]
    fn test_value_conversions() {
        let int_val = UBasicValue::from(42);
        let float_val = UBasicValue::from(3.14);
        
        assert!(int_val.to_integer().is_some());
        assert!(float_val.to_float().is_some());
    }
} 