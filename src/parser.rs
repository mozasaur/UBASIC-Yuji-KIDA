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

    DimStatement {
        name: String,
        dimensions: Vec<ASTNode>,
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
            return Err(self.errors.remove(0));
        }
        
        Ok(ASTNode::Program { statements })
    }

    /// Tokenize BASIC code into tokens
    fn tokenize(&self, code: &str) -> UBasicResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = code.chars().peekable();
        let mut line = 1;
        let mut column = 1;

        while let Some(&ch) = chars.peek() {
            match ch {
                // Whitespace
                ' ' | '\t' => {
                    chars.next();
                    column += 1;
                }
                
                // Newlines
                '\n' => {
                    chars.next();
                    tokens.push(Token::Newline);
                    line += 1;
                    column = 1;
                }
                
                '\r' => {
                    chars.next();
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    tokens.push(Token::Newline);
                    line += 1;
                    column = 1;
                }
                
                // Comments
                '\'' => {
                    // Skip to end of line
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '\n' || next_ch == '\r' {
                            break;
                        }
                        chars.next();
                        column += 1;
                    }
                }
                
                // Numbers
                '0'..='9' => {
                    let mut number = String::new();
                    let mut has_decimal = false;
                    
                    while let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            '0'..='9' => {
                                number.push(chars.next().unwrap());
                                column += 1;
                            }
                            '.' if !has_decimal => {
                                number.push(chars.next().unwrap());
                                has_decimal = true;
                                column += 1;
                            }
                            'e' | 'E' => {
                                number.push(chars.next().unwrap());
                                column += 1;
                                
                                // Handle exponent
                                if let Some(&exp_ch) = chars.peek() {
                                    if exp_ch == '+' || exp_ch == '-' {
                                        number.push(chars.next().unwrap());
                                        column += 1;
                                    }
                                }
                            }
                            _ => break,
                        }
                    }
                    
                    tokens.push(Token::Number(number));
                }
                
                // Strings
                '"' => {
                    chars.next(); // Consume opening quote
                    column += 1;
                    
                    let mut string = String::new();
                    while let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            '"' => {
                                chars.next(); // Consume closing quote
                                column += 1;
                                break;
                            }
                            '\n' | '\r' => {
                                return Err(UBasicError::syntax(
                                    "Unterminated string literal",
                                    Some(line),
                                    Some(column),
                                ));
                            }
                            '\\' => {
                                chars.next(); // Consume backslash
                                column += 1;
                                
                                // Handle escape sequences
                                if let Some(&esc_ch) = chars.peek() {
                                    match esc_ch {
                                        'n' => string.push('\n'),
                                        't' => string.push('\t'),
                                        'r' => string.push('\r'),
                                        '"' => string.push('"'),
                                        '\\' => string.push('\\'),
                                        _ => string.push(esc_ch),
                                    }
                                    chars.next();
                                    column += 1;
                                }
                            }
                            _ => {
                                string.push(chars.next().unwrap());
                                column += 1;
                            }
                        }
                    }
                    
                    tokens.push(Token::String(string));
                }
                
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            identifier.push(ch);
                            chars.next();
                            column += 1;
                        } else {
                            break;
                        }
                    }
                    
                    let token = match identifier.to_uppercase().as_str() {
                        "LET" => Token::Let,
                        "PRINT" => Token::Print,
                        "IF" => Token::If,
                        "THEN" => Token::Then,
                        "ELSE" => Token::Else,
                        "FOR" => Token::For,
                        "TO" => Token::To,
                        "STEP" => Token::Step,
                        "NEXT" => Token::Next,
                        "WHILE" => Token::While,
                        "WEND" => Token::Wend,
                        "DO" => Token::Do,
                        "LOOP" => Token::Loop,
                        "UNTIL" => Token::Until,
                        "GOTO" => Token::Goto,
                        "GOSUB" => Token::Gosub,
                        "RETURN" => Token::Return,
                        "END" => Token::End,
                        "STOP" => Token::Stop,
                        "INPUT" => Token::Input,
                        "READ" => Token::Read,
                        "DATA" => Token::Data,
                        "RESTORE" => Token::Restore,
                        "DIM" => Token::Dim,
                        "DEF" => Token::Def,
                        "FUNCTION" => Token::Function,
                        "SUB" => Token::Sub,
                        "EXIT" => Token::Exit,
                        "AND" => Token::And,
                        "OR" => Token::Or,
                        "NOT" => Token::Not,
                        "SIN" => Token::Identifier(identifier.clone()),
                        "COS" => Token::Identifier(identifier.clone()),
                        "TAN" => Token::Identifier(identifier.clone()),
                        "ASIN" => Token::Identifier(identifier.clone()),
                        "ACOS" => Token::Identifier(identifier.clone()),
                        "ATAN" => Token::Identifier(identifier.clone()),
                        "EXP" => Token::Identifier(identifier.clone()),
                        "LN" => Token::Identifier(identifier.clone()),
                        "LOG" => Token::Identifier(identifier.clone()),
                        "SQRT" => Token::Identifier(identifier.clone()),
                        "ABS" => Token::Identifier(identifier.clone()),
                        "FACTORIAL" => Token::Identifier(identifier.clone()),
                        "PI" => Token::Identifier(identifier.clone()),
                        "E" => Token::Identifier(identifier.clone()),
                        _ => Token::Identifier(identifier.clone()),
                    };
                    
                    tokens.push(token);
                }
                
                // Operators and delimiters
                '+' => {
                    chars.next();
                    tokens.push(Token::Plus);
                    column += 1;
                }
                '-' => {
                    chars.next();
                    tokens.push(Token::Minus);
                    column += 1;
                }
                '*' => {
                    chars.next();
                    tokens.push(Token::Multiply);
                    column += 1;
                }
                '/' => {
                    chars.next();
                    tokens.push(Token::Divide);
                    column += 1;
                }
                '%' => {
                    chars.next();
                    tokens.push(Token::Modulo);
                    column += 1;
                }
                '^' => {
                    chars.next();
                    tokens.push(Token::Power);
                    column += 1;
                }
                '=' => {
                    chars.next();
                    tokens.push(Token::Equal);
                    column += 1;
                }
                '<' => {
                    chars.next();
                    column += 1;
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch == '=' {
                            chars.next();
                            tokens.push(Token::LessEqual);
                            column += 1;
                        } else if next_ch == '>' {
                            chars.next();
                            tokens.push(Token::NotEqual);
                            column += 1;
                        } else {
                            tokens.push(Token::LessThan);
                        }
                    } else {
                        tokens.push(Token::LessThan);
                    }
                }
                '>' => {
                    chars.next();
                    column += 1;
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch == '=' {
                            chars.next();
                            tokens.push(Token::GreaterEqual);
                            column += 1;
                        } else {
                            tokens.push(Token::GreaterThan);
                        }
                    } else {
                        tokens.push(Token::GreaterThan);
                    }
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LeftParen);
                    column += 1;
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RightParen);
                    column += 1;
                }
                '[' => {
                    chars.next();
                    tokens.push(Token::LeftBracket);
                    column += 1;
                }
                ']' => {
                    chars.next();
                    tokens.push(Token::RightBracket);
                    column += 1;
                }
                ',' => {
                    chars.next();
                    tokens.push(Token::Comma);
                    column += 1;
                }
                ';' => {
                    chars.next();
                    tokens.push(Token::Semicolon);
                    column += 1;
                }
                ':' => {
                    chars.next();
                    tokens.push(Token::Colon);
                    column += 1;
                }
                _ => {
                    return Err(UBasicError::syntax(
                        &format!("Unexpected character: {}", ch),
                        Some(line),
                        Some(column),
                    ));
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    /// Parse a list of statements, supporting line numbers
    fn parse_statements(&mut self) -> UBasicResult<Vec<ASTNode>> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() && self.peek() != &Token::Eof {
            // Check for a line number at the start of a line
            let mut line_number = None;
            if let Token::Number(num) = self.peek() {
                if let Ok(n) = num.parse::<usize>() {
                    // Only treat as line number if at the start of a line
                    line_number = Some(n);
                    self.advance();
                }
            }
            
            let mut line_statements = Vec::new();
            // Parse statements until newline or EOF
            while !self.is_at_end() && self.peek() != &Token::Newline && self.peek() != &Token::Eof {
                if let Some(statement) = self.parse_statement()? {
                    line_statements.push(statement);
                }
                // Support statement separator ':'
                if self.peek() == &Token::Colon {
                    self.advance();
                }
            }
            // Skip newlines between lines
            while self.peek() == &Token::Newline {
                self.advance();
            }
            if !line_statements.is_empty() || line_number.is_some() {
                statements.push(ASTNode::Line {
                    number: line_number,
                    statements: line_statements,
                });
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
            Token::Identifier(_) => self.parse_expression_statement(),
            Token::Newline => {
                self.advance();
                Ok(None)
            }
            Token::Dim => self.parse_dim(),
            _ => {
                let expr = self.parse_expression()?;
                Ok(Some(expr))
            }
        }
    }

    /// Parse an assignment statement (LET x = y)
    fn parse_assignment(&mut self) -> UBasicResult<Option<ASTNode>> {
        if self.peek() == &Token::Let {
            self.advance(); // Consume LET
        }
        
        let variable = match self.peek() {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                return Err(UBasicError::syntax(
                    "Expected variable name in assignment",
                    Some(self.current_line()),
                    Some(self.current_column()),
                ));
            }
        };
        
        // Handle array access
        let mut indices = Vec::new();
        if self.peek() == &Token::LeftBracket {
            self.advance(); // Consume [
            indices.push(self.parse_expression()?);
            while self.peek() == &Token::Comma {
                self.advance(); // Consume comma
                indices.push(self.parse_expression()?);
            }
            self.expect(Token::RightBracket)?;
        }
        
        // Expect equals sign
        if self.peek() != &Token::Equal {
            return Err(UBasicError::syntax(
                "Expected '=' in assignment",
                Some(self.current_line()),
                Some(self.current_column()),
            ));
        }
        self.advance(); // Consume =
        
        let value = self.parse_expression()?;
        
        if indices.is_empty() {
            Ok(Some(ASTNode::Assignment {
                variable,
                value: Box::new(value),
            }))
        } else {
            // This would be an array assignment - for now, treat as regular assignment
            Ok(Some(ASTNode::Assignment {
                variable,
                value: Box::new(value),
            }))
        }
    }

    /// Parse a PRINT statement
    fn parse_print(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume PRINT
        
        let mut expressions = Vec::new();
        
        while !self.is_at_end() && self.peek() != &Token::Newline && self.peek() != &Token::Eof {
            if self.peek() == &Token::Comma || self.peek() == &Token::Semicolon {
                self.advance(); // Consume separator
                continue;
            }
            
            expressions.push(self.parse_expression()?);
        }
        
        Ok(Some(ASTNode::PrintStatement { expressions }))
    }

    /// Parse an IF statement
    fn parse_if(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume IF
        
        let condition = self.parse_expression()?;
        
        if self.peek() != &Token::Then {
            return Err(UBasicError::syntax(
                "Expected THEN after IF condition",
                Some(self.current_line()),
                Some(self.current_column()),
            ));
        }
        self.advance(); // Consume THEN
        
        let mut then_branch = Vec::new();
        let mut else_branch = None;
        
        // Parse THEN branch
        while !self.is_at_end() && self.peek() != &Token::Else && self.peek() != &Token::Newline {
            if let Some(statement) = self.parse_statement()? {
                then_branch.push(statement);
            }
        }
        
        // Parse ELSE branch if present
        if self.peek() == &Token::Else {
            self.advance(); // Consume ELSE
            let mut else_statements = Vec::new();
            while !self.is_at_end() && self.peek() != &Token::Newline {
                if let Some(statement) = self.parse_statement()? {
                    else_statements.push(statement);
                }
            }
            else_branch = Some(else_statements);
        }
        
        Ok(Some(ASTNode::IfStatement {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        }))
    }

    /// Parse a FOR loop
    fn parse_for(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume FOR
        
        let variable = match self.peek() {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => {
                return Err(UBasicError::syntax(
                    "Expected variable name in FOR loop",
                    Some(self.current_line()),
                    Some(self.current_column()),
                ));
            }
        };
        
        self.expect(Token::Equal)?;
        let start = self.parse_expression()?;
        
        self.expect(Token::To)?;
        let end = self.parse_expression()?;
        
        let mut step = None;
        if self.peek() == &Token::Step {
            self.advance(); // Consume STEP
            step = Some(Box::new(self.parse_expression()?));
        }
        
        let mut body = Vec::new();
        while !self.is_at_end() && self.peek() != &Token::Next {
            if let Some(statement) = self.parse_statement()? {
                body.push(statement);
            }
        }
        
        if self.peek() == &Token::Next {
            self.advance(); // Consume NEXT
        }
        
        Ok(Some(ASTNode::ForLoop {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            step,
            body,
        }))
    }

    /// Parse a WHILE loop
    fn parse_while(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume WHILE
        
        let condition = self.parse_expression()?;
        
        let mut body = Vec::new();
        while !self.is_at_end() && self.peek() != &Token::Wend {
            if let Some(statement) = self.parse_statement()? {
                body.push(statement);
            }
        }
        
        if self.peek() == &Token::Wend {
            self.advance(); // Consume WEND
        }
        
        Ok(Some(ASTNode::WhileLoop {
            condition: Box::new(condition),
            body,
        }))
    }

    /// Parse a GOTO statement
    fn parse_goto(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume GOTO
        
        let line_number = match self.peek() {
            Token::Number(num) => {
                let num = num.parse::<usize>().unwrap_or(0);
                self.advance();
                num
            }
            _ => {
                return Err(UBasicError::syntax(
                    "Expected line number after GOTO",
                    Some(self.current_line()),
                    Some(self.current_column()),
                ));
            }
        };
        
        Ok(Some(ASTNode::GotoStatement { line_number }))
    }

    /// Parse a GOSUB statement
    fn parse_gosub(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume GOSUB
        
        let line_number = match self.peek() {
            Token::Number(num) => {
                let num = num.parse::<usize>().unwrap_or(0);
                self.advance();
                num
            }
            _ => {
                return Err(UBasicError::syntax(
                    "Expected line number after GOSUB",
                    Some(self.current_line()),
                    Some(self.current_column()),
                ));
            }
        };
        
        Ok(Some(ASTNode::GosubStatement { line_number }))
    }

    /// Parse a RETURN statement
    fn parse_return(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume RETURN
        Ok(Some(ASTNode::ReturnStatement))
    }

    /// Parse an END statement
    fn parse_end(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume END
        Ok(Some(ASTNode::EndStatement))
    }

    /// Parse a STOP statement
    fn parse_stop(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume STOP
        Ok(Some(ASTNode::StopStatement))
    }

    /// Parse an INPUT statement
    fn parse_input(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume INPUT
        
        let mut variables = Vec::new();
        let mut prompt = None;
        
        // Check for prompt string
        if self.peek() == &Token::String(_) {
            if let Token::String(prompt_str) = self.advance() {
                prompt = Some(prompt_str);
            }
        }
        
        // Parse variable list
        while !self.is_at_end() && self.peek() != &Token::Newline {
            if self.peek() == &Token::Comma {
                self.advance(); // Consume comma
                continue;
            }
            
            if let Token::Identifier(name) = self.peek() {
                variables.push(name.clone());
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(Some(ASTNode::InputStatement { variables, prompt }))
    }

    /// Parse an expression as a statement
    fn parse_expression_statement(&mut self) -> UBasicResult<Option<ASTNode>> {
        Ok(Some(self.parse_expression()?))
    }

    /// Parse an expression with operator precedence
    fn parse_expression(&mut self) -> UBasicResult<ASTNode> {
        self.parse_or()
    }

    /// Parse OR expressions (lowest precedence)
    fn parse_or(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_and()?;
        
        while self.peek() == &Token::Or {
            let operator = self.advance();
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
            let operator = self.advance();
            let right = self.parse_equality()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse equality expressions (==, !=)
    fn parse_equality(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_comparison()?;
        
        while matches!(self.peek(), Token::Equal | Token::NotEqual) {
            let operator = self.advance();
            let right = self.parse_comparison()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse comparison expressions (<, <=, >, >=)
    fn parse_comparison(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_term()?;
        
        while matches!(
            self.peek(),
            Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual
        ) {
            let operator = self.advance();
            let right = self.parse_term()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse terms (+, -)
    fn parse_term(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_factor()?;
        
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let operator = self.advance();
            let right = self.parse_factor()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse factors (*, /, %)
    fn parse_factor(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_power()?;
        
        while matches!(self.peek(), Token::Multiply | Token::Divide | Token::Modulo) {
            let operator = self.advance();
            let right = self.parse_power()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse power expressions (^)
    fn parse_power(&mut self) -> UBasicResult<ASTNode> {
        let mut left = self.parse_primary()?;
        
        while self.peek() == &Token::Power {
            let operator = self.advance();
            let right = self.parse_primary()?;
            
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    /// Parse primary expressions (literals, identifiers, parenthesized expressions)
    fn parse_primary(&mut self) -> UBasicResult<ASTNode> {
        match self.peek() {
            Token::Number(num) => {
                let num_str = num.clone();
                self.advance();
                
                // Try to parse as different numeric types
                if let Ok(int_val) = num_str.parse::<i64>() {
                    Ok(ASTNode::Number(UBasicValue::Integer(int_val)))
                } else if let Ok(float_val) = num_str.parse::<f64>() {
                    Ok(ASTNode::Number(UBasicValue::Float(float_val)))
                } else {
                    Err(UBasicError::syntax(
                        &format!("Invalid number: {}", num_str),
                        Some(self.current_line()),
                        Some(self.current_column()),
                    ))
                }
            }
            
            Token::String(s) => {
                let string = s.clone();
                self.advance();
                Ok(ASTNode::String(string))
            }
            
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                
                // Check for function call
                if self.peek() == &Token::LeftParen {
                    self.advance(); // Consume (
                    
                    let mut arguments = Vec::new();
                    if self.peek() != &Token::RightParen {
                        arguments.push(self.parse_expression()?);
                        while self.peek() == &Token::Comma {
                            self.advance(); // Consume comma
                            arguments.push(self.parse_expression()?);
                        }
                    }
                    
                    self.expect(Token::RightParen)?;
                    
                    Ok(ASTNode::FunctionCall { name, arguments })
                } else if self.peek() == &Token::LeftBracket {
                    // Array access
                    self.advance(); // Consume [
                    
                    let mut indices = Vec::new();
                    indices.push(self.parse_expression()?);
                    while self.peek() == &Token::Comma {
                        self.advance(); // Consume comma
                        indices.push(self.parse_expression()?);
                    }
                    
                    self.expect(Token::RightBracket)?;
                    
                    Ok(ASTNode::ArrayAccess { array: name, indices })
                } else {
                    Ok(ASTNode::Identifier(name))
                }
            }
            
            Token::LeftParen => {
                self.advance(); // Consume (
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            
            Token::Minus => {
                self.advance(); // Consume -
                let operand = self.parse_primary()?;
                Ok(ASTNode::UnaryOp {
                    operator: Token::Minus,
                    operand: Box::new(operand),
                })
            }
            
            Token::Not => {
                self.advance(); // Consume NOT
                let operand = self.parse_primary()?;
                Ok(ASTNode::UnaryOp {
                    operator: Token::Not,
                    operand: Box::new(operand),
                })
            }
            
            _ => {
                Err(UBasicError::syntax(
                    &format!("Unexpected token: {:?}", self.peek()),
                    Some(self.current_line()),
                    Some(self.current_column()),
                ))
            }
        }
    }

    /// Parse a DIM statement
    fn parse_dim(&mut self) -> UBasicResult<Option<ASTNode>> {
        self.advance(); // Consume DIM
        let name = match self.peek() {
            Token::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => {
                return Err(UBasicError::syntax(
                    "Expected array name after DIM",
                    Some(self.current_line()),
                    Some(self.current_column()),
                ));
            }
        };
        self.expect(Token::LeftBracket)?;
        let mut dimensions = Vec::new();
        dimensions.push(self.parse_expression()?);
        while self.peek() == &Token::Comma {
            self.advance();
            dimensions.push(self.parse_expression()?);
        }
        self.expect(Token::RightBracket)?;
        Ok(Some(ASTNode::DimStatement { name, dimensions }))
    }

    // Parser helper methods
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.current]
        }
    }

    fn previous(&self) -> Token {
        if self.current == 0 {
            Token::Eof
        } else {
            self.tokens[self.current - 1].clone()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1
    }

    fn expect(&mut self, token: Token) -> UBasicResult<()> {
        if self.peek() == &token {
            self.advance();
            Ok(())
        } else {
            Err(UBasicError::syntax(
                &format!("Expected {:?}, got {:?}", token, self.peek()),
                Some(self.current_line()),
                Some(self.current_column()),
            ))
        }
    }

    fn current_line(&self) -> usize {
        // Simplified - in a real implementation, you'd track line numbers
        1
    }

    fn current_column(&self) -> usize {
        // Simplified - in a real implementation, you'd track column numbers
        self.current
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

    #[test]
    fn test_parse_assignment() {
        let mut parser = Parser::new();
        let ast = parser.parse("LET X = 42").unwrap();
        match ast {
            ASTNode::Program { statements } => {
                assert!(matches!(statements[0], ASTNode::Assignment { .. }));
            }
            _ => panic!("Expected Program node"),
        }
    }

    #[test]
    fn test_parse_arithmetic_expression() {
        let mut parser = Parser::new();
        let ast = parser.parse("LET Y = 2 + 3 * 4").unwrap();
        match ast {
            ASTNode::Program { statements } => {
                assert!(matches!(statements[0], ASTNode::Assignment { .. }));
            }
            _ => panic!("Expected Program node"),
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let mut parser = Parser::new();
        let ast = parser.parse("IF 1 THEN PRINT \"OK\" ELSE PRINT \"NO\"").unwrap();
        match ast {
            ASTNode::Program { statements } => {
                assert!(matches!(statements[0], ASTNode::IfStatement { .. }));
            }
            _ => panic!("Expected Program node"),
        }
    }

    #[test]
    fn test_parse_for_loop() {
        let mut parser = Parser::new();
        let ast = parser.parse("FOR I = 1 TO 3\nPRINT I\nNEXT").unwrap();
        match ast {
            ASTNode::Program { statements } => {
                assert!(matches!(statements[0], ASTNode::ForLoop { .. }));
            }
            _ => panic!("Expected Program node"),
        }
    }

    #[test]
    fn test_parse_print_statement() {
        let mut parser = Parser::new();
        let ast = parser.parse("PRINT 123, \"abc\"").unwrap();
        match ast {
            ASTNode::Program { statements } => {
                assert!(matches!(statements[0], ASTNode::PrintStatement { .. }));
            }
            _ => panic!("Expected Program node"),
        }
    }

    #[test]
    fn test_parse_error() {
        let mut parser = Parser::new();
        let result = parser.parse("LET = 5");
        assert!(result.is_err());
    }
} 