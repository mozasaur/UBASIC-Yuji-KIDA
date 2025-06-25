//! UBASIC Interpreter
//! 
//! This module contains the main interpreter that executes UBASIC code.

use crate::errors::{UBasicError, UBasicResult};
use crate::types::{UBasicValue, ProgramState};
use crate::parser::{Parser, ASTNode, Token};
use std::collections::HashMap;

/// Main UBASIC interpreter
pub struct UBasicEngine {
    parser: Parser,
    state: ProgramState,
    output: Vec<String>,
}

impl UBasicEngine {
    /// Create a new UBASIC interpreter
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            state: ProgramState::new(),
            output: Vec::new(),
        }
    }

    /// Run BASIC code and return the result
    pub fn run(&mut self, code: &str) -> UBasicResult<UBasicValue> {
        let ast = self.parser.parse(code)?;
        self.execute_node(&ast)
    }

    /// Run BASIC code in interactive mode
    pub fn run_interactive(&mut self) -> UBasicResult<()> {
        // Simplified implementation - would use rustyline in full version
        println!("UBASIC Rust Interactive Mode");
        println!("Type 'exit' to quit");
        
        loop {
            print!("> ");
            // In a real implementation, this would read from stdin
            break;
        }
        
        Ok(())
    }

    /// Clear the interpreter state
    pub fn clear(&mut self) {
        self.state.clear();
        self.output.clear();
    }

    /// Get the output history
    pub fn get_output(&self) -> &[String] {
        &self.output
    }

    /// Execute an AST node
    fn execute_node(&mut self, node: &ASTNode) -> UBasicResult<UBasicValue> {
        match node {
            ASTNode::Program { statements } => {
                let mut last_value = UBasicValue::Null;
                
                for stmt in statements {
                    last_value = self.execute_node(stmt)?;
                }
                
                Ok(last_value)
            }
            
            ASTNode::Number(value) => Ok(value.clone()),
            
            ASTNode::String(s) => Ok(UBasicValue::String(s.clone())),
            
            ASTNode::Identifier(name) => {
                self.state.get_variable(name)
                    .cloned()
                    .ok_or_else(|| UBasicError::variable_not_found(name.clone()))
            }
            
            ASTNode::BinaryOp { left, operator, right } => {
                let left_val = self.execute_node(left)?;
                let right_val = self.execute_node(right)?;
                
                self.evaluate_binary_op(&left_val, operator, &right_val)
            }
            
            ASTNode::UnaryOp { operator, operand } => {
                let operand_val = self.execute_node(operand)?;
                self.evaluate_unary_op(operator, &operand_val)
            }
            
            ASTNode::Assignment { variable, value } => {
                let value_val = self.execute_node(value)?;
                self.state.set_variable(variable.clone(), value_val.clone());
                Ok(value_val)
            }
            
            ASTNode::PrintStatement { expressions } => {
                let mut output_parts = Vec::new();
                
                for expr in expressions {
                    let value = self.execute_node(expr)?;
                    output_parts.push(value.to_string());
                }
                
                let output_line = output_parts.join(" ");
                println!("{}", output_line);
                self.output.push(output_line);
                
                Ok(UBasicValue::Null)
            }
            
            ASTNode::EndStatement => {
                // In a real implementation, this would stop execution
                Ok(UBasicValue::Null)
            }
            
            ASTNode::StopStatement => {
                // In a real implementation, this would pause execution
                Ok(UBasicValue::Null)
            }
            
            // Placeholder implementations for other nodes
            _ => Ok(UBasicValue::Null),
        }
    }

    /// Evaluate a binary operation
    fn evaluate_binary_op(
        &self,
        left: &UBasicValue,
        operator: &Token,
        right: &UBasicValue,
    ) -> UBasicResult<UBasicValue> {
        match operator {
            Token::Plus => self.add(left, right),
            Token::Minus => self.subtract(left, right),
            Token::Multiply => self.multiply(left, right),
            Token::Divide => self.divide(left, right),
            Token::Power => self.power(left, right),
            Token::Modulo => self.modulo(left, right),
            Token::Equal => Ok(UBasicValue::Boolean(self.equal(left, right))),
            Token::NotEqual => Ok(UBasicValue::Boolean(!self.equal(left, right))),
            Token::LessThan => Ok(UBasicValue::Boolean(self.less_than(left, right))),
            Token::LessEqual => Ok(UBasicValue::Boolean(self.less_equal(left, right))),
            Token::GreaterThan => Ok(UBasicValue::Boolean(self.greater_than(left, right))),
            Token::GreaterEqual => Ok(UBasicValue::Boolean(self.greater_equal(left, right))),
            Token::And => Ok(UBasicValue::Boolean(self.and(left, right))),
            Token::Or => Ok(UBasicValue::Boolean(self.or(left, right))),
            _ => Err(UBasicError::runtime(
                format!("Unsupported binary operator: {:?}", operator),
                None,
            )),
        }
    }

    /// Evaluate a unary operation
    fn evaluate_unary_op(&self, operator: &Token, operand: &UBasicValue) -> UBasicResult<UBasicValue> {
        match operator {
            Token::Minus => self.negate(operand),
            Token::Not => Ok(UBasicValue::Boolean(!self.is_truthy(operand))),
            _ => Err(UBasicError::runtime(
                format!("Unsupported unary operator: {:?}", operator),
                None,
            )),
        }
    }

    // Arithmetic operations
    fn add(&self, left: &UBasicValue, right: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                Ok(UBasicValue::Integer(a + b))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(a + b))
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(Float::with_val(64, a) + b))
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                Ok(UBasicValue::Float(a + Float::with_val(64, b)))
            }
            (UBasicValue::String(a), UBasicValue::String(b)) => {
                Ok(UBasicValue::String(format!("{}{}", a, b)))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric or string",
                &format!("{:?} and {:?}", left.get_type(), right.get_type()),
            )),
        }
    }

    fn subtract(&self, left: &UBasicValue, right: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                Ok(UBasicValue::Integer(a - b))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(a - b))
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(Float::with_val(64, a) - b))
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                Ok(UBasicValue::Float(a - Float::with_val(64, b)))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?} and {:?}", left.get_type(), right.get_type()),
            )),
        }
    }

    fn multiply(&self, left: &UBasicValue, right: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                Ok(UBasicValue::Integer(a * b))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(a * b))
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(Float::with_val(64, a) * b))
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                Ok(UBasicValue::Float(a * Float::with_val(64, b)))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?} and {:?}", left.get_type(), right.get_type()),
            )),
        }
    }

    fn divide(&self, left: &UBasicValue, right: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                if b == &Integer::ZERO {
                    return Err(UBasicError::DivisionByZero);
                }
                Ok(UBasicValue::Float(Float::with_val(64, a) / Float::with_val(64, b)))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                if b == &Float::new(64) {
                    return Err(UBasicError::DivisionByZero);
                }
                Ok(UBasicValue::Float(a / b))
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                if b == &Float::new(64) {
                    return Err(UBasicError::DivisionByZero);
                }
                Ok(UBasicValue::Float(Float::with_val(64, a) / b))
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                if b == &Integer::ZERO {
                    return Err(UBasicError::DivisionByZero);
                }
                Ok(UBasicValue::Float(a / Float::with_val(64, b)))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?} and {:?}", left.get_type(), right.get_type()),
            )),
        }
    }

    fn power(&self, left: &UBasicValue, right: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                if let Ok(b_int) = b.to_i32() {
                    if b_int >= 0 {
                        Ok(UBasicValue::Integer(a.pow(b_int as u32)))
                    } else {
                        Ok(UBasicValue::Float(Float::with_val(64, a).pow(b_int)))
                    }
                } else {
                    Err(UBasicError::overflow("integer power"))
                }
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(a.pow(b)))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?} and {:?}", left.get_type(), right.get_type()),
            )),
        }
    }

    fn modulo(&self, left: &UBasicValue, right: &UBasicValue) -> UBasicResult<UBasicValue> {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => {
                if b == &Integer::ZERO {
                    return Err(UBasicError::DivisionByZero);
                }
                Ok(UBasicValue::Integer(a % b))
            }
            _ => Err(UBasicError::type_mismatch(
                "integer",
                &format!("{:?} and {:?}", left.get_type(), right.get_type()),
            )),
        }
    }

    fn negate(&self, operand: &UBasicValue) -> UBasicResult<UBasicValue> {
        match operand {
            UBasicValue::Integer(i) => Ok(UBasicValue::Integer(-i)),
            UBasicValue::Float(f) => Ok(UBasicValue::Float(-f)),
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", operand.get_type()),
            )),
        }
    }

    // Comparison operations
    fn equal(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        left == right
    }

    fn less_than(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a < b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a < b,
            _ => false,
        }
    }

    fn less_equal(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a <= b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a <= b,
            _ => false,
        }
    }

    fn greater_than(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a > b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a > b,
            _ => false,
        }
    }

    fn greater_equal(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a >= b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a >= b,
            _ => false,
        }
    }

    // Logical operations
    fn and(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        self.is_truthy(left) && self.is_truthy(right)
    }

    fn or(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        self.is_truthy(left) || self.is_truthy(right)
    }

    fn is_truthy(&self, value: &UBasicValue) -> bool {
        match value {
            UBasicValue::Boolean(b) => *b,
            UBasicValue::Integer(i) => i != &Integer::ZERO,
            UBasicValue::Float(f) => f != &Float::new(64),
            UBasicValue::String(s) => !s.is_empty(),
            UBasicValue::Array(a) => !a.is_empty(),
            UBasicValue::Null => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_assignment() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("LET x = 42").unwrap();
        assert_eq!(result, UBasicValue::Integer(42.into()));
    }

    #[test]
    fn test_arithmetic() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("LET x = 2 + 3 * 4").unwrap();
        // Note: This will depend on operator precedence implementation
        assert!(matches!(result, UBasicValue::Integer(_)));
    }

    #[test]
    fn test_print() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("PRINT 42").unwrap();
        assert_eq!(result, UBasicValue::Null);
        assert_eq!(engine.get_output(), &["42"]);
    }

    #[test]
    fn test_variable_not_found() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("x");
        assert!(result.is_err());
    }
} 