# Rust Learning Project Plan

## Project Overview
A simple temperature converter application that demonstrates Rust fundamentals and best practices.

## What You'll Learn

### 1. Project Structure
- Standard Rust project layout with `src/` directory
- `Cargo.toml` - Rust's package manifest (like `package.json` in Node.js)
- `Cargo.lock` - Dependency lock file (auto-generated)
- Separating library code from application code

### 2. Core Rust Concepts
- **Functions**: Basic function syntax, parameters, and return values
- **Types**: Strong typing with `f64`, `String`, etc.
- **Ownership**: Rust's unique memory management (borrowing with `&`)
- **Error Handling**: Using `Result<T, E>` for safe error handling
- **Modules**: Organizing code with `lib.rs` and `main.rs`

### 3. Project Components

#### a. Library Code (`src/lib.rs`)
A reusable library with temperature conversion functions:
- Celsius to Fahrenheit conversion
- Fahrenheit to Celsius conversion
- Input validation (ensuring temperatures are above absolute zero)
- Public API using `pub` keyword

#### b. Unit Tests
Tests embedded in the library file using:
- `#[cfg(test)]` attribute
- `#[test]` attribute for test functions
- `assert!` and `assert_eq!` macros
- Testing both success and error cases

#### c. Main Application (`src/main.rs`)
A command-line application that:
- Uses the library functions
- Takes user input from command-line arguments
- Handles errors gracefully
- Demonstrates importing and using library code

#### d. Build Script (`build.rs`)
A pre-compilation script that:
- Runs before your code compiles
- Sets environment variables
- Demonstrates build-time customization
- Shows how to pass compile-time information to your app

#### e. External Dependency
Using a popular crate (library) from crates.io:
- `clap` for command-line argument parsing (user-friendly CLI)
- Shows how to add dependencies in `Cargo.toml`
- Demonstrates using external libraries

### 4. Cargo Commands You'll Use

```bash
# Create a new project (you may have already done this)
cargo new project-name

# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run

# Run with arguments
cargo run -- celsius 25

# Run tests
cargo test

# Check code without building (faster)
cargo check

# Format code
cargo fmt

# Lint code for issues
cargo clippy
```

### 5. File Structure
```
rust1/
├── Cargo.toml          # Project manifest and dependencies
├── Cargo.lock          # Dependency lock file (auto-generated)
├── build.rs            # Build script
├── src/
│   ├── lib.rs          # Library code with conversion functions
│   └── main.rs         # Main application entry point
└── LEARNING_PLAN.md    # This file
```

### 6. Learning Path

1. **Start with `Cargo.toml`**
   - Understand package metadata
   - Add the `clap` dependency
   - Configure build script

2. **Build the Library (`lib.rs`)**
   - Write simple conversion functions
   - Add input validation
   - Write comprehensive unit tests
   - Run tests with `cargo test`

3. **Create the Build Script (`build.rs`)**
   - Generate build timestamp
   - Set environment variables
   - See how it affects compilation

4. **Implement Main Application (`main.rs`)**
   - Import library functions
   - Parse command-line arguments with `clap`
   - Handle user input and display results
   - Use `match` for error handling

5. **Run and Test**
   - Build the project
   - Test different inputs
   - See error handling in action

### 7. Key Rust Concepts Demonstrated

- **Ownership & Borrowing**: Functions borrow values with `&` instead of taking ownership
- **Pattern Matching**: Using `match` for elegant error handling
- **Type System**: Strong typing prevents runtime errors
- **Testing**: First-class testing support built into the language
- **Documentation**: Using `///` for doc comments
- **Modules**: Separating concerns between library and application
- **Cargo Ecosystem**: Dependency management and build tools

### 8. Next Steps After Completion

Once you understand this project:
- Add more conversion functions (Kelvin, etc.)
- Write integration tests in `tests/` directory
- Add benchmarks with `cargo bench`
- Publish your library to crates.io
- Explore more complex data structures (structs, enums)
- Learn about lifetimes and advanced borrowing

## Why This Project?

This project is ideal for learning because:
- It's simple enough to understand quickly
- It demonstrates real-world Rust patterns
- You'll use cargo commands daily as a Rust developer
- It shows the separation of library and application code
- Testing is integrated from the start
- It's practical - you can actually use it!

## Working with Claude Code

Since you'll be using Claude Code to build this project, here's how Claude Code will help you:

### Iterative Development Workflow

1. **Write Code**
   - Claude Code will create the initial files based on Rust best practices
   - Code will be written incrementally, one component at a time

2. **Compile and Get Feedback**
   - After writing code, Claude Code will run `cargo build` or `cargo check`
   - Rust compiler errors will be captured and analyzed
   - Claude Code will explain what the errors mean in beginner-friendly terms
   - Code will be fixed based on compiler feedback

3. **Run Tests**
   - After library code is written, Claude Code will run `cargo test`
   - Test output will be analyzed (passes, failures, panics)
   - If tests fail, Claude Code will debug and fix the issues
   - You'll see the test-driven development cycle in action

4. **Run the Application**
   - Once tests pass, Claude Code will run `cargo run` with various arguments
   - Runtime errors will be caught and fixed
   - You'll see the complete working application

5. **Code Quality Tools**
   - Claude Code will run `cargo clippy` to catch common mistakes
   - Will run `cargo fmt` to format code to Rust standards
   - You'll learn Rust idioms through these suggestions

### What Makes This Ideal for Learning with Claude Code

- **Fast Feedback Loop**: You'll see compile → error → fix → success cycles
- **Learn from Mistakes**: Rust's compiler errors are educational, and Claude Code will explain them
- **Best Practices**: Claude Code will demonstrate idiomatic Rust from the start
- **Incremental Building**: Each component is built and tested before moving to the next
- **Real Development**: You'll experience the actual Rust development workflow

### Example Workflow You'll See

```
1. Claude Code writes src/lib.rs with conversion functions
2. Runs: cargo build
3. Compiler error: "missing lifetime specifier" (example)
4. Claude Code explains the error and fixes it
5. Runs: cargo build again - SUCCESS!
6. Runs: cargo test
7. Test output shows which tests pass/fail
8. If failures, Claude Code debugs and fixes
9. Runs: cargo test again - ALL PASS!
10. Moves to next component (main.rs)
```

### Commands Claude Code Will Use

- `cargo check` - Fast compilation check without building binary
- `cargo build` - Full compilation to catch all errors
- `cargo test` - Run unit tests and show results
- `cargo test --verbose` - Detailed test output
- `cargo run -- <args>` - Run the application with arguments
- `cargo clippy` - Linting for common issues
- `cargo fmt` - Format code to standard style

This iterative approach means you'll learn by seeing real errors and their fixes, not just working code.

## Resources

- Official Rust Book: https://doc.rust-lang.org/book/
- Cargo Book: https://doc.rust-lang.org/cargo/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- crates.io: https://crates.io/ (search for libraries)
