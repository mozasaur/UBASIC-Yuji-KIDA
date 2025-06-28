//! Console interface for UBASIC

use crate::errors::{UBasicError, UBasicResult};
use crate::types::UBasicValue;
use std::collections::HashMap;

#[cfg(feature = "console")]
use rustyline::{
    error::ReadlineError, 
    Editor, 
    Helper, 
    highlight::Highlighter, 
    hint::Hinter, 
    validate::Validator,
    completion::Completer,
    Cmd,
};

/// Console interface for UBASIC
pub struct Console {
    prompt: String,
    history_file: Option<String>,
    #[cfg(feature = "console")]
    editor: Option<Editor<UBasicHelper>>,
}

#[cfg(feature = "console")]
struct UBasicHelper {
    keywords: Vec<String>,
    functions: Vec<String>,
    variables: HashMap<String, UBasicValue>,
}

#[cfg(feature = "console")]
impl UBasicHelper {
    fn new() -> Self {
        let keywords = vec![
            "LET".to_string(), "PRINT".to_string(), "IF".to_string(), "THEN".to_string(), "ELSE".to_string(), 
            "FOR".to_string(), "TO".to_string(), "STEP".to_string(), "NEXT".to_string(),
            "WHILE".to_string(), "WEND".to_string(), "DO".to_string(), "LOOP".to_string(), "UNTIL".to_string(), 
            "GOTO".to_string(), "GOSUB".to_string(), "RETURN".to_string(),
            "END".to_string(), "STOP".to_string(), "INPUT".to_string(), "READ".to_string(), "DATA".to_string(), 
            "RESTORE".to_string(), "DIM".to_string(), "DEF".to_string(),
            "FUNCTION".to_string(), "SUB".to_string(), "EXIT".to_string(), "SCREEN".to_string(), "LINE".to_string(), 
            "CIRCLE".to_string(), "PSET".to_string(),
            "CLS".to_string(), "COLOR".to_string(), "LOCATE".to_string(), "BEEP".to_string(), "SLEEP".to_string(), 
            "RANDOMIZE".to_string(), "TIMER".to_string(),
        ];

        let functions = vec![
            "SIN".to_string(), "COS".to_string(), "TAN".to_string(), "ASIN".to_string(), "ACOS".to_string(), 
            "ATAN".to_string(), "EXP".to_string(), "LOG".to_string(), "LN".to_string(),
            "SQRT".to_string(), "ABS".to_string(), "INT".to_string(), "FIX".to_string(), "FRAC".to_string(), 
            "SGN".to_string(), "RND".to_string(), "PI".to_string(), "E".to_string(),
            "GCD".to_string(), "LCM".to_string(), "FACTORIAL".to_string(), "IS_PRIME".to_string(), "REAL".to_string(), 
            "IMAG".to_string(), "CONJ".to_string(),
            "NORM".to_string(), "ARG".to_string(), "POLAR".to_string(), "RECT".to_string(), "DEG".to_string(), 
            "RAD".to_string(),
        ];

        Self {
            keywords,
            functions,
            variables: HashMap::new(),
        }
    }

    fn update_variables(&mut self, variables: &HashMap<String, UBasicValue>) {
        self.variables.clear();
        for (name, value) in variables {
            self.variables.insert(name.clone(), value.clone());
        }
    }
}

#[cfg(feature = "console")]
impl Completer for UBasicHelper {
    type Candidate = String;

    fn complete(&mut self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut candidates = Vec::new();
        let word = line[..pos].split_whitespace().last().unwrap_or("");
        
        if word.is_empty() {
            return Ok((pos, candidates));
        }

        let word_upper = word.to_uppercase();
        
        // Complete keywords
        for keyword in &self.keywords {
            if keyword.starts_with(&word_upper) {
                candidates.push(keyword.clone());
            }
        }
        
        // Complete functions
        for func in &self.functions {
            if func.starts_with(&word_upper) {
                candidates.push(func.clone());
            }
        }
        
        // Complete variables
        for var in self.variables.keys() {
            if var.starts_with(word) {
                candidates.push(var.clone());
            }
        }
        
        Ok((pos - word.len(), candidates))
    }
}

#[cfg(feature = "console")]
impl Highlighter for UBasicHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        use std::borrow::Cow;
        
        let mut highlighted = String::new();
        let mut in_string = false;
        let mut current_word = String::new();
        
        for ch in line.chars() {
            match ch {
                '"' => {
                    if !current_word.is_empty() {
                        highlighted.push_str(&self.highlight_word(&current_word));
                        current_word.clear();
                    }
                    highlighted.push_str("\x1b[32m"); // Green for strings
                    highlighted.push(ch);
                    in_string = !in_string;
                    if !in_string {
                        highlighted.push_str("\x1b[0m"); // Reset
                    }
                }
                ' ' | '\t' | '+' | '-' | '*' | '/' | '=' | '<' | '>' | '(' | ')' | ',' => {
                    if !current_word.is_empty() {
                        highlighted.push_str(&self.highlight_word(&current_word));
                        current_word.clear();
                    }
                    if !in_string {
                        highlighted.push_str("\x1b[36m"); // Cyan for operators
                        highlighted.push(ch);
                        highlighted.push_str("\x1b[0m"); // Reset
                    } else {
                        highlighted.push(ch);
                    }
                }
                _ => {
                    if !in_string {
                        current_word.push(ch);
                    } else {
                        highlighted.push(ch);
                    }
                }
            }
        }
        
        if !current_word.is_empty() {
            highlighted.push_str(&self.highlight_word(&current_word));
        }
        
        Cow::Owned(highlighted)
    }
}

#[cfg(feature = "console")]
impl UBasicHelper {
    fn highlight_word(&self, word: &str) -> String {
        let upper_word = word.to_uppercase();
        
        if self.keywords.contains(&upper_word) {
            format!("\x1b[1;34m{}\x1b[0m", word) // Bold blue for keywords
        } else if self.functions.contains(&upper_word) {
            format!("\x1b[1;35m{}\x1b[0m", word) // Bold magenta for functions
        } else if self.variables.contains_key(word) {
            format!("\x1b[1;33m{}\x1b[0m", word) // Bold yellow for variables
        } else if word.parse::<f64>().is_ok() {
            format!("\x1b[1;31m{}\x1b[0m", word) // Bold red for numbers
        } else {
            word.to_string()
        }
    }
}

#[cfg(feature = "console")]
impl Hinter for UBasicHelper {
    type Hint = String;
}

#[cfg(feature = "console")]
impl Validator for UBasicHelper {
    fn validate(&self, input: &rustyline::line_buffer::LineBuffer) -> rustyline::Result<()> {
        // Basic validation - could be enhanced with proper parsing
        if input.is_empty() {
            return Ok(());
        }
        
        let line = input.as_str();
        let mut paren_count = 0;
        let mut in_string = false;
        
        for ch in line.chars() {
            match ch {
                '"' => {
                    in_string = !in_string;
                }
                '(' if !in_string => paren_count += 1,
                ')' if !in_string => {
                    if paren_count == 0 {
                        return Err(rustyline::error::ReadlineError::Io(
                            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unmatched closing parenthesis")
                        ));
                    }
                    paren_count -= 1;
                }
                _ => {}
            }
        }
        
        if paren_count > 0 {
            return Err(rustyline::error::ReadlineError::Io(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unmatched opening parenthesis")
            ));
        }
        
        if in_string {
            return Err(rustyline::error::ReadlineError::Io(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unclosed string")
            ));
        }
        
        Ok(())
    }
}

#[cfg(feature = "console")]
impl Helper for UBasicHelper {}

impl Console {
    /// Create a new console
    pub fn new() -> Self {
        Self {
            prompt: "> ".to_string(),
            history_file: None,
            #[cfg(feature = "console")]
            editor: None,
        }
    }

    /// Create a new console with history file
    pub fn with_history(history_file: String) -> Self {
        Self {
            prompt: "> ".to_string(),
            history_file: Some(history_file),
            #[cfg(feature = "console")]
            editor: None,
        }
    }

    /// Set the prompt
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    /// Print a message
    pub fn print(&self, message: &str) {
        print!("{}", message);
    }

    /// Print with newline
    pub fn println(&self, message: &str) {
        println!("{}", message);
    }

    /// Read a line from the console
    pub fn read_line(&mut self) -> UBasicResult<Option<String>> {
        #[cfg(feature = "console")]
        {
            if self.editor.is_none() {
                let mut editor = Editor::new().map_err(|e| {
                    UBasicError::runtime(format!("Failed to create editor: {}", e), None)
                })?;
                
                editor.set_helper(Some(UBasicHelper::new()));
                
                if let Some(ref history_file) = self.history_file {
                    if let Err(e) = editor.load_history(history_file) {
                        eprintln!("Warning: Could not load history: {}", e);
                    }
                }
                
                self.editor = Some(editor);
            }
            
            if let Some(ref mut editor) = self.editor {
                match editor.readline(&self.prompt) {
                    Ok(line) => {
                        editor.add_history_entry(line.as_str());
                        Ok(Some(line))
                    }
                    Err(ReadlineError::Interrupted) => {
                        self.println("^C");
                        Ok(None)
                    }
                    Err(ReadlineError::Eof) => {
                        self.println("^D");
                        Ok(None)
                    }
                    Err(e) => Err(UBasicError::runtime(format!("Readline error: {}", e), None)),
                }
            } else {
                unreachable!()
            }
        }
        
        #[cfg(not(feature = "console"))]
        {
            use std::io::{self, Write};
            
            print!("{}", self.prompt);
            io::stdout().flush().map_err(|e| {
                UBasicError::runtime(format!("Failed to flush stdout: {}", e), None)
            })?;
            
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => Ok(None), // EOF
                Ok(_) => Ok(Some(line.trim().to_string())),
                Err(e) => Err(UBasicError::runtime(format!("Failed to read line: {}", e), None)),
            }
        }
    }

    /// Update the helper with current variables
    #[cfg(feature = "console")]
    pub fn update_variables(&mut self, variables: &HashMap<String, UBasicValue>) {
        if let Some(ref mut editor) = self.editor {
            if let Some(helper) = editor.helper_mut() {
                helper.update_variables(variables);
            }
        }
    }

    /// Save history to file
    pub fn save_history(&self) -> UBasicResult<()> {
        #[cfg(feature = "console")]
        {
            if let Some(ref editor) = self.editor {
                if let Some(ref history_file) = self.history_file {
                    editor.save_history(history_file).map_err(|e| {
                        UBasicError::runtime(format!("Failed to save history: {}", e), None)
                    })?;
                }
            }
        }
        Ok(())
    }

    /// Clear the screen
    pub fn clear_screen(&self) {
        print!("\x1b[2J\x1b[H");
    }

    /// Show help information
    pub fn show_help(&self) {
        self.println("UBASIC Rust Commands:");
        self.println("  help     - Show this help");
        self.println("  clear    - Clear all variables");
        self.println("  cls      - Clear the screen");
        self.println("  exit     - Exit the interpreter");
        self.println("  quit     - Exit the interpreter");
        self.println("");
        self.println("Examples:");
        self.println("  LET x = 42");
        self.println("  PRINT x");
        self.println("  LET y = 2 + 3 * 4");
        self.println("  PRINT \"Hello, World!\"");
        self.println("  LET z = SIN(PI/2)");
        self.println("  PRINT z");
        self.println("");
        self.println("Mathematical Functions:");
        self.println("  SIN, COS, TAN, ASIN, ACOS, ATAN");
        self.println("  EXP, LOG, LN, SQRT, ABS");
        self.println("  PI, E, GCD, LCM, FACTORIAL");
        self.println("");
        self.println("Control Flow:");
        self.println("  IF condition THEN statements ELSE statements");
        self.println("  FOR variable = start TO end STEP step");
        self.println("  WHILE condition ... WEND");
        self.println("  DO ... LOOP UNTIL condition");
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
} 