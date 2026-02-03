# Feature Planning Document

This document discusses design decisions for upcoming fib language features.
Each section presents the problem, explores different approaches, and recommends a direction that aligns with fib's core principles: **developer control**, **zero-cost abstractions**, and **predictable performance**.

---

## Table of Contents

6. [Async/Concurrency](#6-asyncconcurrency)
7. [I/O](#7-io)
8. [Threading](#8-threading)
9. [Libraries, Packaging, and Remote Code](#9-libraries-packaging-and-remote-code)

---

## 6. Async/Concurrency

### Problem Statement

How should fib handle asynchronous programming? You mentioned `yield`.

### Approaches

| Approach                                  | Description                                    | Pros                                        | Cons                                      |
| ----------------------------------------- | ---------------------------------------------- | ------------------------------------------- | ----------------------------------------- |
| **A: Stackful coroutines**                | Each coroutine has own stack, `yield` suspends | Simple mental model, any function can yield | Memory overhead, complex runtime          |
| **B: Stackless coroutines (async/await)** | State machine transformation                   | Zero-cost, no runtime                       | Function coloring, viral async            |
| **C: Green threads**                      | M:N threading with runtime scheduler           | Looks like sync code                        | Hidden runtime, unpredictable performance |
| **D: Algebraic effects**                  | Effects as first-class values                  | Composable, flexible                        | Complex, academic                         |
| **E: Generator/iterator pattern**         | `yield` only in generator functions            | Simple, predictable                         | Limited to iteration                      |
| **F: No built-in async**                  | Library-based (futures, callbacks)             | Minimal core                                | Verbose, no syntactic support             |

### Recommendation: Stackless Coroutines with Explicit Syntax (B)

This aligns with fib's zero-cost abstraction principle and provides predictable performance.

#### Core Design

```fib
// Async function declaration
async function fetch_data(url string) Result string Error {
    let response Response = await http:get(url);
    return response.body;
}

// Async functions return a Future type
// Future is a state machine, zero allocation until polled

// Calling async functions
async function main() {
    let data string = await fetch_data("https://example.com");
    print(data);
}
```

#### The Future Type

```fib
// Built-in or standard library
type Future T = (
    // Compiler-generated state machine
    // Contains all local variables as fields
    // Has poll() method
)

contract Poll T {
    function poll(self) PollResult T;
}

type PollResult T = 'Ready T + 'Pending;
```

#### Executors (Standard Library)

```fib
// Simple single-threaded executor
let executor Executor = Executor:new();
executor:block_on(main());

// Multi-threaded executor
let runtime Runtime = Runtime:new(num_threads: 4);
runtime:spawn(task1());
runtime:spawn(task2());
runtime:block_on_all();
```

#### Generators (Separate Feature)

For iteration, use generators with `yield`:

```fib
// Generator function (not async, for iteration)
generator function range(start int, end int) int {
    for let i = start; i < end; i++ {
        yield i;
    }
}

// Usage
for let x = range(0, 10) {
    print(x);
}

// Generators produce Iterator type
type Iterator T = generator function() T;
```

#### Why This Approach?

1. **Zero-cost**: Futures compile to state machines, no heap allocation
2. **Explicit**: `async`/`await` marks suspension points clearly
3. **Predictable**: No hidden runtime, developer controls executor
4. **Composable**: Futures are values, can be stored and passed

#### Alternative: Effect System (Future Consideration)

For a more unified approach, consider algebraic effects:

```fib
// Effect declaration
effect Async {
    function suspend() unit;
    function resume(value T) T;
}

// Handler-based execution
handle async_computation() with {
| suspend() -> scheduler:park_current()
| resume(v) -> scheduler:wake(v)
}
```

This is more advanced and could be a future addition.

---

## 7. I/O

### Problem Statement

How should I/O be handled? Is it entirely standard library, or are language features needed?

### Approaches

| Approach                            | Description                         | Pros                     | Cons                                |
| ----------------------------------- | ----------------------------------- | ------------------------ | ----------------------------------- |
| **A: Pure standard library**        | All I/O via library functions       | Minimal core             | May lack optimization opportunities |
| **B: Effect-based I/O**             | I/O as effects, handlers in library | Pure functions, testable | Complex                             |
| **C: Capability-based**             | I/O requires capability tokens      | Secure, explicit         | Verbose                             |
| **D: Traditional syscall wrappers** | Thin wrappers over OS               | Fast, predictable        | Platform-specific                   |

### Recommendation: Standard Library with Language Support for Errors (A+D)

#### Core Principle

I/O is inherently side-effecting and platform-specific. Keep it in the standard library but provide good patterns.

#### Error Handling Integration

The language's sum types and `Error` contract already support I/O errors well:

```fib
// I/O functions return Result types
type IoResult T = 'Ok T + 'Err IoError;

function read_file(path string) IoResult string {
    ...
}

// Usage with pattern matching
let content string = match read_file("data.txt") {
| 'Ok data -> data
| 'Err e -> {
    print("Error: " + e:get_error_message());
    return;
}
};
```

#### Standard Library I/O

```fib
// File I/O
module io;

type File = (
    'handle int *  // OS file descriptor
    'mode FileMode
)

function open(path string, mode FileMode) IoResult File;
function read(file File, buffer slice byte) IoResult int;
function write(file File, data slice byte) IoResult int;
function close(file File) IoResult unit;

// Buffered I/O
type BufferedReader = (
    'file File *
    'buffer [4096]byte *
    'pos int *
    'len int
)

function read_line(reader BufferedReader) IoResult string;
```

#### With Statement for Resources

```fib
// RAII-style resource management
with file = io:open("data.txt", 'Read)? {
    let content string = io:read_all(file)?;
    process(content);
}  // file automatically closed

// The `?` operator propagates errors (sugar for match + early return)
```

#### Async I/O

```fib
// Async I/O uses the async system
async function read_file_async(path string) IoResult string {
    let file File = await io:async_open(path, 'Read)?;
    let content string = await io:async_read_all(file)?;
    await io:async_close(file)?;
    return 'Ok content;
}
```

#### Language Features for I/O

Consider these language-level features:

1. **`?` operator**: Early return on error (sugar, not I/O specific)
2. **`with` statement**: Automatic resource cleanup
3. **`@io` hint**: Mark functions that perform I/O (for documentation and optimization)

```fib
// ? operator desugars to:
let x T = expr?;
// becomes:
let x T = match expr {
| 'Ok v -> v
| 'Err e -> return 'Err e
};

// @io hint for documentation
@io
function save_data(data Data) IoResult unit {
    ...
}
```

---

## 8. Threading

### Problem Statement

How does threading work? How does it interact with async and I/O?

### Approaches

| Approach                      | Description                       | Pros                    | Cons                         |
| ----------------------------- | --------------------------------- | ----------------------- | ---------------------------- |
| **A: OS threads via library** | Thin wrappers over pthreads/Win32 | Predictable, no runtime | Manual synchronization       |
| **B: Thread pools**           | Fixed pool of worker threads      | Efficient resource use  | Complexity                   |
| **C: Actor model**            | Message-passing between actors    | No shared state         | Overhead, different paradigm |
| **D: Structured concurrency** | Scoped thread lifetimes           | Safe, leak-free         | Restrictive                  |

### Recommendation: OS Threads + Structured Concurrency (A+D)

#### Core Threading (Standard Library)

```fib
module thread;

type Thread T = (
    'handle ThreadHandle *
    'result ptr T
)

// Spawn a thread
function spawn(func () -> T) Thread T;

// Wait for completion
function join(t Thread T) T;

// Example
let t Thread int = thread:spawn(function() int {
    return expensive_computation();
});
let result int = thread:join(t);
```

#### Structured Concurrency

```fib
// Scoped threads - all must complete before scope exits
thread:scope(function(scope ThreadScope) {
    scope:spawn(function() { task1(); });
    scope:spawn(function() { task2(); });
    scope:spawn(function() { task3(); });
    // All three tasks complete before scope exits
});
// Guaranteed: no dangling threads

// With results
let results []int = thread:scope(function(scope ThreadScope) []int {
    let handles []ThreadHandle int;
    for let i = 0; i < 10; i++ {
        handles:push(scope:spawn(function() int { return compute(i); }));
    }
    return handles:map(function(h) int { return h:join(); });
});
```

#### Synchronization Primitives

```fib
module sync;

// Mutex
type Mutex T = (...);
function lock(m Mutex T) MutexGuard T;

// With automatic unlock
with guard = mutex:lock() {
    guard.value = 42;  // access protected data
}  // automatically unlocked

// Atomic types
type Atomic T = (...);
function load(a Atomic T) T;
function store(a Atomic T, value T);
function compare_exchange(a Atomic T, expected T, desired T) bool;

// Channels
type Channel T = (...);
function send(ch Channel T, value T);
function recv(ch Channel T) Option T;
```

#### Compiler Hints for Threading

```fib
// Mark function as thread-safe
@thread_safe
function get_counter() int {
    return atomic:load(counter);
}

// Mark data as requiring synchronization
@synchronized
let shared_state State;

// Warn if accessed without lock
shared_state.x = 10;  // Warning: accessing @synchronized without lock
```

#### Relationship with Async

```fib
// Async is for I/O-bound concurrency (single thread, many tasks)
// Threading is for CPU-bound parallelism (multiple cores)

// Combine them:
async function parallel_fetch(urls []string) []string {
    // Use thread pool for CPU work, async for I/O
    let results []Future string;
    for let url = urls {
        results:push(fetch(url));  // I/O bound, async
    }
    return await Future:join_all(results);
}

// CPU-bound work should use threads
function parallel_compute(data slice int) []int {
    return thread:parallel_map(data, function(x int) int {
        return expensive_transform(x);
    });
}
```

#### Debugging Support

```fib
// Thread naming for debugging
let t Thread = thread:spawn_named("worker-1", worker_func);

// Debug hints
@debug_thread_id
function log(msg string) {
    print("[Thread " + thread:current_id() + "] " + msg);
}

// Data race detection (debug builds)
// Compiler inserts checks when @synchronized data accessed
```

---

## 9. Libraries, Packaging, and Remote Code

### Problem Statement

How do developers use external code? Static/dynamic linking? Package management?

### Approaches

| Approach                             | Description                               | Pros                     | Cons                        |
| ------------------------------------ | ----------------------------------------- | ------------------------ | --------------------------- |
| **A: Vendoring only**                | Copy dependencies into project            | Simple, reproducible     | Manual updates, large repos |
| **B: Central package registry**      | npm/crates.io style                       | Easy discovery, versions | Single point of failure     |
| **C: Decentralized (Git URLs)**      | Go-style direct imports                   | No registry needed       | Version management harder   |
| **D: Hybrid (registry + vendoring)** | Registry for discovery, vendor for builds | Best of both             | More complexity             |

### Recommendation: Hybrid Approach (D)

#### Project Structure

```
my_project/
├── fib.toml              # Project manifest
├── fib.lock              # Lock file (exact versions)
├── src/
│   ├── main.fib
│   └── lib/
│       └── utils.fib
├── vendor/               # Optional: vendored dependencies
│   └── http/
│       └── ...
└── target/
    └── ...
```

#### Manifest File (fib.toml)

```toml
[package]
name = "my_project"
version = "1.0.0"
authors = ["Developer <dev@example.com>"]

[dependencies]
# From registry
http = "2.1.0"
json = "^1.0"        # semver compatible

# From git
my_lib = { git = "https://github.com/user/my_lib", tag = "v1.0.0" }

# Local path
utils = { path = "../shared/utils" }

[dev-dependencies]
test_helpers = "1.0"

[build]
# Linking preferences
link_type = "static"  # or "dynamic", "prefer-static"
```

#### Import Syntax

```fib
// Import from dependency
import http;
import json:parse;
import json:{ parse, stringify };

// Import from local module
import my_project:utils;

// Qualified vs unqualified
import http;           // use as http:get()
import http:*;         // use as get() (glob import, discouraged)
import http:get as fetch;  // rename
```

#### Package Manager Commands

```bash
# Initialize new project
fib init my_project

# Add dependency
fib add http
fib add http@2.1.0
fib add https://github.com/user/lib --git

# Update dependencies
fib update
fib update http

# Vendor dependencies (copy to vendor/)
fib vendor
fib vendor --all

# Build
fib build
fib build --release

# Run
fib run

# Test
fib test
```

#### Linking

```fib
// In fib.toml
[build]
link_type = "static"  // Default: statically link all dependencies

// Or per-dependency
[dependencies.openssl]
version = "1.1"
link = "dynamic"  // Use system OpenSSL

// FFI for C libraries
[dependencies.sqlite]
version = "3.0"
native = true  // Has C code
link = "static"
```

#### Module Resolution

1. Check local `src/` directory
2. Check `vendor/` directory
3. Check downloaded packages in `~/.fib/packages/`
4. If not found, download from registry/git

#### Vendoring Strategy

```bash
# Vendor all dependencies for reproducible builds
fib vendor --all

# Vendor specific dependency
fib vendor http

# Build using only vendored code (offline)
fib build --frozen
```

#### Security

```toml
# fib.toml
[security]
# Require checksum verification
verify_checksums = true

# Allow/deny specific registries
allowed_registries = ["https://registry.fib-lang.org"]

# Audit for known vulnerabilities
audit = true
```

#### Publishing

```bash
# Publish to registry
fib publish

# Publish to specific registry
fib publish --registry https://private.company.com
```

#### Why This Design?

1. **Reproducibility**: Lock files + vendoring ensure builds are reproducible
2. **Flexibility**: Support registry, git, and local dependencies
3. **Security**: Checksums, auditing, registry control
4. **Simplicity**: Single manifest file, familiar to Cargo/npm users
5. **Offline capable**: Vendoring allows fully offline builds

---

## Summary Matrix

| Feature        | Built-in                      | Standard Library     | Recommendation          |
| -------------- | ----------------------------- | -------------------- | ----------------------- |
| Pointers       | `ptr T`, `addressof`, `deref` | —                    | Keywords for safety     |
| Arenas         | `new(alloc)` syntax           | `Arena`, `Allocator` | Arena-first design      |
| Fixed Arrays   | `[N]T`                        | —                    | Value type, stack       |
| Slices         | `slice T`, `[a..b]`           | —                    | Fat pointer view        |
| Dynamic Arrays | —                             | `Vec T`              | Library type            |
| Maps           | Literal syntax `{ k -> v }`   | `Map K V`            | Hybrid                  |
| Async          | `async`, `await`, `Future`    | Executors            | Stackless coroutines    |
| Generators     | `generator`, `yield`          | —                    | For iteration           |
| I/O            | `?` operator, `with`          | All I/O functions    | Library-based           |
| Threading      | —                             | `thread`, `sync`     | OS threads + structured |
| Packages       | `import`                      | —                    | Manifest + registry     |

---

## Next Steps

1. **Prototype** each feature in isolation
2. **Write tests** that exercise edge cases
3. **Document** each feature thoroughly
4. **Gather feedback** from potential users
5. **Iterate** based on real-world usage

---

## Open Questions

1. Should `ptr` allow arithmetic, or require explicit functions?
2. Should slices be mutable by default, or have `mut_slice T`?
3. How do generics interact with contracts for `Map K V`?
4. Should `async` functions require explicit annotation, or be inferred?
5. What's the default behavior when a thread panics?
6. Should the package registry be centralized or federated?
