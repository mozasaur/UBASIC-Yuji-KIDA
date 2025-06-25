//! UBASIC Parser
//! 
//! This module handles parsing of UBASIC code into an Abstract Syntax Tree (AST).

use crate::errors::{UBasicError, UBasicResult};
use crate::types::UBasicValue;
use std::collections::HashMap;

/// Token types for UBASIC
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Print,
    If,
    Then,
    Else,
    For,
    To,
    Step,
    Next,
    While,
    Wend,
    Do,
    Loop,
    Until,
    WhileLoop,
    Goto,
    Gosub,
    Return,
    End,
    Stop,
    Input,
    Read,
    Data,
    Restore,
    Dim,
    Def,
    Function,
    Sub,
    Exit,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Assign,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
    Not,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Quote,
    
    // Literals
    Number(String),
    String(String),
    Identifier(String),
    
    // Special
    Newline,
    Eof,
}

/// Abstract Syntax Tree nodes
#[derive(Debug, Clone)]
pub enum ASTNode {
    // Literals
    Number(UBasicValue),
    String(String),
    Identifier(String),
    
    // Binary operations
    BinaryOp {
        left: Box<ASTNode>,
        operator: Token,
        right: Box<ASTNode>,
    },
    
    // Unary operations
    UnaryOp {
        operator: Token,
        operand: Box<ASTNode>,
    },
    
    // Variable assignment
    Assignment {
        variable: String,
        value: Box<ASTNode>,
    },
    
    // Array access
    ArrayAccess {
        array: String,
        indices: Vec<ASTNode>,
    },
    
    // Function call
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },
    
    // Control flow
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        else_branch: Option<Vec<ASTNode>>,
    },
    
    ForLoop {
        variable: String,
        start: Box<ASTNode>,
        end: Box<ASTNode>,
        step: Option<Box<ASTNode>>,
        body: Vec<ASTNode>,
    },
    
    WhileLoop {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    
    // Statements
    PrintStatement {
        expressions: Vec<ASTNode>,
    },
    
    InputStatement {
        variables: Vec<String>,
        prompt: Option<String>,
    },
    
    GotoStatement {
        line_number: usize,
    },
    
    GosubStatement {
        line_number: usize,
    },
    
    ReturnStatement,
    
    EndStatement,
    
    StopStatement,
    
    // Program structure
    Program {
        statements: Vec<ASTNode>,
    },
    
    Line {
        number: Option<usize>,
        statements: Vec<ASTNode>,
    },
}

/// Parser for UBASIC code
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<UBasicError>,
}

impl Parser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Parse BASIC code into an AST
    pub fn parse(&mut self, code: &str) -> UBasicResult<ASTNode> {
        self.tokens = self.tokenize(code)?;
        self.current = 0;
        self.errors.clear();
        
        let statements = self.parse_statements()?;
        
        if !self.errors.is_empty() {
            return Err(self.errors[0].clone());
        }
        
        Ok(ASTNode::Program { statements })
    }

    /// Tokenize the input code
    fn tokenize(&self, code: &str) -> UBasicResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = code.chars().peekable();
        let mut line = 1;
        let mut column = 1;

        while let Some(ch) = chars.next() {
            match ch {
                // Whitespace
                ' ' | '\t' => {
                    column += 1;
                }
                
                // Newlines
                '\n' => {
                    tokens.push(Token::Newline);
                    line += 1;
                    column = 1;
                }
                
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                        tokens.push(Token::Newline);
                        line += 1;
                        column = 1;
                    } else {
                        tokens.push(Token::Newline);
                        line += 1;
                        column = 1;
                    }
                }
                
                // Numbers
                '0'..='9' => {
                    let mut number = String::new();
                    number.push(ch);
                    
                    while let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            '0'..='9' | '.' | 'e' | 'E' | '+' | '-' => {
                                number.push(chars.next().unwrap());
                            }
                            _ => break,
                        }
                    }
                    
                    tokens.push(Token::Number(number));
                    column += number.len();
                }
                
                // Strings
                '"' => {
                    let mut string = String::new();
                    column += 1;
                    
                    while let Some(next_ch) = chars.next() {
                        match next_ch {
                            '"' => break,
                            '\n' | '\r' => {
                                return Err(UBasicError::syntax(
                                    "Unterminated string literal",
                                    line,
                                    column,
                                ));
                            }
                            _ => {
                                string.push(next_ch);
                                column += 1;
                            }
                        }
                    }
                    
                    tokens.push(Token::String(string));
                    column += 1;
                }
                
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = String::new();
                    identifier.push(ch);
                    
                    while let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                identifier.push(chars.next().unwrap());
                            }
                            _ => break,
                        }
                    }
                    
                    let token = match identifier.to_lowercase().as_str() {
                        "let" => Token::Let,
                        "print" => Token::Print,
                        "if" => Token::If,
                        "then" => Token::Then,
                        "else" => Token::Else,
                        "for" => Token::For,
                        "to" => Token::To,
                        "step" => Token::Step,
                        "next" => Token::Next,
                        "while" => Token::While,
                        "wend" => Token::Wend,
                        "do" => Token::Do,
                        "loop" => Token::Loop,
                        "until" => Token::Until,
                        "goto" => Token::Goto,
                        "gosub" => Token::Gosub,
                        "return" => Token::Return,
                        "end" => Token::End,
                        "stop" => Token::Stop,
                        "input" => Token::Input,
                        "read" => Token::Read,
                        "data" => Token::Data,
                        "restore" => Token::Restore,
                        "dim" => Token::Dim,
                        "def" => Token::Def,
                        "function" => Token::Function,
                        "sub" => Token::Sub,
                        "exit" => Token::Exit,
                        "and" => Token::And,
                        "or" => Token::Or,
                        "not" => Token::Not,
                        _ => Token::Identifier(identifier),
                    };
                    
                    tokens.push(token);
                    column += identifier.len();
                }
                
                // Operators and delimiters
                '+' => {
                    tokens.push(Token::Plus);
                    column += 1;
                }
                '-' => {
                    tokens.push(Token::Minus);
                    column += 1;
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    column += 1;
                }
                '/' => {
                    tokens.push(Token::Divide);
                    column += 1;
                }
                '%' => {
                    tokens.push(Token::Modulo);
                    column += 1;
                }
                '^' => {
                    tokens.push(Token::Power);
                    column += 1;
                }
                '=' => {
                    tokens.push(Token::Assign);
                    column += 1;
                }
                '<' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::LessEqual);
                        column += 2;
                    } else if chars.peek() == Some(&'>') {
                        chars.next();
                        tokens.push(Token::NotEqual);
                        column += 2;
                    } else {
                        tokens.push(Token::LessThan);
                        column += 1;
                    }
                }
                '>' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::GreaterEqual);
                        column += 2;
                    } else {
                        tokens.push(Token::GreaterThan);
                        column += 1;
                    }
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    column += 1;
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    column += 1;
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    column += 1;
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    column += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    column += 1;
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    column += 1;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    column += 1;
                }
                
                // Unknown character
                _ => {
                    return Err(UBasicError::syntax(
                        &format!("Unexpected character: '{}'", ch),
                        line,
                        column,
                    ));
                }
            }
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    /// Parse a list of statements
    fn parse_statements(&mut self) -> UBasicResult<Vec<ASTNode>> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() && self.peek() != &Token::Eof {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
            
            // Skip newlines between statements
            while self.peek() == &Token::Newline {
                self.advance();
            }
        }
        
        Ok(statements)
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> UBasicResult<Option<ASTNode>> {
        match self.peek() {
            Token::Let => self.parse_assignment(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::For => self.parse_for(),
            Token::While => self.parse_while(),
            Token::Goto => self.parse_goto(),
            Token::Gosub => self.parse_gosub(),
            Token::Return => self.parse_return(),
            Token::End => self.parse_end(),
            Token::Stop => self.parse_stop(),
            Token::Input => self.parse_input(),
            Token::Newline => {
                self.advance();
                Ok(None)
            }
            _ => self.parse_expression_statement(),
        }
    }

    /// Parse an assignment statement
    fn parse_assignment(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.expect(Token::Let)?;
        
        let variable = match self.advance() {
            Token::Identifier(name) => name,
            _ => {
                return Err(UBasicError::syntax(
                    "Expected variable name after LET",
                    self.current_line(),
                    self.current_column(),
                ));
            }
        };
        
        self.expect(Token::Assign)?;
        
        let value = self.parse_expression()?;
        
        Ok(Some(ASTNode::Assignment {
            variable,
            value: Box::new(value),
        }))
    }

    /// Parse a print statement
    fn parse_print(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.expect(Token::Print)?;
        
        let mut expressions = Vec::new();
        
        if !self.is_at_end() && self.peek() != &Token::Newline {
            expressions.push(self.parse_expression()?);
            
            while self.peek() == &Token::Comma || self.peek() == &Token::Semicolon {
                self.advance();
                if self.peek() != &Token::Newline {
                    expressions.push(self.parse_expression()?);
                }
            }
        }
        
        Ok(Some(ASTNode::PrintStatement { expressions }))
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> UBasicResult<ASTNode> {
        self.parse_or()
    }

    /// Parse OR expressions
    fn parse_or(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_and()?;
        
        while self.peek() == &Token::Or {
            let operator = self.advance().clone();
            let right = self.parse_and()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse AND expressions
    fn parse_and(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_equality()?;
        
        while self.peek() == &Token::And {
            let operator = self.advance().clone();
            let right = self.parse_equality()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse equality expressions
    fn parse_equality(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_comparison()?;
        
        while matches!(self.peek(), Token::Equal | Token::NotEqual) {
            let operator = self.advance().clone();
            let right = self.parse_comparison()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse comparison expressions
    fn parse_comparison(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_term()?;
        
        while matches!(
            self.peek(),
            Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual
        ) {
            let operator = self.advance().clone();
            let right = self.parse_term()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse term expressions (addition and subtraction)
    fn parse_term(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_factor()?;
        
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let operator = self.advance().clone();
            let right = self.parse_factor()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse factor expressions (multiplication and division)
    fn parse_factor(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_power()?;
        
        while matches!(self.peek(), Token::Multiply | Token::Divide | Token::Modulo) {
            let operator = self.advance().clone();
            let right = self.parse_power()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse power expressions
    fn parse_power(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_primary()?;
        
        while self.peek() == &Token::Power {
            let operator = self.advance().clone();
            let right = self.parse_primary()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse primary expressions
    fn parse_primary(&mut self) -> UBasicResult<ASTNode> {
        match self.advance() {
            Token::Number(num_str) => {
                // Try to parse as integer first, then float
                if let Ok(int_val) = num_str.parse::<i64>() {
                    Ok(ASTNode::Number(UBasicValue::from(int_val)))
                } else if let Ok(float_val) = num_str.parse::<f64>() {
                    Ok(ASTNode::Number(UBasicValue::from(float_val)))
                } else {
                    Err(UBasicError::invalid_number(num_str))
                }
            }
            Token::String(s) => Ok(ASTNode::String(s)),
            Token::Identifier(name) => Ok(ASTNode::Identifier(name)),
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::Minus => {
                let operand = self.parse_primary()?;
                Ok(ASTNode::UnaryOp {
                    operator: Token::Minus,
                    operand: Box::new(operand),
                })
            }
            Token::Not => {
                let operand = self.parse_primary()?;
                Ok(ASTNode::UnaryOp {
                    operator: Token::Not,
                    operand: Box::new(operand),
                })
            }
            _ => Err(UBasicError::syntax(
                "Unexpected token in expression",
                self.current_line(),
                self.current_column(),
            )),
        }
    }

    // Helper methods for parsing other statements
    fn parse_if(&mut self) -> UBasicResult<Option<ASTNode>> {
        // Simplified implementation
        Ok(None)
    }

    fn parse_for(&mut self) -> UBasicResult<Option<ASTNode>> {
        // Simplified implementation
        Ok(None)
    }

    fn parse_while(&mut self) -> UBasicResult<Option<ASTNode>> {
        // Simplified implementation
        Ok(None)
    }

    fn parse_goto(&mut self) -> UBasicResult<Option<ASTNode>> {
        // Simplified implementation
        Ok(None)
    }

    fn parse_gosub(&mut self) -> UBasicResult<Option<ASTNode>> {
        // Simplified implementation
        Ok(None)
    }

    fn parse_return(&mut self) -> UBasicResult<Option<ASTNode>> {
        // Simplified implementation
        Ok(None)
    }

    fn parse_end(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.expect(Token::End)?;
        Ok(Some(ASTNode::EndStatement))
    }

    fn parse_stop(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.expect(Token::Stop)?;
        Ok(Some(ASTNode::StopStatement))
    }

    fn parse_input(&mut self) -> UBasicResult<Option<ASTNode>> {
        // Simplified implementation
        Ok(None)
    }

    fn parse_expression_statement(&mut self) -> UBasicResult<Option<ASTNode>> {
        let expr = self.parse_expression()?;
        Ok(Some(expr))
    }

    // Parser helper methods
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            &Token::Eof
        } else {
            &self.tokens[self.current]
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn expect(&mut self, token: Token) -> UBasicResult<()> {
        if self.peek() == &token {
            self.advance();
            Ok(())
        } else {
            Err(UBasicError::syntax(
                &format!("Expected {:?}, got {:?}", token, self.peek()),
                self.current_line(),
                self.current_column(),
            ))
        }
    }

    fn current_line(&self) -> usize {
        // Simplified - in a real implementation, you'd track line numbers
        1
    }

    fn current_column(&self) -> usize {
        // Simplified - in a real implementation, you'd track column numbers
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let parser = Parser::new();
        let tokens = parser.tokenize("123 456.789").unwrap();
        
        assert_eq!(tokens[0], Token::Number("123".to_string()));
        assert_eq!(tokens[1], Token::Number("456.789".to_string()));
    }

    #[test]
    fn test_tokenize_strings() {
        let parser = Parser::new();
        let tokens = parser.tokenize(r#""hello world""#).unwrap();
        
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
    }

    #[test]
    fn test_tokenize_keywords() {
        let parser = Parser::new();
        let tokens = parser.tokenize("LET PRINT IF").unwrap();
        
        assert_eq!(tokens[0], Token::Let);
        assert_eq!(tokens[1], Token::Print);
        assert_eq!(tokens[2], Token::If);
    }

    #[test]
    fn test_parse_simple_assignment() {
        let mut parser = Parser::new();
        let ast = parser.parse("LET x = 42").unwrap();
        
        if let ASTNode::Program { statements } = ast {
            assert_eq!(statements.len(), 1);
        } else {
            panic!("Expected Program node");
        }
    }

    #[test]
    fn test_parse_arithmetic() {
        let mut parser = Parser::new();
        let ast = parser.parse("LET x = 2 + 3 * 4").unwrap();
        
        // Should parse correctly
        assert!(matches!(ast, ASTNode::Program { .. }));
    }
} 