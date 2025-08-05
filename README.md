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

## Minimal Language Feature Set (v0.1)

## Lexical Analysis
- [x] Keywords: `let`, `if`, `else`, `while`, `fn`, `return` 
- [x] Identifiers (variable and function names)
- [x] Integer literals
- [x] Boolean literals: `true`, `false`
- [ ] Arithmetic Operators: 
    - [x] `+`: Addition
    - [x] `-`: Substraction
    - [ ] `*`: Multiplication
    - [ ] `/`: Division
- [ ] Logical Operators:
    - [x] `==`: Equals
    - [ ] `!=`: Different
    - [ ] `<`: LessThan
    - [ ] `>`: GreaterThan
    - [ ] `<=`: LessEquals
    - [ ] `>=`: GreaterEquals
    - [ ] `&&`: And
    - [ ] `||`: Or
    - [ ] `!`: Not
    - [ ] `^`: Exclusive or (XOR bitwise)
- [x] Other Operators and Punctuation:
    - [x] `=`: Assignment
    - [x] `(`: Opening Parenthesis
    - [x] `)`: Closing Parenthesis
    - [x] `{`: Opening Curly Brace
    - [x] `}`: Closing Curly Brace
    - [x] `->`: Thin Arrow (Type specification)
    - [x] `,`: Comma
    - [x] `;`: Semicolon

### Parsing
- [ ] Variable declarations and assignments
- [ ] Function definitions
- [ ] Function calls with arguments
- [ ] If/else expressions
- [ ] While loops
- [ ] Return statements
- [ ] Expression grouping and operator precedence

### Semantic Analysis
- [ ] Type checking for integers and booleans
- [ ] Function arity and return type checking
- [ ] Variable scope resolution
- [ ] Pointer type recognition (opaque, optional runtime use only)

### Intermediate Representation & Code Generation
- [ ] Arithmetic operations
- [ ] Logical and comparison operations
- [ ] Control flow: conditional branching, loops, function returns
- [ ] Call stack management for function calls
- [ ] Support for integer and boolean operations
- [ ] Pointer-level operations (optional low-level access or address-of/dereference)

### Runtime & Standard Library
- [ ] `print()` for basic output
- [ ] `read()` for basic input (e.g., read integer from stdin)

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
