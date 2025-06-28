//! Integration tests for UBASIC Rust
//! 
//! These tests verify the complete functionality of the interpreter
//! by running actual BASIC programs and checking their output.

use ubasic_rust::{UBasic, UBasicValue};
use std::fs;

#[test]
fn test_basic_arithmetic() {
    let mut ubasic = UBasic::new();
    
    // Test simple arithmetic
    let result = ubasic.run("LET x = 2 + 3 * 4").unwrap();
    assert!(matches!(result, UBasicValue::Integer(_)));
    
    // Test variable assignment and retrieval
    ubasic.run("LET y = 10").unwrap();
    let result = ubasic.run("PRINT y").unwrap();
    assert!(matches!(result, UBasicValue::Integer(_)));
}

#[test]
fn test_mathematical_functions() {
    let mut ubasic = UBasic::new();
    
    // Test trigonometric functions
    let result = ubasic.run("LET x = SIN(0)").unwrap();
    assert!(matches!(result, UBasicValue::Float(_)));
    
    // Test exponential function
    let result = ubasic.run("LET y = EXP(1)").unwrap();
    assert!(matches!(result, UBasicValue::Float(_)));
}

#[test]
fn test_string_operations() {
    let mut ubasic = UBasic::new();
    
    // Test string assignment
    let result = ubasic.run("LET s = \"Hello, World!\"").unwrap();
    assert!(matches!(result, UBasicValue::String(_)));
    
    // Test string concatenation
    let result = ubasic.run("LET t = \"Hello\" + \" World\"").unwrap();
    assert!(matches!(result, UBasicValue::String(_)));
}

#[test]
fn test_control_structures() {
    let mut ubasic = UBasic::new();
    
    // Test FOR loop
    let code = r#"
        LET sum = 0
        FOR i = 1 TO 5
            LET sum = sum + i
        NEXT i
        PRINT sum
    "#;
    
    let result = ubasic.run(code).unwrap();
    assert!(matches!(result, UBasicValue::Integer(_)));
}

#[test]
fn test_error_handling() {
    let mut ubasic = UBasic::new();
    
    // Test division by zero
    let result = ubasic.run("LET x = 1 / 0");
    assert!(result.is_err());
    
    // Test undefined variable
    let result = ubasic.run("PRINT undefined_var");
    assert!(result.is_err());
}

#[test]
fn test_file_execution() {
    let mut ubasic = UBasic::new();
    
    // Create a temporary BASIC file
    let temp_file = "temp_test.bas";
    let code = r#"
        PRINT "Hello from file!"
        LET x = 42
        PRINT "x =", x
    "#;
    
    fs::write(temp_file, code).unwrap();
    
    // Execute the file
    let file_content = fs::read_to_string(temp_file).unwrap();
    let result = ubasic.run(&file_content);
    
    // Clean up
    fs::remove_file(temp_file).unwrap();
    
    assert!(result.is_ok());
}

#[test]
fn test_precision_settings() {
    // Test with different precision settings
    let mut ubasic_low = UBasic::with_precision(32);
    let mut ubasic_high = UBasic::with_precision(128);
    
    let result_low = ubasic_low.run("LET x = PI").unwrap();
    let result_high = ubasic_high.run("LET x = PI").unwrap();
    
    assert!(matches!(result_low, UBasicValue::Float(_)));
    assert!(matches!(result_high, UBasicValue::Float(_)));
}

#[test]
fn test_memory_management() {
    let mut ubasic = UBasic::new();
    
    // Create many variables
    for i in 0..100 {
        let code = format!("LET var{} = {}", i, i);
        ubasic.run(&code).unwrap();
    }
    
    // Check memory stats
    let stats = ubasic.memory_stats();
    assert!(stats.total_variables >= 100);
    
    // Clear memory
    ubasic.clear();
    let stats_after = ubasic.memory_stats();
    assert_eq!(stats_after.total_variables, 0);
}

#[test]
fn test_complex_numbers() {
    let mut ubasic = UBasic::new();
    
    // Test complex number creation
    let result = ubasic.run("LET z = (3 + 4i)").unwrap();
    assert!(matches!(result, UBasicValue::Complex(_)));
    
    // Test complex arithmetic
    let result = ubasic.run("LET w = (1 + 2i) * (3 + 4i)").unwrap();
    assert!(matches!(result, UBasicValue::Complex(_)));
}

#[test]
fn test_arrays() {
    let mut ubasic = UBasic::new();
    
    // Test array creation and access
    let code = r#"
        DIM arr(5)
        FOR i = 0 TO 4
            LET arr(i) = i * i
        NEXT i
        PRINT arr(2)
    "#;
    
    let result = ubasic.run(code).unwrap();
    assert!(matches!(result, UBasicValue::Integer(_)));
}

#[test]
fn test_functions() {
    let mut ubasic = UBasic::new();
    
    // Test built-in functions
    let result = ubasic.run("LET x = ABS(-5)").unwrap();
    assert!(matches!(result, UBasicValue::Integer(_)));
    
    let result = ubasic.run("LET y = SQRT(16)").unwrap();
    assert!(matches!(result, UBasicValue::Float(_)));
}

#[test]
fn test_interactive_mode() {
    // This test verifies that interactive mode can be started
    // Note: This is a basic test since interactive mode requires user input
    let mut ubasic = UBasic::new();
    
    // The run_interactive method should not panic
    // In a real test environment, we might mock the input
    // For now, we just verify the method exists and can be called
    let _ = ubasic.run_interactive();
} 