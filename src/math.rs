//! Mathematical functions for UBASIC
//! 
//! This module provides mathematical functions like trigonometric, exponential,
//! logarithmic, and other mathematical operations.

use crate::errors::{UBasicError, UBasicResult};
use crate::types::UBasicValue;
use rug::{Integer, Float};
use num_complex::Complex;

/// Mathematical engine for UBASIC
pub struct MathEngine {
    precision: u32,
}

impl MathEngine {
    /// Create a new math engine with default precision
    pub fn new() -> Self {
        Self {
            precision: 64,
        }
    }

    /// Create a new math engine with custom precision
    pub fn with_precision(precision: u32) -> Self {
        Self { precision }
    }

    /// Set the precision for calculations
    pub fn set_precision(&mut self, precision: u32) {
        self.precision = precision;
    }

    /// Get the current precision
    pub fn get_precision(&self) -> u32 {
        self.precision
    }

    /// Calculate the sine of a value
    pub fn sin(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.as_ref().sin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.sin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.sin();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the cosine of a value
    pub fn cos(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.as_ref().cos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.cos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.cos();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the tangent of a value
    pub fn tan(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.as_ref().tan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.tan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.tan();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the arcsine of a value
    pub fn asin(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.as_ref().asin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.asin();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.asin();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the arccosine of a value
    pub fn acos(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.as_ref().acos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.acos();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.acos();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the arctangent of a value
    pub fn atan(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.as_ref().atan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.atan();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.atan();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the exponential of a value
    pub fn exp(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                let result = f.as_ref().exp();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                let f = Float::with_val(self.precision, i);
                let result = f.exp();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.exp();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the natural logarithm of a value
    pub fn ln(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                if f <= &Float::new(self.precision) {
                    return Err(UBasicError::math("Cannot take ln of non-positive number", None));
                }
                let result = f.as_ref().ln();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                if i <= &Integer::ZERO {
                    return Err(UBasicError::math("Cannot take ln of non-positive number", None));
                }
                let f = Float::with_val(self.precision, i);
                let result = f.ln();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.ln();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the square root of a value
    pub fn sqrt(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Float(f) => {
                if f < &Float::new(self.precision) {
                    return Err(UBasicError::math("Cannot take sqrt of negative number", None));
                }
                let result = f.as_ref().sqrt();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Integer(i) => {
                if i < &Integer::ZERO {
                    return Err(UBasicError::math("Cannot take sqrt of negative number", None));
                }
                let f = Float::with_val(self.precision, i);
                let result = f.sqrt();
                Ok(UBasicValue::Float(Float::with_val(self.precision, result)))
            }
            UBasicValue::Complex(c) => {
                let result = c.sqrt();
                Ok(UBasicValue::Complex(result))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the absolute value of a value
    pub fn abs(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Integer(i) => {
                Ok(UBasicValue::Integer(i.abs()))
            }
            UBasicValue::Float(f) => {
                Ok(UBasicValue::Float(f.abs()))
            }
            UBasicValue::Complex(c) => {
                Ok(UBasicValue::Float(Float::with_val(self.precision, c.norm())))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Calculate the factorial of an integer
    pub fn factorial(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Integer(i) => {
                if i < &Integer::ZERO {
                    return Err(UBasicError::math("Cannot calculate factorial of negative number", None));
                }
                if let Ok(n) = i.to_u32() {
                    let mut result = Integer::from(1);
                    for k in 2..=n {
                        result *= k;
                    }
                    Ok(UBasicValue::Integer(result))
                } else {
                    Err(UBasicError::overflow("factorial"))
                }
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?}", value.get_type()),
            )),
        }
    }

    /// Get the value of π (pi)
    pub fn pi(&self) -> UBasicValue {
        UBasicValue::Float(Float::with_val(self.precision, std::f64::consts::PI))
    }

    /// Get the value of e (Euler's number)
    pub fn e(&self) -> UBasicValue {
        UBasicValue::Float(Float::with_val(self.precision, std::f64::consts::E))
    }

    /// Calculate the greatest common divisor
    pub fn gcd(&self, a: &UBasicValue, b: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (a, b) {
            (UBasicValue::Integer(x), UBasicValue::Integer(y)) => {
                Ok(UBasicValue::Integer(x.gcd_ref(y).into()))
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?} and {:?}", a.get_type(), b.get_type()),
            )),
        }
    }

    /// Calculate the least common multiple
    pub fn lcm(&self, a: &UBasicValue, b: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (a, b) {
            (UBasicValue::Integer(x), UBasicValue::Integer(y)) => {
                Ok(UBasicValue::Integer(x.lcm_ref(y).into()))
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?} and {:?}", a.get_type(), b.get_type()),
            )),
        }
    }

    /// Check if a number is prime
    pub fn is_prime(&self, value: &UBasicValue) -> UBasicResult<UBasicValue> {
        match value {
            UBasicValue::Integer(i) => {
                if i <= &Integer::from(1) {
                    return Ok(UBasicValue::Boolean(false));
                }
                if i == &Integer::from(2) {
                    return Ok(UBasicValue::Boolean(true));
                }
                if i % 2 == 0 {
                    return Ok(UBasicValue::Boolean(false));
                }
                
                // Simple primality test
                let limit = i.sqrt_ref().into();
                let mut d = Integer::from(3);
                while d <= limit {
                    if i % &d == 0 {
                        return Ok(UBasicValue::Boolean(false));
                    }
                    d += 2;
                }
                Ok(UBasicValue::Boolean(true))
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?}", value.get_type()),
            )),
        }
    }
}

impl Default for MathEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigonometric_functions() {
        let math = MathEngine::new();
        
        let pi = math.pi();
        let result = math.sin(&pi).unwrap();
        assert!(matches!(result, UBasicValue::Float(_)));
        
        let zero = UBasicValue::Integer(Integer::ZERO);
        let result = math.cos(&zero).unwrap();
        assert!(matches!(result, UBasicValue::Float(_)));
    }

    #[test]
    fn test_exponential_and_logarithm() {
        let math = MathEngine::new();
        
        let one = UBasicValue::Integer(Integer::from(1));
        let result = math.exp(&one).unwrap();
        assert!(matches!(result, UBasicValue::Float(_)));
        
        let e = math.e();
        let result = math.ln(&e).unwrap();
        assert!(matches!(result, UBasicValue::Float(_)));
    }

    #[test]
    fn test_sqrt() {
        let math = MathEngine::new();
        
        let four = UBasicValue::Integer(Integer::from(4));
        let result = math.sqrt(&four).unwrap();
        assert!(matches!(result, UBasicValue::Float(_)));
        
        let negative = UBasicValue::Integer(Integer::from(-1));
        let result = math.sqrt(&negative);
        assert!(result.is_err());
    }

    #[test]
    fn test_factorial() {
        let math = MathEngine::new();
        
        let five = UBasicValue::Integer(Integer::from(5));
        let result = math.factorial(&five).unwrap();
        assert_eq!(result, UBasicValue::Integer(Integer::from(120)));
        
        let negative = UBasicValue::Integer(Integer::from(-1));
        let result = math.factorial(&negative);
        assert!(result.is_err());
    }

    #[test]
    fn test_gcd_lcm() {
        let math = MathEngine::new();
        
        let a = UBasicValue::Integer(Integer::from(12));
        let b = UBasicValue::Integer(Integer::from(18));
        
        let gcd_result = math.gcd(&a, &b).unwrap();
        assert_eq!(gcd_result, UBasicValue::Integer(Integer::from(6)));
        
        let lcm_result = math.lcm(&a, &b).unwrap();
        assert_eq!(lcm_result, UBasicValue::Integer(Integer::from(36)));
    }

    #[test]
    fn test_is_prime() {
        let math = MathEngine::new();
        
        let prime = UBasicValue::Integer(Integer::from(17));
        let result = math.is_prime(&prime).unwrap();
        assert_eq!(result, UBasicValue::Boolean(true));
        
        let not_prime = UBasicValue::Integer(Integer::from(15));
        let result = math.is_prime(&not_prime).unwrap();
        assert_eq!(result, UBasicValue::Boolean(false));
    }
} 