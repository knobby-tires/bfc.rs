# BrainFuck Compiler and Debugger

A  Rust implementation of a BrainFuck compiler featuring lexical analysis, parsing, optimization, and debugging capabilities. This project provides both a compiler and interactive debugger for BrainFuck programs with real-time memory visualization and execution statistics.

## Features

### Core Functionality
- Lexical analysis 
- Abstract Syntax Tree (AST) parsing
- Code optimization (combining repeated operations)
- Code generation (BrainFuck to Rust)
- Interactive interpreter

### Debug Features
- Step-by-step execution
- Memory state visualization
- Execution statistics tracking
- Breakpoint system:
  - Break at specific instruction counts
  - Break on memory values
  - Break at loop depths
- Detailed execution metrics
  - Instruction counts
  - Timing per instruction type
  - Loop iteration tracking

## Installation

1. Ensure you have Rust and Cargo installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:
```bash
git clone https://github.com/yourusername/brainfuck-compiler
cd brainfuck-compiler
```

3. Build the project:
```bash
cargo build --release
```

## Usage

### Basic Usage
```bash
# Run the Hello World example
cargo run

# Run a program from a file
cargo run program.bf

# Run a program directly
cargo run -p "+++++."

# Debug Options
cargo run --debug          # Enable debug mode
cargo run --step          # Enable step-by-step execution
cargo run --stats         # Show execution statistics

# Combine options
cargo run program.bf --debug --step --stats
```

### Example Programs

#### 1. Hello World
```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```
Output: `Hello World!`

#### 2. Simple Addition (adds 2 + 3)
```brainfuck
++>+++<[->+<]>.
```
Output: `5`

#### 3. Cat Program (echoes input)
```brainfuck
,[.,]
```

### Understanding Debug Output

When running with --debug, you'll see:
```
Step 1:
Loop depth: 0
Executing: Add(2)
Pointer: 0
Memory around pointer: [(0, 2), (1, 0), (2, 0)]

Execution Statistics:
Total instructions executed: 7
Instruction counts:
Add(3): 1 times
Loop([Decrement]): 1 times
...
```

- Step: Current instruction number
- Loop depth: Current nesting level of loops
- Memory around pointer: Shows memory cell values around current pointer
- Execution Statistics: Detailed performance metrics

## Testing

Run the complete test suite:
```bash
cargo test
```

Individual test categories:
```bash
cargo test lexer     # Test lexical analysis
cargo test parser    # Test parsing
cargo test optimizer # Test optimizations
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---