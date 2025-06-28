# UBASIC Rust

A modern Rust implementation of UBASIC - Advanced BASIC interpreter with mathematical capabilities, graphics support, and interactive console.

## Features

- **Arbitrary Precision Arithmetic**: Using the `rug` crate for high-precision mathematical calculations
- **Complex Numbers**: Full support for complex number operations and mathematical functions
- **Interactive Console**: Modern REPL with syntax highlighting, auto-completion, and command history
- **Graphics Support**: 2D graphics using ggez and egui (optional features)
- **File I/O**: Execute BASIC programs from files
- **Error Handling**: Comprehensive error reporting with line numbers and context
- **Memory Management**: Efficient variable and array management
- **Extensible**: Modular architecture for easy extension

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/ubasic-rust.git
cd ubasic-rust

# Build the project
cargo build --release

# Run in interactive mode
cargo run -- interactive
```

### Basic Usage

```bash
# Interactive mode
cargo run -- interactive

# Execute a BASIC file
cargo run -- --file program.bas

# Evaluate code directly
cargo run -- --eval "PRINT 2 + 2"

# Set precision for calculations
cargo run -- --precision 128 --eval "PRINT PI"
```

### Example BASIC Programs

#### Mathematical Calculations
```basic
LET x = 3.14159
LET y = SIN(x)
PRINT "sin(π) =", y

LET z = (1 + 2i) * (3 + 4i)
PRINT "Complex multiplication:", z
```

#### Variables and Loops
```basic
LET sum = 0
FOR i = 1 TO 10
    LET sum = sum + i
NEXT i
PRINT "Sum of 1 to 10:", sum
```

#### Arrays
```basic
DIM arr(5)
FOR i = 0 TO 4
    LET arr(i) = i * i
NEXT i
PRINT "Array:", arr
```

## Usage Example

### Run a BASIC file

```
cargo run -- examples/hello.bas
```

### Interactive Console

```
cargo run -- --interactive
```

In interactive mode, you can type BASIC commands directly:

```
> LET A = 5
> PRINT A + 2
7
> PRINT "Hello, UBASIC!"
Hello, UBASIC!
> exit
```

## Architecture

The project is organized into several modules:

- **`types.rs`**: Core data types (Integer, Float, Complex, String, Array, etc.)
- **`parser.rs`**: Lexical analysis and parsing of BASIC code
- **`interpreter.rs`**: Execution engine and statement processing
- **`math.rs`**: Mathematical functions and operations
- **`memory.rs`**: Variable and array management
- **`console.rs`**: Interactive console interface
- **`graphics.rs`**: Graphics operations (optional)
- **`errors.rs`**: Error handling and reporting

## Development

### Building

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Build with specific features
cargo build --features "graphics,console"
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_arithmetic
```

### Code Quality

```bash
# Check code formatting
cargo fmt

# Run clippy linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

## Features

### Mathematical Capabilities

- **Arbitrary Precision**: Integer and floating-point arithmetic with configurable precision
- **Complex Numbers**: Full support for complex arithmetic and functions
- **Mathematical Functions**: sin, cos, tan, exp, ln, sqrt, abs, factorial, etc.
- **Constants**: π (pi), e (Euler's number)

### Interactive Console

- **Syntax Highlighting**: Color-coded BASIC syntax
- **Auto-completion**: Variable and function name completion
- **Command History**: Persistent command history across sessions
- **Built-in Commands**: help, clear, exit, etc.

### Graphics Support (Optional)

- **2D Graphics**: Drawing lines, circles, rectangles
- **GUI Framework**: Integration with egui for immediate mode GUI
- **Game Engine**: ggez integration for game development

## Error Handling

The interpreter provides detailed error messages including:

- **Syntax Errors**: Line and column numbers for parsing errors
- **Runtime Errors**: Variable not found, type mismatches, etc.
- **Mathematical Errors**: Division by zero, overflow, etc.
- **Stack Traces**: Function call stack for debugging

## Performance

- **Efficient Parsing**: Fast lexical analysis and parsing
- **Memory Management**: Optimized variable storage and garbage collection
- **Arbitrary Precision**: Configurable precision for mathematical calculations
- **Parallel Processing**: Support for parallel execution (optional)

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust coding conventions
- Add tests for new features
- Update documentation
- Use meaningful commit messages
- Ensure all tests pass before submitting

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original UBASIC by Yuji KIDA
- Rust community for excellent libraries and tools
- Contributors and maintainers

## Roadmap

- [ ] Complete parser implementation
- [ ] Graphics integration
- [ ] WebAssembly support
- [ ] Package manager for BASIC libraries
- [ ] IDE integration
- [ ] Performance optimizations
- [ ] Additional mathematical functions
- [ ] Network and I/O capabilities

## Support

For questions, issues, or contributions:

- Open an issue on GitHub
- Join our discussion forum
- Check the documentation
- Review existing issues and pull requests

---

**UBASIC Rust** - Bringing the power of modern Rust to BASIC programming!
