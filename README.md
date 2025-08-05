# Fiber (Klog Compiler)

This is an early-stage toy compiler project for Klog, a C-inspired language 
designed for performance, clarity, and modern language features. The goal is to 
create a language and toolchain that combines low-level control with safer 
abstractions, ownership models, and ergonomics suited for systems programming 
and backend development.

The compiler, called Fiber, is written in Rust and will support incremental 
compilation, LLVM backend, and a modular package system.

## Current Status

At this stage, the compiler can read source files and tokenize input code into 
a stream of tokens. The lexer is the foundation for the next phases: parsing, 
type checking, IR generation, and code emission.

The project is under active development and meant for learning, experimentation, 
and laying the groundwork for a modern systems programming language.

## Features

Below is an overview of the desired features of the language

- C-like syntax
- Memory model with explicit garbage-collected (`@`) and manual (`man`) allocations
- Structs, tagged unions, and basic pattern matching
- Interfaces with per-method explicit implementation syntax
- Support for destructors and simple RAII for `man` types
- File-based modules and a basic standard library
- Downcasting via `type_id` matching on interface values
- Support for coroutines or green threads.
- Multi-pass, IR-based, AOT compiler
  - Front-end: parsing, type checking, symbol resolution
  - Middle-end: lowering to SSA, optimization, monomorphization
  - Back-end: LLVM IR generation
- Incremental compilation with module caching and API hashing
- Lazy compilation of unused functions/types
- Whole-program release mode with cross-module optimization
- Fiber CLI for compilation, dependency resolution and testing
- Git-based external package management with version pinning

## Getting Started

Clone the repository and build the compiler:

```
git clone https://github.com/santi34mg/fiber.git
cd fiber
cargo build --release
```

### Compile a `.klg` source file:

```
./target/release/fiber compile path/to/file.klg
```

Currently, this will tokenize the input and print tokens to stdout as a debugging step.

## Contributing

Contributions, suggestions, and bug reports are welcome. This project is a personal exploration of compiler design and language implementation, but any help or feedback is appreciated.

## License

MIT License. See [LICENSE](LICENSE) for details.
