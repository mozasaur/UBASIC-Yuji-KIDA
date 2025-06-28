//! UBASIC Interpreter
//! 
//! This module contains the main interpreter that executes UBASIC code.

use crate::errors::{UBasicError, UBasicResult};
use crate::types::{UBasicValue, ProgramState};
use crate::parser::{Parser, ASTNode, Token};
use crate::console::Console;
use crate::math::MathEngine;
use rug::{Float, Integer, ops::Pow};
use std::collections::HashMap;

/// Main UBASIC interpreter
pub struct UBasicEngine {
    parser: Parser,
    state: ProgramState,
    output: Vec<String>,
    console: Console,
    math: MathEngine,
}

impl UBasicEngine {
    /// Create a new UBASIC interpreter
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            state: ProgramState::new(),
            output: Vec::new(),
            console: Console::new(),
            math: MathEngine::new(),
        }
    }

    /// Create a new UBASIC interpreter with history file
    pub fn with_history(history_file: String) -> Self {
        Self {
            parser: Parser::new(),
            state: ProgramState::new(),
            output: Vec::new(),
            console: Console::with_history(history_file),
            math: MathEngine::new(),
        }
    }

    /// Run BASIC code and return the result
    pub fn run(&mut self, code: &str) -> UBasicResult<UBasicValue> {
        let ast = self.parser.parse(code)?;
        self.execute_node(&ast)
    }

    /// Run BASIC code in interactive mode
    pub fn run_interactive(&mut self) -> UBasicResult<()> {
        self.console.println("UBASIC Rust Interactive Mode");
        self.console.println("Type 'exit' to quit, 'help' for help");
        self.console.println();

        loop {
            match self.console.read_line()? {
                Some(input) => {
                    let input = input.trim();
                    if input.is_empty() {
                        continue;
                    }

                    if input == "exit" || input == "quit" {
                        break;
                    }

                    if input == "help" {
                        self.console.show_help();
                        continue;
                    }

                    if input == "clear" {
                        self.clear();
                        self.console.println("Memory cleared");
                        continue;
                    }

                    if input == "cls" {
                        self.console.clear_screen();
                        continue;
                    }

                    match self.run(input) {
                        Ok(result) => {
                            if result != UBasicValue::Null {
                                self.console.println(&format!("= {}", result));
                            }
                        }
                        Err(e) => {
                            self.console.println(&format!("Error: {}", e));
                        }
                    }

                    // Update console with current variables for highlighting
                    self.console.update_variables(&self.state.variables);
                }
                None => break, // EOF or interrupt
            }
        }

        // Save history before exiting
        self.console.save_history()?;
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
                // Build line number table
                let mut line_table = std::collections::HashMap::new();
                for (idx, stmt) in statements.iter().enumerate() {
                    if let ASTNode::Line { number: Some(n), .. } = stmt {
                        line_table.insert(*n, idx);
                    }
                }
                // Execution state
                let mut pc = 0;
                let mut call_stack = Vec::new();
                let mut last_value = UBasicValue::Null;
                while pc < statements.len() {
                    match &statements[pc] {
                        ASTNode::Line { number: _, statements: line_stmts } => {
                            let mut inner_pc = 0;
                            while inner_pc < line_stmts.len() {
                                let stmt = &line_stmts[inner_pc];
                                match stmt {
                                    ASTNode::GotoStatement { line_number } => {
                                        if let Some(&target) = line_table.get(line_number) {
                                            pc = target;
                                            break; // jump to next line
                                        } else {
                                            return Err(UBasicError::runtime(
                                                format!("GOTO to undefined line {}", line_number), None));
                                        }
                                    }
                                    ASTNode::GosubStatement { line_number } => {
                                        if let Some(&target) = line_table.get(line_number) {
                                            call_stack.push(pc + 1); // return to next line after GOSUB
                                            pc = target;
                                            break;
                                        } else {
                                            return Err(UBasicError::runtime(
                                                format!("GOSUB to undefined line {}", line_number), None));
                                        }
                                    }
                                    ASTNode::ReturnStatement => {
                                        if let Some(ret_pc) = call_stack.pop() {
                                            pc = ret_pc;
                                            break;
                                        } else {
                                            return Err(UBasicError::runtime(
                                                "RETURN without GOSUB", None));
                                        }
                                    }
                                    ASTNode::EndStatement => {
                                        return Ok(UBasicValue::Null);
                                    }
                                    ASTNode::StopStatement => {
                                        return Err(UBasicError::runtime("Program stopped".to_string(), None));
                                    }
                                    _ => {
                                        last_value = self.execute_node(stmt)?;
                                    }
                                }
                                inner_pc += 1;
                            }
                        }
                        _ => {
                            last_value = self.execute_node(&statements[pc])?;
                        }
                    }
                    pc += 1;
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
                // Array assignment if variable contains '['
                if let Some(idx) = variable.find('[') {
                    let name = &variable[..idx];
                    let indices_str = &variable[idx+1..variable.len()-1];
                    let indices: Vec<usize> = indices_str.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                    if let Some(UBasicValue::Array(mut arr)) = self.state.get_variable(name).cloned() {
                        let flat_idx = if indices.len() == 1 { indices[0] } else { indices.iter().product() };
                        let val = self.execute_node(value)?;
                        if flat_idx < arr.len() {
                            arr[flat_idx] = val.clone();
                            self.state.set_variable(name.to_string(), UBasicValue::Array(arr));
                            Ok(val)
                        } else {
                            Err(UBasicError::runtime("Array index out of bounds", None))
                        }
                    } else {
                        Err(UBasicError::runtime("Array not defined", None))
                    }
                } else {
                    let value_val = self.execute_node(value)?;
                    self.state.set_variable(variable.clone(), value_val.clone());
                    Ok(value_val)
                }
            }
            
            ASTNode::PrintStatement { expressions } => {
                let mut output_parts = Vec::new();
                
                for expr in expressions {
                    let value = self.execute_node(expr)?;
                    output_parts.push(value.to_string());
                }
                
                let output_line = output_parts.join(" ");
                self.console.println(&output_line);
                self.output.push(output_line);
                
                Ok(UBasicValue::Null)
            }
            
            ASTNode::InputStatement { variables, prompt } => {
                if let Some(prompt_text) = prompt {
                    self.console.print(prompt_text);
                }
                
                match self.console.read_line()? {
                    Some(input) => {
                        let values: Vec<&str> = input.split(',').map(|s| s.trim()).collect();
                        
                        for (i, var_name) in variables.iter().enumerate() {
                            if i < values.len() {
                                let value = self.parse_input_value(values[i])?;
                                self.state.set_variable(var_name.clone(), value);
                            }
                        }
                        
                        Ok(UBasicValue::Null)
                    }
                    None => Err(UBasicError::runtime("Input interrupted".to_string(), None)),
                }
            }
            
            ASTNode::IfStatement { condition, then_branch, else_branch } => {
                let condition_val = self.execute_node(condition)?;
                
                if self.is_truthy(&condition_val) {
                    let mut last_value = UBasicValue::Null;
                    for stmt in then_branch {
                        last_value = self.execute_node(stmt)?;
                    }
                    Ok(last_value)
                } else if let Some(else_statements) = else_branch {
                    let mut last_value = UBasicValue::Null;
                    for stmt in else_statements {
                        last_value = self.execute_node(stmt)?;
                    }
                    Ok(last_value)
                } else {
                    Ok(UBasicValue::Null)
                }
            }
            
            ASTNode::ForLoop { variable, start, end, step, body } => {
                let start_val = self.to_numeric(&self.execute_node(start)?)?;
                let end_val = self.to_numeric(&self.execute_node(end)?)?;
                let step_val = if let Some(step_expr) = step {
                    self.to_numeric(&self.execute_node(step_expr)?)?
                } else {
                    1.0
                };
                
                let mut current = start_val;
                let mut last_value = UBasicValue::Null;
                
                while if step_val > 0.0 { current <= end_val } else { current >= end_val } {
                    self.state.set_variable(variable.clone(), UBasicValue::Float(current));
                    
                    for stmt in body {
                        last_value = self.execute_node(stmt)?;
                    }
                    
                    current += step_val;
                }
                
                Ok(last_value)
            }
            
            ASTNode::WhileLoop { condition, body } => {
                let mut last_value = UBasicValue::Null;
                
                while self.is_truthy(&self.execute_node(condition)?) {
                    for stmt in body {
                        last_value = self.execute_node(stmt)?;
                    }
                }
                
                Ok(last_value)
            }
            
            ASTNode::FunctionCall { name, arguments } => {
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.execute_node(arg)?);
                }
                
                self.call_function(name, &arg_values)
            }
            
            ASTNode::ArrayAccess { array, indices } => {
                if let Some(UBasicValue::Array(arr)) = self.state.get_variable(array).cloned() {
                    let mut idx = 0;
                    if indices.len() == 1 {
                        idx = self.to_numeric(&self.execute_node(&indices[0])?)? as usize;
                    } else {
                        let mut mult = 1;
                        for i in indices.iter().rev() {
                            idx += (self.to_numeric(&self.execute_node(i)?)? as usize) * mult;
                            mult *= arr.len();
                        }
                    }
                    if idx < arr.len() {
                        Ok(arr[idx].clone())
                    } else {
                        Err(UBasicError::runtime("Array index out of bounds", None))
                    }
                } else {
                    Err(UBasicError::runtime("Array not defined", None))
                }
            }
            
            ASTNode::Line { number: _, statements } => {
                let mut last_value = UBasicValue::Null;
                for stmt in statements {
                    last_value = self.execute_node(stmt)?;
                }
                Ok(last_value)
            }
            
            ASTNode::DimStatement { name, dimensions } => {
                // Evaluate dimensions
                let mut dims = Vec::new();
                for dim in dimensions {
                    let d = self.to_numeric(&self.execute_node(dim)?)?;
                    dims.push(d as usize);
                }
                // Create array (vector of Nulls)
                let size = dims.iter().product();
                let arr = vec![UBasicValue::Null; size];
                self.state.set_variable(name.clone(), UBasicValue::Array(arr));
                Ok(UBasicValue::Null)
            }
        }
    }

    /// Parse input value from string
    fn parse_input_value(&self, input: &str) -> UBasicResult<UBasicValue> {
        let input = input.trim();
        
        // Try to parse as number
        if let Ok(i) = input.parse::<i64>() {
            return Ok(UBasicValue::Integer(i.into()));
        }
        
        if let Ok(f) = input.parse::<f64>() {
            return Ok(UBasicValue::Float(f.into()));
        }
        
        // If it starts and ends with quotes, it's a string
        if input.starts_with('"') && input.ends_with('"') {
            let s = &input[1..input.len()-1];
            return Ok(UBasicValue::String(s.to_string()));
        }
        
        // Otherwise treat as string
        Ok(UBasicValue::String(input.to_string()))
    }

    /// Convert value to numeric for loop iteration
    fn to_numeric(&self, value: &UBasicValue) -> UBasicResult<f64> {
        match value {
            UBasicValue::Integer(i) => Ok(i.to_f64()),
            UBasicValue::Float(f) => Ok(f.to_f64()),
            _ => Err(UBasicError::type_mismatch("numeric", &format!("{:?}", value.get_type()))),
        }
    }

    /// Call a function
    fn call_function(&mut self, name: &str, arguments: &[UBasicValue]) -> UBasicResult<UBasicValue> {
        let upper_name = name.to_uppercase();
        
        // Built-in mathematical functions
        match upper_name.as_str() {
            "SIN" => {
                if arguments.len() != 1 {
                    return Err(UBasicError::argument_count_mismatch(1, arguments.len()));
                }
                self.math.sin(&arguments[0])
            }
            "COS" => {
                if arguments.len() != 1 {
                    return Err(UBasicError::argument_count_mismatch(1, arguments.len()));
                }
                self.math.cos(&arguments[0])
            }
            "TAN" => {
                if arguments.len() != 1 {
                    return Err(UBasicError::argument_count_mismatch(1, arguments.len()));
                }
                self.math.tan(&arguments[0])
            }
            "SQRT" => {
                if arguments.len() != 1 {
                    return Err(UBasicError::argument_count_mismatch(1, arguments.len()));
                }
                self.math.sqrt(&arguments[0])
            }
            "ABS" => {
                if arguments.len() != 1 {
                    return Err(UBasicError::argument_count_mismatch(1, arguments.len()));
                }
                self.math.abs(&arguments[0])
            }
            "PI" => {
                if !arguments.is_empty() {
                    return Err(UBasicError::argument_count_mismatch(0, arguments.len()));
                }
                Ok(self.math.pi())
            }
            "E" => {
                if !arguments.is_empty() {
                    return Err(UBasicError::argument_count_mismatch(0, arguments.len()));
                }
                Ok(self.math.e())
            }
            _ => {
                // Check for user-defined functions
                if let Some(function) = self.state.functions.get(name) {
                    if function.parameters.len() != arguments.len() {
                        return Err(UBasicError::argument_count_mismatch(
                            function.parameters.len(),
                            arguments.len(),
                        ));
                    }
                    
                    // Save current state
                    let old_variables = self.state.variables.clone();
                    
                    // Set function parameters
                    for (param, arg) in function.parameters.iter().zip(arguments.iter()) {
                        self.state.set_variable(param.clone(), arg.clone());
                    }
                    
                    // Execute function body
                    let mut result = UBasicValue::Null;
                    for line in &function.body {
                        result = self.run(line)?;
                    }
                    
                    // Restore state
                    self.state.variables = old_variables;
                    
                    Ok(result)
                } else {
                    Err(UBasicError::function_not_found(name.to_string()))
                }
            }
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
                Ok(UBasicValue::Integer((a + b).into()))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float((a + b).into()))
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
                Ok(UBasicValue::Integer((a - b).into()))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float((a - b).into()))
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
                Ok(UBasicValue::Integer((a * b).into()))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float((a * b).into()))
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
                    return Err(UBasicError::runtime("Division by zero".to_string(), None));
                }
                Ok(UBasicValue::Float(Float::with_val(64, a) / Float::with_val(64, b)))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                if b == &Float::new(64) {
                    return Err(UBasicError::runtime("Division by zero".to_string(), None));
                }
                Ok(UBasicValue::Float(a / b))
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                if b == &Float::new(64) {
                    return Err(UBasicError::runtime("Division by zero".to_string(), None));
                }
                Ok(UBasicValue::Float(Float::with_val(64, a) / b))
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                if b == &Integer::ZERO {
                    return Err(UBasicError::runtime("Division by zero".to_string(), None));
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
                if let Some(b_small) = b.to_u32() {
                    if b_small <= 1000 { // Reasonable limit
                        Ok(UBasicValue::Integer(a.pow(b_small).into()))
                    } else {
                        Ok(UBasicValue::Float(Float::with_val(64, a).pow(b)))
                    }
                } else {
                    Ok(UBasicValue::Float(Float::with_val(64, a).pow(b)))
                }
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(a.pow(b)))
            }
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Ok(UBasicValue::Float(Float::with_val(64, a).pow(b)))
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                Ok(UBasicValue::Float(a.pow(Float::with_val(64, b))))
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
                    return Err(UBasicError::runtime("Modulo by zero".to_string(), None));
                }
                Ok(UBasicValue::Integer((a % b).into()))
            }
            (UBasicValue::Float(a), UBasicValue::Float(b)) => {
                if b == &Float::new(64) {
                    return Err(UBasicError::runtime("Modulo by zero".to_string(), None));
                }
                Ok(UBasicValue::Float((a % b).into()))
            }
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?} and {:?}", left.get_type(), right.get_type()),
            )),
        }
    }

    fn negate(&self, operand: &UBasicValue) -> UBasicResult<UBasicValue> {
        match operand {
            UBasicValue::Integer(i) => Ok(UBasicValue::Integer((-i).into())),
            UBasicValue::Float(f) => Ok(UBasicValue::Float((-f).into())),
            _ => Err(UBasicError::type_mismatch(
                "numeric",
                &format!("{:?}", operand.get_type()),
            )),
        }
    }

    fn equal(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        left == right
    }

    fn less_than(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a < b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a < b,
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Float::with_val(64, a) < *b
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                a < &Float::with_val(64, b)
            }
            _ => false,
        }
    }

    fn less_equal(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a <= b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a <= b,
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Float::with_val(64, a) <= *b
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                a <= &Float::with_val(64, b)
            }
            _ => false,
        }
    }

    fn greater_than(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a > b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a > b,
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Float::with_val(64, a) > *b
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                a > &Float::with_val(64, b)
            }
            _ => false,
        }
    }

    fn greater_equal(&self, left: &UBasicValue, right: &UBasicValue) -> bool {
        match (left, right) {
            (UBasicValue::Integer(a), UBasicValue::Integer(b)) => a >= b,
            (UBasicValue::Float(a), UBasicValue::Float(b)) => a >= b,
            (UBasicValue::Integer(a), UBasicValue::Float(b)) => {
                Float::with_val(64, a) >= *b
            }
            (UBasicValue::Float(a), UBasicValue::Integer(b)) => {
                a >= &Float::with_val(64, b)
            }
            _ => false,
        }
    }

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
        let result = engine.run("LET y = 2 + 3 * 4").unwrap();
        assert_eq!(result, UBasicValue::Integer(14.into()));
    }

    #[test]
    fn test_print() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("PRINT \"Hello\"").unwrap();
        assert_eq!(result, UBasicValue::Null);
        assert_eq!(engine.get_output(), &["Hello"]);
    }

    #[test]
    fn test_variable_not_found() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("PRINT x");
        assert!(result.is_err());
    }

    #[test]
    fn test_interpreter_assignment_and_arithmetic() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("LET X = 10\nLET Y = X + 5\nPRINT Y");
        assert!(result.is_ok());
        assert_eq!(engine.state.get_variable("Y").unwrap().to_string(), "15");
    }

    #[test]
    fn test_interpreter_print() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("PRINT 123, \"abc\"");
        assert!(result.is_ok());
        let output = engine.get_output();
        assert!(output.iter().any(|line| line.contains("123")));
        assert!(output.iter().any(|line| line.contains("abc")));
    }

    #[test]
    fn test_interpreter_if_statement() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("LET X = 1\nIF X THEN PRINT \"YES\" ELSE PRINT \"NO\"");
        assert!(result.is_ok());
        let output = engine.get_output();
        assert!(output.iter().any(|line| line.contains("YES")));
    }

    #[test]
    fn test_interpreter_for_loop() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("FOR I = 1 TO 3\nPRINT I\nNEXT");
        assert!(result.is_ok());
        let output = engine.get_output();
        assert!(output.iter().any(|line| line.contains("1")));
        assert!(output.iter().any(|line| line.contains("2")));
        assert!(output.iter().any(|line| line.contains("3")));
    }

    #[test]
    fn test_interpreter_error() {
        let mut engine = UBasicEngine::new();
        let result = engine.run("LET = 5");
        assert!(result.is_err());
    }

    #[test]
    fn test_goto_statement() {
        let mut engine = UBasicEngine::new();
        let code = "10 PRINT \"A\"
20 GOTO 40
30 PRINT \"B\"
40 PRINT \"C\"";
        let result = engine.run(code);
        assert!(result.is_ok());
        let output = engine.get_output();
        assert!(output.iter().any(|line| line.contains("A")));
        assert!(!output.iter().any(|line| line.contains("B")));
        assert!(output.iter().any(|line| line.contains("C")));
    }

    #[test]
    fn test_gosub_and_return() {
        let mut engine = UBasicEngine::new();
        let code = "10 PRINT \"Start\"
20 GOSUB 100
30 PRINT \"Back\"
40 END
100 PRINT \"Sub\"
110 RETURN";
        let result = engine.run(code);
        assert!(result.is_ok());
        let output = engine.get_output();
        assert_eq!(output[0], "Start");
        assert_eq!(output[1], "Sub");
        assert_eq!(output[2], "Back");
    }

    #[test]
    fn test_return_without_gosub_error() {
        let mut engine = UBasicEngine::new();
        let code = "10 RETURN";
        let result = engine.run(code);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("RETURN without GOSUB"));
    }

    #[test]
    fn test_dim_and_array_assignment_access() {
        let mut engine = UBasicEngine::new();
        let code = "10 DIM A[3]\n20 LET A[0] = 42\n30 LET A[1] = 7\n40 LET A[2] = A[0] + A[1]\n50 PRINT A[2]";
        let result = engine.run(code);
        assert!(result.is_ok());
        let output = engine.get_output();
        assert!(output.iter().any(|line| line.contains("49")));
    }
} 