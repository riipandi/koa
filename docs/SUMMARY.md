# Koa Programming Language

**Koa** is a general-purpose compiled programming language designed for building efficient, reliable, and scalable software with a focus on simplicity and productivity.

## Goals

1. **Familiar with TypeScript** - Low learning curve for JavaScript/TypeScript developers
2. **No manual memory management** - Automatic via concurrent mark-sweep GC
3. **Compiled** - Native code performance via LLVM
4. **Simple but Powerful** - Minimal complexity, maximum expressiveness

## Design Philosophy

- **Explicit > Implicit** - Everything must be explicit, no magic
- **Const > Let** - Immutable by default
- **No Shadowing** - No shadowing variables allowed
- **Errors as Values** - Error unions, not exceptions
- **Safety First** - Runtime checks by default
- **No Null** - Explicit nullable types
- **Too Lazy to be Complex** - Reject unnecessary complexity

## Quick Links

### Core Documentation
- [Introduction](01-introduction.md) - What is Koa and why?
- [Syntax Guide](02-syntax-guide.md) - Complete syntax reference
- [Type System](03-type-system.md) - Type system and generics
- [Error Handling](04-error-handling.md) - Error sets and error unions
- [Memory Management](05-memory-management.md) - Concurrent mark-sweep GC
- [Concurrency](06-concurrency.md) - Async/await model

### Advanced Topics
- [Module System](07-modules.md) - Rust-style module resolution
- [Conditional Compilation](08-conditional-compilation.md) - Build modes and annotations
- [Standard Library](09-standard-library.md) - Stdlib architecture
- [Cross Compilation](15-cross-compilation.md) - Multi-platform support

### Tooling & Build System
- [Build System](16-build-system.md) - Koa.toml specification
- [Lockfile Format](17-lockfile-spec.md) - Koa.lock JSON schema
- [Package Manager](11-package-manager.md) - Dependency management
- [Build Cache](18-build-cache.md) - Incremental compilation
- [Project Initialization](19-project-init.md) - koa init command

### Architecture & Decisions
- [Architecture Decisions](20-architecture-decisions.md) - ADRs (Architecture Decision Records)
- [FFI](13-ffi.md) - C interop and foreign function interface
- [Implementation Plan](10-implementation-plan.md) - Roadmap and milestones

## Example

```typescript
// Struct with methods in body
pub struct Point {
    x: f64,
    y: f64,

    pub fn new(x: f64, y: f64): Self {
        return Self { x, y };
    }

    pub fn distance(self, other: Point): f64 {
        let dx: f64 = self.x - other.x;
        let dy: f64 = self.y - other.y;
        return (dx * dx + dy * dy).sqrt();
    }
}

// Error handling with error sets
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

// Async/await with explicit error handling
pub async fn fetch_data(url: string): !Data {
    let response: HttpResponse = await http_get(url);
    return response.data;
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

// Required return keyword
fn identity<T>(x: T): T {
    return x;
}

// Usage
pub fn main(): i32 {
    const p1: Point = Point::new(0.0, 0.0);
    const p2: Point = Point::new(3.0, 4.0);
    let dist: f64 = p1.distance(p2);

    [@debug]
    log_debug("Distance calculated");

    println!("Distance: {}", dist);

    return 0;
}
```

## Key Features

- TypeScript-familiar syntax with snake_case naming
- Concurrent mark-sweep garbage collector with programmatic control
- Error sets and error unions (no exceptions)
- Generics support
- Methods defined in struct body
- const/let variables (immutable by default)
- Async/await with Promise-based runtime
- Required return keyword (explicit control flow)
- Rust-style documentation comments (`///`)
- No relative paths in module imports
- LLVM backend with cross-compilation support

## Implementation Status

### Completed (Phase 0)
- [x] Language specification
- [x] Project structure setup
- [x] Architecture decisions
- [x] Documentation planning

### In Progress
- [ ] Milestone 1: Lexer Implementation
- [ ] Milestone 2: Parser Implementation
- [ ] Milestone 3: AST definitions

### Planned
- [ ] Milestone 4: If/else expressions
- [ ] Milestone 5: While loops
- [ ] Milestone 6: Functions with imports
- [ ] Milestone 7: Structs
- [ ] Milestone 8: Arrays
- [ ] Milestone 9: Strings
- [ ] Milestone 10: Error handling
- [ ] Milestone 11: Standard library
- [ ] Milestone 12: Garbage collector

## License

Koa is dual-licensed under either:

- **MIT License** (LICENSE-MIT or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

You may choose to license this code under either license at your option.
