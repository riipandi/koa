# Koa Programming Language

**Koa** is a modern compiled programming language with garbage collector, static typing, and syntax familiar to TypeScript developers.

## Goals

1. **Familiar with TypeScript** - Low learning curve for JavaScript/TypeScript developers
2. **No manual memory management** - Automatic via concurrent mark-sweep GC
3. **Compiled** - Native code performance via LLVM
4. **Simple but Powerful** - Minimal complexity, maximum expressiveness

## Design Philosophy

- **Explicit > Implicit** - Everything must be explicit, no magic
- **Const > Var** - Immutable by default
- **No Shadowing** - No shadowing variables allowed
- **Errors as Values** - Error unions, not exceptions
- **Safety First** - Runtime checks by default
- **No Null** - Explicit nullable types

## Quick Links

- [Introduction](01-introduction.md) - What is Koa and why?
- [Syntax Guide](02-syntax-guide.md) - Complete syntax reference
- [Type System](03-type-system.md) - Type system and generics
- [Error Handling](04-error-handling.md) - Error sets and error unions
- [Memory Management](05-memory-management.md) - Concurrent mark-sweep GC
- [Concurrency](06-concurrency.md) - Async/await model
- [Module System](07-modules.md) - Rust + TypeScript hybrid modules
- [Conditional Compilation](08-conditional-compilation.md) - Build modes and annotations
- [Standard Library](09-standard-library.md) - Standard library plan
- [Implementation Plan](10-implementation-plan.md) - Roadmap
- [Package Manager](11-package-manager.md) - Built-in package manager
- [Database Drivers](12-database-drivers.md) - SQLite & PostgreSQL drivers
- [FFI](13-ffi.md) - C interop and foreign function interface
- [HMR](14-hmr.md) - Hot Module Reload for fast development

## Example

```typescript
// Struct with methods in body
pub struct Point {
    x: f64,
    y: f64,

    pub fn new(x: f64, y: f64): Self {
        Self { x, y };
    }

    pub fn distance(self, other: Point): f64 {
        let dx: f64 = self.x - other.x;
        let dy: f64 = self.y - other.y;
        (dx * dx + dy * dy).sqrt();
    }
}

// Error handling
const FileError = error {
    NotFound,
    AccessDenied,
}

fn read_file(path: string): FileError!string {
    if !exists(path) {
        return error.NotFound;
    };
    // ...
}

// Async/await
pub async fn fetch_data(url: string): !Data {
    let response: HttpResponse = await http_get(url);
    response.data;
}

// Conditional compilation
[@debug]
fn log_debug(msg: string): void {
    println!("DEBUG: {}", msg);
}

[@not_debug]
fn log_debug(msg: string): void {
// No-op in release
    }

// Generics
fn identity<T>(x: T): T {
    x;
}

// Usage
pub fn main(): i32 {
    const p1: Point = Point::new(0.0, 0.0);
    const p2: Point = Point::new(3.0, 4.0);
    let dist: f64 = p1.distance(p2);

    [@debug]
    log_debug("Distance calculated");

    println!("Distance: {}", dist);

    0;
}
```

## Key Features

- TypeScript-familiar syntax
- Concurrent mark-sweep garbage collector
- Error sets and error unions
- Generics support
- Methods defined in struct body
- const/let variables
- Async/await
- Simple annotations
- LLVM backend

## Implementation Status

- [x] Language specification
- [x] Project structure setup
- [ ] Phase 1: Lexer & Parser
- [ ] Phase 2: Type Checker
- [ ] Phase 3: Code Generation
- [ ] Phase 4: Runtime & GC
- [ ] Phase 5: Concurrency
- [ ] Phase 6: Modules & Stdlib
- [ ] Phase 7: Error Handling & Tooling
- [ ] Phase 8: HMR (Hot Module Reload)
- [ ] Phase 9: Testing & Polish

## License

Koa is dual-licensed under either:

- **MIT License** (LICENSE-MIT or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

You may choose to license this code under either license at your option.
