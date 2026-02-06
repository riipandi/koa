# Implementation Plan

Roadmap for Koa implementation from start to production-ready.

## Overview

Total timeline: **~40-54 weeks** (9-12 months) for 1 full-time developer.

---

## Phase 1: Core Foundation (4-6 weeks)

**Goal**: Basic executable programs

### Deliverables

- ✅ Lexer + Parser
- ✅ AST definitions
- ✅ Basic type checker
- ✅ LLVM IR generation
- ✅ Hello World example

### Tasks

**Week 1-2: Lexer**
- Token types dan definitions
- Lexer implementation (handwritten recursive descent)
- Error handling dengan line/column info
- Unicode support (UTF-8)

**Week 3-4: Parser**
- Grammar definition
- Parser implementation (lalrpop or handmade)
- AST node types
- Error recovery

**Week 5-6: Type Checker & Codegen**
- Primitive types: i32, f64, bool, string
- Basic type checker
- Functions & calls
- LLVM IR generation
- Hello World program

### Example Output

```typescript
// Hello World
fn main(): i32 {
    println!("Hello, World!")
    0
}
```

---

## Phase 2: Generics & Type System (6-8 weeks)

**Goal**: Full generics support

### Deliverables

- ✅ Type parameters
- ✅ Monomorphization
- ✅ Generic functions
- ✅ Generic structs
- ✅ Type inference

### Tasks

**Week 1-2: Type Parameters**
- Type parameter syntax
- Generic function definitions
- Generic struct definitions

**Week 3-4: Monomorphization**
- Type instantiation
- Code generation for generics
- Type deduplication

**Week 5-6: Type Inference**
- Local type inference (limited)
- Function return type inference
- Generic call inference

**Week 7-8: Testing**
- Unit tests for type checker
- Integration tests

### Example Output

```typescript
fn identity<T>(x: T): T {
    x
}

struct Vec<T> {
    data: *mut T,
    len: usize,
}
```

---

## Phase 3: Control Flow & Patterns (4-6 weeks)

**Goal**: Complete control flow

### Deliverables

- ✅ if/else expressions
- ✅ while/loop/for loops
- ✅ match expressions (pattern matching)
- ✅ defer/errdefer

### Tasks

**Week 1-2: Control Flow**
- if/else expressions
- while/loop/for loops
- break/continue

**Week 3-4: Pattern Matching**
- match expressions
- Enum patterns
- Struct destructuring
- Exhaustiveness checking

**Week 5-6: Cleanup**
- defer statements
- errdefer statements

### Example Output

```typescript
fn classify(n: i32): string {
    switch n {
        0 => "Zero",
        1 | 2 | 3 => "Small",
        _ => "Other",
    }
}
```

---

## Phase 4: Memory Management & GC (8-10 weeks)

**Goal**: Working concurrent GC

### Deliverables

- ✅ Bump pointer allocator
- ✅ Concurrent tri-color mark-sweep GC
- ✅ Write barriers
- ✅ Stack maps
- ✅ GC integration

### Tasks

**Week 1-2: Allocator**
- Bump pointer allocator
- Arena allocation
- Heap allocation intrinsics

**Week 3-4: GC Implementation**
- Tri-color marking
- Mark setup/termination
- Concurrent sweeping
- Write barriers (Dijkstra)

**Week 5-6: Compiler Integration**
- Stack map generation
- Root set identification
- Safe point insertion
- GC pacing (GOGC)

**Week 7-8: Testing**
- Stress tests
- Memory leak tests
- Performance benchmarks

**Week 9-10: Optimization**
- Escape analysis
- Stack vs heap allocation decisions

### Example Output

```typescript
fn create_tree(depth: i32): Node | null {
    if depth == 0 {
        return null
    }
    Node {
        value: depth,
        left: create_tree(depth - 1),
        right: create_tree(depth - 1),
    }
}
```

---

## Phase 5: Concurrency (8-10 weeks)

**Goal**: Async/await runtime

### Deliverables

- ✅ Async/await syntax
- ✅ Event loop runtime
- ✅ Async I/O
- ✅ Task scheduling

### Tasks

**Week 1-2: Async Syntax**
- async function definitions
- await expressions
- Future types

**Week 3-4: Event Loop**
- Event loop implementation
- Task scheduling
- Timer wheel

**Week 5-6: Async I/O**
- Non-blocking I/O
- Async file operations
- Async networking

**Week 7-8: Integration**
- Async runtime integration
- Standard library async functions

**Week 9-10: Testing**
- Async tests
- Performance tests

### Example Output

```typescript
async fn fetch(url: string): !Data {
    let response: HttpResponse = await http_get(url)
    response.data
}
```

---

## Phase 6: Modules & Stdlib (6-8 weeks)

**Goal**: Module system + basic stdlib

### Deliverables

- ✅ File-based modules
- ✅ import/export system
- ✅ Basic standard library

### Tasks

**Week 1-2: Module System**
- File = module
- import/export syntax
- Module resolution
- Visibility rules

**Week 3-4: Standard Library - Core**
- std/io (print, file I/O)
- std/collections (Vec, HashMap)
- std/error (common errors)

**Week 5-6: Standard Library - Extended**
- std/net (HTTP, TCP)
- std/time (sleep, timestamp)
- std/math (math functions)

**Week 7-8: Testing & Polish**
- Stdlib tests
- Documentation

### Example Output

```typescript
import { Vec, HashMap } from "std/collections"
import { println } from "std/io"

fn main(): i32 {
    let numbers: Vec<i32> = Vec::new()
    try numbers.push(42)
    println!("{}", numbers.get(0))
    0
}
```

---

## Phase 7: Error Handling & Tooling (4-6 weeks)

**Goal**: Developer experience

### Deliverables

- ✅ Zig-style error handling
- ✅ Compiler error messages
- ✅ Package manager
- ✅ Build system

### Tasks

**Week 1-2: Error Handling**
- Error sets
- Error union types
- try/catch keywords
- Error return traces

**Week 3: Package Manager**
- Koa.toml parsing
- Koa.lock generation
- Git dependency fetching
- Dependency resolution

**Week 4: Build System**
- Build modes (debug, release-safe, release-fast, release-small)
- Conditional compilation
- Annotations (@debug, @release, etc.)

**Week 5-6: Tooling**
- koa build
- koa run
- koa test
- koa fetch
- Error messages yang jelas

### Example Output

```typescript
const FileError = error {
    NotFound,
    AccessDenied,
}

fn read_file(path: string): FileError!string {
    try open_file(path)
}
```

---

## Phase 8: HMR (Hot Module Reload) (6-8 weeks)

**Goal**: Development experience dengan fast iteration

### Deliverables

- File watcher
- Recompilation on change
- Live code reloading
- State preservation

### Tasks

**Week 1-2: File Watching**
- File system watcher
- Change detection
- Debouncing

**Week 3-4: Incremental Compilation**
- Cache compiled modules
- Recompile only changed files
- Dependency tracking

**Week 5-6: Live Reload**
- Hot code swapping
- State preservation
- Error recovery

**Week 7-8: Integration**
- Development server
- Browser integration (WASM)
- CLI flags

### Example Output

```typescript
// koa watch --hot
// Watching for changes...
// File changed: main.koa
// Recompiling...
// Hot reloading... ✓
// State preserved
```

---

## Phase 9: Testing & Polish (4-6 weeks)

**Goal**: Production-ready

### Deliverables

- ✅ Comprehensive test suite
- ✅ Documentation
- ✅ Examples
- ✅ Performance optimization

### Tasks

**Week 1-2: Testing**
- Unit tests
- Integration tests
- Stdlib tests
- Stress tests

**Week 3-4: Documentation**
- Language spec
- API documentation
- Tutorials
- Examples

**Week 5-6: Polish**
- Performance optimization
- Bug fixes
- Code review
- Release preparation

---

## Technology Stack

### Compiler

- **Language**: Rust 2024
- **LLVM Binding**: `inkwell`
- **Parser**: `lalrpop` (atau handmade)
- **CLI**: `clap`
- **Testing**: Rust's built-in test framework

### Runtime

- **Memory**: Bump pointer allocator
- **GC**: Concurrent tri-color mark-sweep
- **Async**: Event loop dengan epoll/kqueue
- **I/O**: Non-blocking I/O

---

## Project Structure

```
koa/
├── crates/
│   ├── koa/                # Compiler library
│   │   ├── lexer/          # Tokenization
│   │   ├── parser/         # AST generation
│   │   ├── ast/            # AST definitions
│   │   ├── typeck/         # Type checker
│   │   ├── ir/             # Intermediate representation
│   │   └── llvm_gen/       # LLVM IR generation
│   ├── koa-cli/            # CLI tool (binary: "koa")
│   │   ├── build.rs
│   │   ├── run.rs
│   │   ├── test.rs
│   │   └── fetch.rs        # Package manager
│   └── koa-runtime/        # Runtime library
│       ├── gc/             # Concurrent mark-sweep GC
│       ├── async/          # Async runtime
│       ├── alloc/          # Memory allocator
│       └── stdlib/         # Standard library
├── std/                    # Stdlib source (Koa code)
├── tests/
│   ├── unit/
│   └── integration/
├── examples/
├── build/                  # Build output directory (gitignored)
│   ├── debug/
│   │   ├── x86_64/
│   │   ├── aarch64/
│   │   └── wasm32/
│   └── release/
│       ├── x86_64/
│       ├── aarch64/
│       └── wasm32/
├── docs/
├── Cargo.toml
└── Makefile
```

### Build Output Structure

The Koa compiler produces explicit, architecture-aware build outputs:

```
build/
├── debug/                      # Debug builds (unoptimized, with debug symbols)
│   ├── x86_64/                 # x86_64 (amd64) binaries
│   │   ├── hello               # Compiled binary
│   │   └── *.o                 # Object files
│   ├── aarch64/                # ARM64 binaries
│   │   ├── myapp
│   │   └── *.o
│   └── wasm32/                 # WebAssembly binaries
│       ├── app.wasm
│       └── *.o
└── release/                    # Release builds (optimized)
    ├── x86_64/
    │   ├── server              # Stripped, optimized binary
    │   └── *.o
    ├── aarch64/
    │   ├── myapp
    │   └── *.o
    └── wasm32/
        ├── app.wasm
        └── *.o
```

**Build Mode Examples:**

```bash
# Debug build for current architecture
koa build main.koa                    # → build/debug/x86_64/main

# Release build for current architecture
koa build --release main.koa          # → build/release/x86_64/main

# Cross-compile for ARM64
koa build --target aarch64 main.koa   # → build/debug/aarch64/main

# WebAssembly
koa build --target wasm32 main.koa    # → build/debug/wasm32/main.wasm
```

**Architecture Detection:**

The compiler automatically detects the target architecture:
- `x86_64` - AMD64/Intel 64-bit
- `aarch64` - ARM64 (Apple Silicon, ARM servers)
- `wasm32` - WebAssembly
- `riscv64` - RISC-V 64-bit (future)

---

## Milestones

| Milestone           | Duration   | Deliverable       |
|---------------------|------------|-------------------|
| **M1: Hello World** | Week 1-6   | Basic executable  |
| **M2: Generics**    | Week 7-14  | Full generics     |
| **M3: Patterns**    | Week 15-20 | Pattern matching  |
| **M4: GC**          | Week 21-30 | Working GC        |
| **M5: Async**       | Week 31-40 | Async runtime     |
| **M6: Stdlib**      | Week 41-48 | Standard library  |
| **M7: HMR**         | Week 49-56 | Hot module reload |
| **M8: v0.1.0**      | Week 57-62 | First release     |

---

## Success Criteria

### Phase 1-3

- ✅ Can compile and run Hello World
- ✅ Can define and use structs
- ✅ Can use generics
- ✅ Can do pattern matching

### Phase 4-5

- ✅ GC runs without crashes
- ✅ No memory leaks (detected by tests)
- ✅ Async I/O works
- ✅ Event loop handles multiple tasks

### Phase 6-7

- ✅ Module system works
- ✅ Stdlib has essential modules
- ✅ Error handling works
- ✅ Package manager fetches dependencies

### Phase 8

- ✅ Comprehensive test suite passes
- ✅ Documentation complete
- ✅ Examples work
- ✅ Performance acceptable

---

## Next Steps

1. **Start Phase 1**: Setup Rust project structure
2. **Implement Lexer**: Tokenization
3. **Implement Parser**: AST generation
4. **Bootstrap**: Hello World

---

## Timeline Visualization

```
Phase 1: ████░░░░░░░░░░░░░░░░ (4-6 weeks)
Phase 2: ░░░░████░░░░░░░░░░░░ (6-8 weeks)
Phase 3: ░░░░░░░░░███░░░░░░░░░ (4-6 weeks)
Phase 4: ░░░░░░░░░░░░░░████░░░░ (8-10 weeks)
Phase 5: ░░░░░░░░░░░░░░░░░░░███ (8-10 weeks)
Phase 6: ░░░░░░░░░░░░░░░░░░░░░░██ (6-8 weeks)
Phase 7: ░░░░░░░░░░░░░░░░░░░░░░░░█ (4-6 weeks)
Phase 8: ░░░░░░░░░░░░░░░░░░░░░░░░░███ (6-8 weeks)
Phase 9: ░░░░░░░░░░░░░░░░░░░░░░░░░░░░█ (4-6 weeks)

Total: ~46-62 weeks
```

---

Let's start building! 🚀
