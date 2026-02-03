# Language Philosphy

fib is a systems language with a focous on performance and expressiveness. Developer control is the core principal of the language.

fib follows the imperative llanguage paradigm.

## Principles

As stated previously, fib mainly follows two principles: developer control and performance.

### Developer control

The term _developer control_ refers to the principle that developers' intentions and objectives guide and determine the way code is produced instead of already existing abstractions taking over and limiting the way developers can write code.

To uphold the principle of developer control, fib intends to provide language features that aid the developer in clearly expressing what the code intends to do. Declarative programming should be avoided as it obscures implementation details.

#### Memory control

Following the principle of developer control, developers should have explicit control over how, when and where memory is allocated. Always.

By giving developers control over memory allocation, performance can be optimized.

#### Error handling

Errors are treated as a normal occurance in programming. Errors are _not_ exceptions.

In fib, error are values and can be treated as any other value of the language.

Callers of potentially erroring functions need to address these errors explicitly.
Callers can also check what potential errors can be caused by such functions thanks to the [type system](language_specification.md#types).

By pushing the responsability of error handling onto callers and making errors values instead of exceptions, error handling is made explicit and can be resolved statically.

Some errors might be unrecoverable. For these instances the language allows panicking (through the use of the [panic](language_specification.md#panic) keyword).

### Performance

Performance is of utmost concern in fib.
As a systems language, fib is designed to produce highly efficient code that can compete with hand-written low-level implementations.

#### Zero-cost abstractions

Abstractions in fib should compile down to code equivalent to hand-written low-level implementations.
When you use high-level language features, you should pay no runtime overhead compared to writing the equivalent code manually.

This principle means that language features should not introduce hidden costs.
Every operation should have a clear and predictable performance profile.
For example, iterating over a collection using a high-level abstraction should generate the same machine code as manually indexing into that collection.
Virtual dispatch, dynamic typing, or reflection mechanisms that add indirection layers are avoided unless explicitly requested.

Unused features should not impact the performance of generated code.
If a developer doesn't use a particular language feature, their binary should be just as fast and small as if that feature didn't exist at all.
Dead code elimination and whole-program optimization ensure that only the code paths actually used by the program make it into the final executable.
Developers pay only for what they use, both in terms of binary size and runtime performance.

The compiler aggressively optimizes away abstraction layers.
Contract types are monomorphized to concrete implementations.
Higher-order functions are inlined.
Iterator chains are fused into single loops.
Pattern matching compiles to efficient jump tables or conditional branches.

The goal is to give developers the tools to write expressive, maintainable code while ensuring the compiler transforms it into the most efficient machine code possible.
Zero-cost abstractions enable developers to structure their programs clearly without worrying about performance penalties.

#### Compile-time computation

fib encourages moving computation from runtime to compile-time wherever possible.
By evaluating expressions and executing functions during compilation, the language reduces runtime overhead and enables additional optimizations.

This includes constant evaluation of expressions known at compile-time, compile-time function execution for pure functions with constant inputs, static guarantees that eliminate runtime checks, and type-level computation for zero-runtime-cost generic specialization.

This approach results in faster executables with smaller binary sizes while maintaining safety guarantees.

#### Predictable performance

Performance characteristics in fib should be predictable and transparent.
Developers must be able to reason about the performance of their code without surprises or hidden costs.

To ensure predictability, the language avoids garbage collection pauses or stop-the-world events, provides deterministic memory management that developers control, documents clear performance characteristics for all language constructs, prohibits implicit allocations or expensive operations, and maintains consistent timing behavior for real-time and embedded systems.

Predictable performance is essential for systems programming where timing guarantees matter.

#### Low-level control with high-level ergonomics

fib provides direct access to hardware and system resources while maintaining ergonomic high-level constructs.

Developers should never feel constrained by the language when they need fine-grained control.
This includes direct memory access and pointer manipulation when needed, inline assembly support for critical performance sections, control over data layout, alignment, and padding, SIMD operations and vector types for data parallelism, direct system call interfaces, and bit-level manipulation primitives.

The goal is to make low-level operations accessible without sacrificing the expressiveness of high-level code.

#### Efficient defaults

The language should guide developers toward efficient patterns through sensible defaults.
Common operations should be fast by default, with explicit syntax required for potentially expensive operations.

Efficient defaults include stack allocation by default with explicit heap allocation, pass-by-value semantics that optimize well for small types, immutability as default to enable compiler optimizations, zero-initialization only when explicitly requested or required for safety, inlining hints and automatic inlining of small functions, and efficient string and collection representations.
By making the efficient path the easy path, fib helps developers write fast code naturally.

#### Transparent costs

All operations in fib should have transparent, visible costs.
Expensive operations must be explicit in the code so developers can identify performance bottlenecks by inspection.

Principles of cost transparency include requiring explicit syntax for allocations (e.g., special operators or keywords), ensuring deep copies are never implicit, maintaining a clear distinction between cheap operations (moves) and expensive ones (clones), prohibiting operator overloading that hides complexity, using obvious syntax for performance-critical operations, and ensuring documentation clearly indicates computational complexity.

This transparency empowers developers to write performant code from the start and easily identify optimization opportunities.

## Variable semantics

Variables are names for _locations_. A variable binds a location and its valua can change.

## Language feature core

The language tries to be as simple as possible, providing abstractions not by expanding the feature set but by building on top of what has already been stabilshed.

This simplicity also gives developers a tight core they can always rely on without worrying over new abstractions.
