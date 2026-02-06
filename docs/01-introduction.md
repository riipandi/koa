# Introduction to Koa

## What is Koa?

**Koa** is a general-purpose compiled programming language designed for building efficient, reliable, and scalable software with a focus on simplicity and productivity.

### Tagline

> **"Native performance, modern simplicity"**

### Philosophy

> **"Too lazy to be complex"**
>
> Inspired by the Koala, we believe engineers should be "lazy" about the tedious parts of systems programming:
>
> - **Too lazy to manage memory**: Why `malloc` and `free` when a concurrent GC can do the cleanup for you?
> - **Too lazy to optimize manually**: Why hand-tune assembly when LLVM optimization passes exist?
> - **Too lazy to fight the compiler**: Why wrestle with a borrow checker when runtime safety is enough?
>
> Koa handles the heavy lifting so you can stay relaxed and productive.

## Design Goals

### 1. Familiar with TypeScript

Koa adopts syntax and concepts from TypeScript:

```typescript
// Variables (like TypeScript's const/let)
const x: i32 = 42        // Immutable
let y: f64 = 3.14        // Mutable

// Struct with methods (like TypeScript classes)
struct Point {
    x: f64,
    y: f64,

    pub fn new(x: f64, y: f64): Self {
        return Self { x, y };
    }
}

// Generics (like TypeScript)
fn identity<T>(x: T): T {
    return x;
}

// Async/await (like TypeScript)
async fn fetch(): !Data {
    let response: HttpResponse = await http_get(url);
    return response.data;
}
```

### 2. No Need for Memory Management

Koa has **Concurrent Mark-Sweep Garbage Collector** (Go-style):

- **Tri-color marking** - Efficient and low-latency
- **Concurrent** - GC runs parallel with application
- **No manual free** - No need to deallocate memory
- **Programmatic control** - Adjust GC behavior via API

```typescript
// No need to free! GC handles it.
fn create_nodes(count: i32): Vec<Node> {
    let nodes: Vec<Node> = Vec::new();
    for i in 0..count {
        try nodes.push(Node::new(i));
    }
    return nodes;  // GC will cleanup when not used
}
```

### 3. Compiled Language

Koa is compiled to native code via **LLVM**:

- **Fast execution** - Native performance
- **Optimizations** - LLVM's mature optimization passes
- **Multi-platform** - x86, ARM, WebAssembly, etc

```bash
# Compile to native executable
koa build --mode release-fast
./myprogram  # Native speed!
```

### 4. Simple but Powerful

Koa rejects unnecessary complexity:

- ❌ **No trait system** (Rust) - Too complex
- ❌ **No arrow functions** - Keep it simple
- ❌ **No OOP inheritance** - Composition over inheritance
- ✅ **Error sets** (Zig) - Simple and explicit
- ✅ **Methods in struct body** (Zig) - Natural, no impl blocks
- ✅ **Required return keyword** - Explicit control flow
- ✅ **snake_case naming** - Rust-style conventions

## Why Not Other Languages?

| Language       | Koa Advantage                                               |
|----------------|-------------------------------------------------------------|
| **Rust**       | Simpler (no traits, no borrow checker)                       |
| **Go**         | Better syntax, generics from start, explicit error handling |
| **Zig**        | More familiar for TS developers, async/await                |
| **TypeScript** | Compiled, faster, runtime type-safe                         |
| **C/C++**      | Memory safe, modern syntax, GC                              |

## Use Cases

Koa is suitable for:

- **Primary:** Backend systems, CLI tools, web services
- **Secondary:** Desktop applications, game development
- **Not suitable for:** Browser environments (use TypeScript), kernel development (use Rust), embedded <1MB RAM (use C)

## What Koa is NOT

Koa is **not** for:

- ❌ Browser/JavaScript environments (use TypeScript)
- ❌ Extreme low-level systems programming (use Rust/C/Zig)
- ❌ Quick scripting (use Python/JavaScript)
- ❌ Embedded systems with < 1MB RAM (use C)

## Language Comparison

### Variables

```typescript
// TypeScript
const x: number = 42;
let y: string = "hello";

// Koa
const x: i32 = 42;
let y: string = "hello";
```

### Structs & Methods

```typescript
// TypeScript
class Point {
    constructor(public x: number, public y: number) {}

    distance(other: Point): number {
        // ...
    }
}

// Koa
struct Point {
    x: f64,
    y: f64,

    pub fn new(x: f64, y: f64): Self {
        return Self { x, y };
    }

    pub fn distance(self, other: Point): f64 {
        // ...
    }
}
```

### Error Handling

```typescript
// TypeScript
try {
    something();
} catch (err) {
    handleError(err);
}

// Koa
fn something(): !void {
    // ...
}

fn main(): i32 {
    match something() {
        Ok(()) => return 0,
        Err(err) => {
            println!("Error: {}", err);
            return 1;
        },
    }
}
```

### Async

```typescript
// TypeScript
async function fetch(): Promise<Data> {
    const response = await httpGet(url);
    return response.data;
}

// Koa
async fn fetch(): !Data {
    let response: HttpResponse = await http_get(url);
    return response.data;
}
```

### Naming Convention

```typescript
// TypeScript (camelCase)
const userName = "Alice";
function calculateSum() { }

// Koa (snake_case)
const user_name = "Alice";
fn calculate_sum() { }
```

### Return Keyword

```typescript
// TypeScript (implicit return allowed)
const add = (x, y) => x + y;

// Koa (explicit return required)
fn add(x: i32, y: i32): i32 {
    return x + y;  // Required!
}
```

## Key Features

### Core Language
- TypeScript-familiar syntax with **snake_case** naming
- **Required return keyword** (no implicit returns)
- **Rust-style documentation comments** (`///`)
- **No relative paths** in module imports
- **const/let variables** (immutable by default)
- **No variable shadowing**

### Type System
- Static typing with type inference
- Generics support
- Error sets and error unions (Zig-style)
- No null, explicit nullable types
- enum with integer values (not string)

### Memory & Concurrency
- Concurrent mark-sweep GC (Go-style)
- Programmatic GC control via `gc` module
- Async/await with Promise-based runtime
- Multi-threaded executor

### Tooling
- Built-in package manager
- JSON lockfile for reproducible builds
- Incremental compilation cache
- Cross-compilation support

## Project Status

**Current Version:** 0.1.0 (Experimental)

**Maturity:** Early implementation phase (~15-20% complete)

**Stability:** Not production-ready, API may change

## Next Steps

- [Syntax Guide](02-syntax-guide.md) - See complete syntax reference
- [Type System](03-type-system.md) - Learn type system
- [Implementation Plan](10-implementation-plan.md) - See roadmap
- [Architecture Decisions](20-architecture-decisions.md) - Understand design choices

## Community

- GitHub: [https://github.com/riipandi/koa](https://github.com/riipandi/koa)
- Documentation: [https://koa-lang.com](https://koa-lang.com)
- Discord: [Join the server](https://discord.gg/koa)

## License

Koa is dual-licensed under either:

- **MIT License** (LICENSE-MIT or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

You may choose to license this code under either license at your option.
