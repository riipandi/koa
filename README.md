# Koa

**Koa** is a modern compiled programming language designed for developer productivity without sacrificing performance.

> **⚠️ Experimental & Educational Project**
> 
> This is an **experimental** compiler project created for **educational purposes**. It is **not** intended for production use. The implementation is incomplete, may contain bugs, and the language specification is subject to change. Use at your own risk!
> 
> **Learning Goals:**
> - Understanding compiler design and implementation
> - Exploring LLVM integration
> - Studying garbage collection algorithms
> - Learning programming language theory

## Features

- **TypeScript-Familiar Syntax** - Low learning curve for JavaScript/TypeScript developers
- **Type-Safe** - Static typing with explicit type annotations
- **Memory Safe** - Concurrent mark-sweep garbage collector (no manual memory management)
- **Fast Performance** - Compiled to native code via LLVM
- **Simple Module System** - File-based modules with explicit imports/exports
- **Built-in Package Manager** - Git-based dependencies with lockfile support
- **Database Drivers** - Built-in SQLite and PostgreSQL drivers
- **FFI Support** - C-compatible foreign function interface
- **Hot Module Reload** - Fast development iteration

## Installation

### From Source

```bash
# Clone repository
git clone https://github.com/riipandi/koa.git
cd koa

# Build
cargo build --release

# Add to PATH
export PATH="$PATH:$PWD/target/release:$PATH"
```

### Requirements

- Rust 1.70+ (for building)
- LLVM 17+
- C compiler (for building runtime)

## Quick Start

### Hello World

Create `main.koa`:

```koa
pub fn main(): i32 {
    println!("Hello, World!");
    0;
}
```

Compile and run:

```bash
koa build main.koa
./main
```

### Using the CLI

```bash
# Build with debug mode (outputs to build/debug/<arch>/main)
koa build main.koa

# Build with release mode (outputs to build/release/<arch>/main)
koa build --release main.koa

# Specify output path
koa build main.koa --output build/debug/x86_64/myapp

# Cross-compile for ARM64
koa build --target aarch64 main.koa

# Run directly
koa run main.koa

# Watch for changes with HMR
koa watch
```

#### Build Output Structure

```
build/
├── debug/
│   ├── x86_64/        # AMD64/Intel 64-bit binaries
│   ├── aarch64/       # ARM64 binaries (Apple Silicon)
│   └── wasm32/        # WebAssembly
└── release/
    ├── x86_64/        # Optimized binaries
    ├── aarch64/
    └── wasm32/
```

The compiler automatically detects your system architecture and places build outputs in the appropriate directory.

## Documentation

- [Introduction](docs/01-introduction.md)
- [Syntax Guide](docs/02-syntax-guide.md)
- [Type System](docs/03-type-system.md)
- [Error Handling](docs/04-error-handling.md)
- [Memory Management](docs/05-memory-management.md)
concurrency
- [Module System](docs/07-modules.md)
- [Conditional Compilation](docs/08-conditional-compilation.md)
- [Standard Library](docs/09-standard-library.md)
- [Implementation Plan](docs/10-implementation-plan.md)
- [Package Manager](docs/11-package-manager.md)
- [Database Drivers](docs/12-database-drivers.md)
- [FFI](docs/13-ffi.md)
- [HMR](docs/14-hmr.md)

## Examples

See the `examples/` directory for more examples.

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https://sh.rustup.rs' | sh

# Install LLVM
# macOS
brew install llvm@17

# Ubuntu/Debian
sudo apt-get install llvm-17-dev libssl-17-dev
```

### Build Commands

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with optimizations
cargo build --release
```

### Watch Mode (HMR)

```bash
# Watch for file changes and auto-reload
cargo watch
```

## Language Showcase

### Struct with Methods

```koa
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
```

### Error Handling

```koa
const FileError = error {
    NotFound,
    AccessDenied,
};

fn read_file(path: string): FileError!string {
    if !exists(path) {
        return error.NotFound;
    };
    // ...
}
```

### Async/Await

```koa
async fn fetch_data(url: string): !Data {
    let response: HttpResponse = await http_get(url);
    response.data;
}
```

### Conditional Compilation

```koa
[@debug]
fn log_debug(msg: string): void {
    println!("DEBUG: {}", msg);
}

[@release]
fn optimized_path(): void {
    // Release-only optimizations
}
```

## Package Management

```bash
# Add dependency
# Edit Koa.toml
koa fetch

# Update dependencies
koa update

# List dependencies
koa list
```

## Roadmap

See [Implementation Plan](docs/10-implementation-plan.md) for the complete roadmap.

Current status: Phase 1 (Core Foundation) - In Progress

## Contributing

We welcome contributions! Please see:
- [CONTRIBUTING.md](CONTRIBUTING.md) (coming soon)

## License

Koa is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose to license this code under either license at your option.

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for the full license text.

## Acknowledgments

Koa is inspired by:
- TypeScript (syntax)
- Rust (type system, LLVM backend)
- Zig (error handling, simplicity)
- Go (concurrent GC, goroutines)
- C (FFI design)

## Community

- Website: https://riipandi.github.io/koa
- Repository: https://github.com/riipandi/koa
- Issues: https://github.com/riipandi/koa/issues

---

**Koa** - Modern simplicity, compiled performance. 🌴
