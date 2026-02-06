# Introduction to Koa

## What is Koa?

**Koa** is a modern programming language designed for:

1. **TypeScript Developers** - Familiar syntax that's easy to learn
2. **Systems Programming** - Performance close to C/Rust with LLVM backend
3. **Memory Safety** - Automatic garbage collector, no manual memory management
4. **Simplicity** - Without complexity like trait system (Rust) or modules (Go)

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
        Self { x, y }
    }
}

// Generics (seperti TypeScript)
fn identity<T>(x: T): T {
    x
}

// Async/await (seperti TypeScript)
async fn fetch(): !Data {
    await http_get(url)
}
```

### 2. No Need for Memory Management

Koa has **Concurrent Mark-Sweep Garbage Collector** (like Go):

- **Tri-color marking** - Efficient and low-latency
- **Concurrent** - GC runs parallel with application
- **No manual free** - No need to deallocate memory
- **Safe** - No dangling pointers, use-after-free, etc

```typescript
// No need to free! GC handles it.
fn create_nodes(count: i32): Vec<Node> {
    let nodes: Vec<Node> = Vec::new()
    for i in 0..count {
        try nodes.push(Node::new(i))
    }
    nodes  // GC will cleanup this when not used
}
```

### 3. Compiled Language

Koa is compiled to native code via **LLVM**:

- **Fast execution** - Native performance
- **Optimizations** - LLVM's mature optimization passes
- **Multi-platform** - x86, ARM, WebAssembly, etc

```bash
# Compile ke native executable
koa build --mode release-fast
./myprogram  # Native speed!
```

### 4. Simple but Powerful

Koa rejects unnecessary complexity:

- ❌ **No trait system** (Rust) - Too complex
- ❌ **No complex module system** (Go) - Rigid workspace
- ✅ **Error sets** (Zig) - Simple and explicit
- ✅ **Methods in struct body** (Zig) - Natural, no impl blocks
- ✅ **const/let variables** - Clear intent

## Why Not Other Languages?

| Language       | Koa Advantage                                                    |
|----------------|------------------------------------------------------------------|
| **Rust**       | Simpler (no traits, borrow checker)                              |
| **Go**         | Better syntax, generics in Phase 1, explicit error handling      |
| **Zig**        | More familiar for TS developers, async/await                     |
| **TypeScript** | Compiled, faster, runtime type-safe                              |
| **C/C++**      | Memory safe, modern syntax, GC                                   |

## Use Cases

Koa is suitable for:

- **Systems programming** - OS, embedded, databases
- **Web backends** - Fast servers with low latency
- **CLI tools** - Performance-critical command-line apps
- **Game development** - Performance with productivity balance
- **Desktop applications** - Native GUI apps

## What Koa is NOT

Koa is **not** for:

- ❌ Browser/JavaScript environments (use TypeScript)
- ❌ Extreme low-level systems programming (use Rust/C)
- ❌ Quick scripting (use Python/JavaScript)
- ❌ Embedded systems with < 1MB RAM (use C)

## Language Comparison

### Variables

```typescript
// TypeScript
const x: number = 42
let y: string = "hello"

// Koa
const x: i32 = 42
let y: string = "hello"
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
        Self { x, y }
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
    something()
} catch (err) {
    handleError(err)
}

// Koa
fn something(): !void {
    // ...
}

fn main(): i32 {
    match something() {
        Ok(()) => 0,
        Err(err) => {
            println!("Error: {}", err)
            1
        },
    }
}
```

### Async

```typescript
// TypeScript
async function fetch(): Promise<Data> {
    const response = await httpGet(url)
    return response.data
}

// Koa
async fn fetch(): !Data {
    let response: HttpResponse = await http_get(url)
    response.data
}
```

## Next Steps

- [Syntax Guide](02-syntax-guide.md) - See complete syntax reference
- [Type System](03-type-system.md) - Learn type system
- [Implementation Plan](10-implementation-plan.md) - See roadmap

## Community

- GitHub: [https://github.com/riipandi/koa](https://github.com/riipandi/koa)
- Documentation: [https://koa-lang.com](https://koa-lang.com)
- Discord: [Join the server](https://discord.gg/koa)

## License

Koa is dual-licensed under either:

- **MIT License** (LICENSE-MIT or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

You may choose to license this code under either license at your option.
