# Compiler hints

## Function Hints

Function hints appear before function declarations and affect how the function is compiled and optimized.

### Inlining Hints

| Hint         | Description                                                       |
| ------------ | ----------------------------------------------------------------- |
| `@inline`    | Inline the function at call sites to avoid function call overhead |
| `@no_inline` | Prevent inlining; useful for debugging or controlling code size   |
| `@flatten`   | Inline all function calls within this function's body             |

These hints may be applied at the function declaration level or at specific call sites within a function body.
Call-site hints override function-level hints for that specific invocation.

### Execution Frequency Hints

| Hint    | Description                                                                |
| ------- | -------------------------------------------------------------------------- |
| `@hot`  | Mark function as frequently executed; prioritize optimization for speed    |
| `@cold` | Mark function as rarely executed; optimize for size, move out of hot paths |

### Purity and Side Effect Hints

| Hint     | Description                                                                                                              |
| -------- | ------------------------------------------------------------------------------------------------------------------------ |
| `@pure`  | Function has no side effects; result depends only on arguments. Enables memoization and common subexpression elimination |
| `@const` | Like `@pure`, but also does not read any global or external state                                                        |

### Recursion Hints

| Hint              | Description                                                                 |
| ----------------- | --------------------------------------------------------------------------- |
| `@recursive`      | Hint that function is recursive; enable recursion-specific optimizations    |
| `@no_recurse`     | Promise the function does not recurse; enables stack analysis optimizations |
| `@tail_recursive` | Assert function uses tail recursion; enable tail-call optimization          |
| `@leaf`           | Function does not call other functions; simplifies stack frame              |

### Control Flow Hints

| Hint         | Description                                                       |
| ------------ | ----------------------------------------------------------------- |
| `@no_return` | Function never returns (e.g., `exit()`, `panic()`, infinite loop) |

### Return Value Hints

| Hint                   | Description                                              |
| ---------------------- | -------------------------------------------------------- |
| `@must_use`            | Warn if the function's return value is ignored           |
| `@deprecated`          | Warn when function is called; optionally provide message |
| `@deprecated(message)` | Warn with custom message when function is called         |

### Exception and Error Hints

| Hint        | Description                                                    |
| ----------- | -------------------------------------------------------------- |
| `@throws`   | Function may return an error or panic                          |
| `@no_throw` | Function guarantees it will not panic or return error variants |

## Memory and Allocation Hints

These hints provide information about memory behavior for optimization.

### Function-Level Memory Hints

| Hint          | Description                                                  |
| ------------- | ------------------------------------------------------------ |
| `@no_alloc`   | Function performs no heap allocation                         |
| `@stack_only` | All data used by function stays on the stack                 |
| `@allocator`  | Function returns newly allocated memory (for alias analysis) |

## Variable and Parameter Hints

### Memory Access Hints

| Hint         | Description                                                                       |
| ------------ | --------------------------------------------------------------------------------- |
| `@readonly`  | Data will not be modified through this reference                                  |
| `@writeonly` | Data will only be written, not read                                               |
| `@restrict`  | Pointer/reference does not alias with other pointers (major optimization enabler) |
| `@noalias`   | Alias for `@restrict`                                                             |
| `@volatile`  | Do not optimize away reads/writes; used for hardware registers                    |
| `@immutable` | Value never changes after initialization                                          |

### Nullability Hints

| Hint        | Description                                            |
| ----------- | ------------------------------------------------------ |
| `@nonnull`  | Pointer or reference is guaranteed to never be null    |
| `@nullable` | Pointer or reference may be null (explicit annotation) |

### Alignment Hints

| Hint             | Description                                         |
| ---------------- | --------------------------------------------------- |
| `@aligned(n)`    | Data is aligned to n bytes; enables SIMD operations |
| `@packed`        | Minimize padding in struct layout                   |
| `@cache_aligned` | Align to cache line (typically 64 bytes)            |

#### Storage Hints

| Hint            | Description                                    |
| --------------- | ---------------------------------------------- |
| `@register`     | Keep value in CPU register                     |
| `@thread_local` | Each thread gets its own copy of this variable |
| `@unused`       | Suppress warnings for unused variable          |

## Control Flow Hints

Control flow hints provide branch prediction and reachability information.

### Branch Prediction Hints

| Hint        | Description                        |
| ----------- | ---------------------------------- |
| `@likely`   | Branch condition is probably true  |
| `@unlikely` | Branch condition is probably false |

### Reachability Hints

| Hint                       | Description                                                   |
| -------------------------- | ------------------------------------------------------------- |
| `@unreachable`             | Code path should never execute; enables dead code elimination |
| `@assume(condition)`       | Tell compiler to assume condition is true for optimization    |
| `@expect(value, expected)` | Hint the expected value for branch optimization               |

## Loop Hints

Loop hints appear before loop statements to guide loop optimization.

### Unrolling Hints

| Hint         | Description                                           |
| ------------ | ----------------------------------------------------- |
| `@unroll`    | Unroll the loop completely or use compiler heuristics |
| `@unroll(n)` | Unroll the loop n times                               |
| `@no_unroll` | Do not unroll this loop                               |

### Vectorization Hints

| Hint            | Description                                                      |
| --------------- | ---------------------------------------------------------------- |
| `@vectorize`    | Enable SIMD vectorization for this loop                          |
| `@no_vectorize` | Disable vectorization                                            |
| `@ivdep`        | Ignore vector dependencies (assert no loop-carried dependencies) |

### Parallelization Hints

| Hint           | Description                                     |
| -------------- | ----------------------------------------------- |
| `@parallelize` | Loop iterations are safe to execute in parallel |
| `@distribute`  | Split loop into multiple loops for optimization |

## Concurrency Hints

Concurrency hints provide information about thread safety.

| Hint               | Description                                                             |
| ------------------ | ----------------------------------------------------------------------- |
| `@thread_safe`     | Function is safe to call from multiple threads concurrently             |
| `@not_thread_safe` | Function must not be called concurrently                                |
| `@reentrant`       | Function is safe to call recursively or be interrupted and called again |
| `@atomic`          | Operations are atomic                                                   |
| `@lock_free`       | Implementation does not use locks                                       |
| `@synchronized`    | Function requires external synchronization                              |

## Code Generation Hints

Code generation hints control low-level compilation behavior.
These hints appear before functions or at module level.

### Optimization Level Hints

| Hint               | Description                                                                 |
| ------------------ | --------------------------------------------------------------------------- |
| `@optimize(level)` | Override optimization level: 0 (none), 1, 2, 3 or -1 (optimize binary size) |
| `@no_optimize`     | Disable all optimizations for this function                                 |

### Target-Specific Hints

| Hint               | Description                                                           |
| ------------------ | --------------------------------------------------------------------- |
| `@target(feature)` | Enable specific CPU features for this function (e.g., "avx2", "neon") |
| `@section(name)`   | Place code in specific binary section                                 |

### Linkage and ABI Hints

| Hint               | Description                                             |
| ------------------ | ------------------------------------------------------- |
| `@export`          | Make symbol visible for external linking                |
| `@internal`        | Symbol is internal to compilation unit                  |
| `@weak`            | Weak linkage; can be overridden                         |
| `@abi(convention)` | Use specific calling convention: "c", "fast", "stdcall" |

## Safety and Verification Hints

Safety hints control runtime checks and provide verification information.

### Bounds Checking Hints

| Hint               | Description                                                                          |
| ------------------ | ------------------------------------------------------------------------------------ |
| `@bounds_check`    | Enable bounds checking (default behavior)                                            |
| `@no_bounds_check` | Disable bounds checking; use when performance is critical and bounds are proven safe |

### Overflow Checking Hints

| Hint              | Description                                            |
| ----------------- | ------------------------------------------------------ |
| `@overflow_check` | Enable integer overflow checking                       |
| `@no_overflow`    | Assert that integer overflow will not occur            |
| `@wrapping`       | Use wrapping arithmetic (overflow wraps around)        |
| `@saturating`     | Use saturating arithmetic (overflow clamps to min/max) |

### Unsafe Code Hints

| Hint       | Description                                 |
| ---------- | ------------------------------------------- |
| `@unsafe`  | Mark code as unsafe; disables safety checks |
| `@trusted` | Mark code as trusted; assumes correctness   |

### Pre, Post and Invariant conditions

| Hint                    | Description                                         |
| ----------------------- | --------------------------------------------------- |
| `@pre(condition)`       | Precondition that must hold when function is called |
| `@post(condition)`      | Postcondition that holds when function returns      |
| `@invariant(condition)` | Invariant that holds throughout execution           |
