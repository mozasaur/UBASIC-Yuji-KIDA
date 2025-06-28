//! Benchmarks for UBASIC Rust
//! 
//! These benchmarks measure the performance of various interpreter operations
//! to ensure optimal performance and identify bottlenecks.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ubasic_rust::{UBasic, UBasicValue};
use rug::Integer;

fn benchmark_arithmetic(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("simple_addition", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET x = 1 + 2")).unwrap();
        });
    });
    
    c.bench_function("complex_arithmetic", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET x = (2 + 3) * 4 - 1")).unwrap();
        });
    });
    
    c.bench_function("floating_point", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET x = 3.14159 * 2.71828")).unwrap();
        });
    });
}

fn benchmark_mathematical_functions(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("trigonometric", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET x = SIN(0.5)")).unwrap();
        });
    });
    
    c.bench_function("exponential", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET x = EXP(1.0)")).unwrap();
        });
    });
    
    c.bench_function("square_root", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET x = SQRT(16.0)")).unwrap();
        });
    });
}

fn benchmark_variable_operations(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("variable_assignment", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET x = 42")).unwrap();
        });
    });
    
    c.bench_function("variable_retrieval", |b| {
        ubasic.run("LET x = 42").unwrap();
        b.iter(|| {
            ubasic.run(black_box("PRINT x")).unwrap();
        });
    });
    
    c.bench_function("multiple_variables", |b| {
        b.iter(|| {
            for i in 0..10 {
                let code = format!("LET var{} = {}", i, i);
                ubasic.run(&code).unwrap();
            }
        });
    });
}

fn benchmark_loops(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("for_loop_small", |b| {
        b.iter(|| {
            ubasic.run(black_box(r#"
                LET sum = 0
                FOR i = 1 TO 10
                    LET sum = sum + i
                NEXT i
            "#)).unwrap();
        });
    });
    
    c.bench_function("for_loop_medium", |b| {
        b.iter(|| {
            ubasic.run(black_box(r#"
                LET sum = 0
                FOR i = 1 TO 100
                    LET sum = sum + i
                NEXT i
            "#)).unwrap();
        });
    });
}

fn benchmark_string_operations(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("string_assignment", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET s = \"Hello, World!\"")).unwrap();
        });
    });
    
    c.bench_function("string_concatenation", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET t = \"Hello\" + \" World\"")).unwrap();
        });
    });
}

fn benchmark_complex_numbers(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("complex_creation", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET z = (3 + 4i)")).unwrap();
        });
    });
    
    c.bench_function("complex_arithmetic", |b| {
        b.iter(|| {
            ubasic.run(black_box("LET w = (1 + 2i) * (3 + 4i)")).unwrap();
        });
    });
}

fn benchmark_arrays(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("array_creation", |b| {
        b.iter(|| {
            ubasic.run(black_box("DIM arr(10)")).unwrap();
        });
    });
    
    c.bench_function("array_access", |b| {
        ubasic.run("DIM arr(10)").unwrap();
        ubasic.run("LET arr(5) = 42").unwrap();
        b.iter(|| {
            ubasic.run(black_box("PRINT arr(5)")).unwrap();
        });
    });
}

fn benchmark_error_handling(c: &mut Criterion) {
    let mut ubasic = UBasic::new();
    
    c.bench_function("division_by_zero", |b| {
        b.iter(|| {
            let _ = ubasic.run(black_box("LET x = 1 / 0"));
        });
    });
    
    c.bench_function("undefined_variable", |b| {
        b.iter(|| {
            let _ = ubasic.run(black_box("PRINT undefined_var"));
        });
    });
}

fn benchmark_precision_comparison(c: &mut Criterion) {
    c.bench_function("precision_32", |b| {
        let mut ubasic = UBasic::with_precision(32);
        b.iter(|| {
            ubasic.run(black_box("LET x = PI")).unwrap();
        });
    });
    
    c.bench_function("precision_64", |b| {
        let mut ubasic = UBasic::with_precision(64);
        b.iter(|| {
            ubasic.run(black_box("LET x = PI")).unwrap();
        });
    });
    
    c.bench_function("precision_128", |b| {
        let mut ubasic = UBasic::with_precision(128);
        b.iter(|| {
            ubasic.run(black_box("LET x = PI")).unwrap();
        });
    });
}

fn benchmark_memory_operations(c: &mut Criterion) {
    c.bench_function("memory_allocation", |b| {
        let mut ubasic = UBasic::new();
        b.iter(|| {
            for i in 0..100 {
                let code = format!("LET var{} = {}", i, i);
                ubasic.run(&code).unwrap();
            }
        });
    });
    
    c.bench_function("memory_clear", |b| {
        let mut ubasic = UBasic::new();
        for i in 0..100 {
            let code = format!("LET var{} = {}", i, i);
            ubasic.run(&code).unwrap();
        }
        b.iter(|| {
            ubasic.clear();
        });
    });
}

criterion_group!(
    benches,
    benchmark_arithmetic,
    benchmark_mathematical_functions,
    benchmark_variable_operations,
    benchmark_loops,
    benchmark_string_operations,
    benchmark_complex_numbers,
    benchmark_arrays,
    benchmark_error_handling,
    benchmark_precision_comparison,
    benchmark_memory_operations
);

criterion_main!(benches); 