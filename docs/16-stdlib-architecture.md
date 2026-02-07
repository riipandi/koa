# Koa Standard Library Architecture

## Overview

The Koa standard library is split into two parts:

1. **`library/std/`** - Standard library source code written in Koa
2. **`crates/koa-runtime/`** - Runtime implementation written in Rust (using Rust std)

This separation is common in compiled languages like Rust, Go, and C.

## 🎯 Important Clarification: C ABI vs C Functions

**Common misconception:** "We use C functions in the runtime"

**Reality:** We use **Rust's standard library internally**, expose via **C ABI** for FFI.

### Why C ABI?

```
┌─────────┐      ┌─────────┐      ┌──────────┐      ┌─────────────┐
│ Koa Code │ ───> │ LLVM IR │ ───> │ Machine  │ ───> │   Rust Func  │
│         │      │         │      │  Code    │      │ (using std::io)
└─────────┘      └─────────┘      └──────────┘      └─────────────┘
                                                          │
                                                    Must use C ABI
                                                    (standard FFI boundary)
```

**Why C ABI?**
- ✅ LLVM knows how to call C ABI functions
- ✅ Standard for cross-language interoperability
- ✅ NOT about using C functions
- ✅ About calling convention standard

### What We Actually Do

```rust
// io.rs - Runtime implementation in RUST
#[unsafe(no_mangle)]
pub extern "C" fn koa_println(s: *const c_char) {
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(s);
        // We use RUST'S println! here, NOT C's puts
        println!("{}", c_str.to_str().unwrap());
        //  ^^^^^^ ^^^ This is Rust standard library!
    }
}
```

**Key Points:**
1. `extern "C"` = Uses C calling convention (ABI)
2. `println!()` = Uses Rust's standard library
3. The C ABI is ONLY for the FFI boundary

## Directory Structure

```
koa/
├── library/
│   └── std/
│       ├── mod.koa         # Root module, re-exports everything
│       ├── io.koa          # I/O operations (extern declarations)
│       ├── string.koa       # String manipulation
│       ├── convert.koa     # Type conversions
│       └── math.koa        # Mathematical functions
│
└── crates/
    └── koa-runtime/
        ├── Cargo.toml
        └── src/
            ├── lib.rs      # Runtime entry point
            ├── io.rs       # I/O using Rust std::io with C ABI exports
            ├── alloc/
            │   └── mod.rs  # Memory allocators
            ├── gc/
            │   └── mod.rs  # Garbage collector
            └── async_.rs   # Async runtime
```

## How It Works

### 1. Koa Source Files (`library/std/*.koa`)

These files contain:
- **Extern declarations** for runtime functions (with C ABI)
- **Pure Koa implementations** for helper functions
- **Type definitions** and constants

Example:
```koa
// io.koa
extern fn koa_println(s: string): void;

pub fn println(s: string): void {
    koa_println(s);  // Calls into Rust runtime via C ABI
    return;
}
```

### 2. Runtime Implementation (`crates/koa-runtime/src/*.rs`)

These files contain:
- **Rust implementations** using Rust std::io, std::alloc, etc.
- **C ABI exports** (`extern "C"`) for FFI compatibility
- **System-level operations** using Rust's standard library

Example:
```rust
// io.rs
#[unsafe(no_mangle)]
pub extern "C" fn koa_println(s: *const c_char) {
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(s);
        println!("{}", c_str.to_str().unwrap());  // Rust std::io!
    }
}
```

### 3. The Complete Flow

```
┌────────────────────────────────────────────────────────────────────┐
│ 1. Koa Code                                                       │
│    println("Hello")                                              │
└────────────────────────────────────────────────────────────────────┘
                                ↓
┌────────────────────────────────────────────────────────────────────┐
│ 2. Koa Stdlib (io.koa)                                            │
│    extern fn koa_println(s: string): void;                       │
│    koa_println("Hello");                                          │
└────────────────────────────────────────────────────────────────────┘
                                ↓
┌────────────────────────────────────────────────────────────────────┐
│ 3. LLVM IR                                                       │
│    declare extern "C" @koa_println(i8*)                         │
│    call @koa_println                                              │
└────────────────────────────────────────────────────────────────────┘
                                ↓
┌────────────────────────────────────────────────────────────────────┐
│ 4. Machine Code (x86-64, ARM64, etc.)                            │
│    CALL koa_println  ; Uses C calling convention                  │
└────────────────────────────────────────────────────────────────────┘
                                ↓
┌────────────────────────────────────────────────────────────────────┐
│ 5. Rust Runtime (io.rs)                                           │
│    #[unsafe(no_mangle)]                                           │
│    pub extern "C" fn koa_println(...) {                           │
│        println!("Hello");  // ← Uses Rust's std::io!               │
│    }                                                             │
└────────────────────────────────────────────────────────────────────┘
```

## Standard Library Modules

### Core Modules

| Module | File | Description |
|--------|------|-------------|
| I/O | `io.koa` | Input/output operations (extern declarations) |
| String | `string.koa` | String manipulation functions |
| Convert | `convert.koa` | Type conversion utilities |
| Math | `math.koa` | Mathematical functions and constants |

### Runtime Modules (Rust Implementation)

| Module | File | Implementation |
|--------|------|----------------|
| I/O | `io.rs` | Uses Rust `std::io::{print, println}`, `std::io::stdin` |
| Allocator | `alloc/mod.rs` | Will use Rust `std::alloc` |
| GC | `gc/mod.rs` | Custom implementation |
| Async | `async_.rs` | Will use Rust `tokio` |

## Adding New Functions

### Step 1: Add Koa Declaration

Create or update a file in `library/std/`:

```koa
// library/std/mymodule.koa

// Declare extern function (will be provided by Rust runtime)
extern fn koa_my_function(x: i32): i32;

// Wrapper for convenience
pub fn my_wrapper(x: i32): i32 {
    return koa_my_function(x);
}
```

### Step 2: Add Runtime Implementation (in Rust)

Create or update a file in `crates/koa-runtime/src/`:

```rust
// crates/koa-runtime/src/mymodule.rs

use std::ffi::c_int;

#[unsafe(no_mangle)]
pub extern "C" fn koa_my_function(x: c_int) -> c_int {
    // Actual implementation using RUST standard library
    x.wrapping_mul(2)
}
```

### Step 3: Export from lib.rs

Update `crates/koa-runtime/src/lib.rs`:

```rust
pub mod mymodule;
```

### Step 4: Use in Koa Code

```koa
import from "std/mymodule";

let result: i32 = mymodule::my_wrapper(10);
```

## Why This Architecture?

### Benefits

1. **Rust Standard Library** - We get battle-tested I/O, allocators, etc.
2. **Type Safety** - Rust's type system protects runtime code
3. **Performance** - Rust compiles to efficient machine code
4. **Memory Safety** - Rust prevents common C bugs (buffer overflows, etc.)
5. **C ABI** - Standard FFI boundary that LLVM understands

### Comparison with Other Languages

| Language | Stdlib Language | FFI Boundary |
|----------|-----------------|--------------|
| **Koa** | Rust (ours) | C ABI |
| Rust | Rust | C ABI (for extern functions) |
| Go | Go | C ABI (for cgo) |
| C# | C# | P/Invoke (C ABI) |
| Python | C | C API (Python C API) |

## Current Limitations

### Not Yet Implemented

1. **String Functions** - String operations are stubs (TODO: use Rust `std::str`)
2. **Math Functions** - Math operations are stubs (TODO: use Rust `libm`)
3. **Format Strings** - Printf formatting is basic (TODO: proper parsing)
4. **Error Handling** - No proper error handling (TODO: use Rust `Result`)

### TODO

- [ ] Implement proper string operations using Rust `std::str`
- [ ] Implement math functions using Rust `libm` bindings
- [ ] Add format string parsing in printf
- [ ] Implement error types and propagation using Rust `Result`
- [ ] Add file I/O operations using Rust `std::fs`
- [ ] Add collections using Rust `std::collections`
- [ ] Add concurrency primitives using Rust `std::thread`
- [ ] Add networking support using Rust `std::net`

## Linking

When compiling a Koa program, the CLI:

1. Generates LLVM IR with extern function declarations (C ABI)
2. Compiles LLVM IR to native object file
3. Links object file with `libkoa_runtime.a`
4. Produces final executable

## Best Practices

### For Koa Code

- Use `pub` keyword for exported functions
- Provide wrapper functions for better ergonomics
- Document all public functions
- Keep functions simple and focused

### For Runtime Code

- Use `#[unsafe(no_mangle)]` for exported functions
- Use `extern "C"` for C ABI compatibility (FFI boundary only)
- Prefer Rust's standard library over C functions
- Handle null pointers gracefully
- Return proper error codes
- Document unsafe operations

## Examples

### Simple I/O

```koa
import from "std/io";

fn main(): i32 {
    io.println("Hello, World!");
    let name: string = io.readline();
    io.printf("Hello, %s\n", name);
    return 0;
}
```

**Behind the scenes:**
1. `io.println()` → `koa_println()` (Koa)
2. `koa_println()` → Rust function with C ABI
3. Rust implementation: `println!(...)` using Rust std::io

### Math Operations

```koa
import from "std/math";

fn main(): i32 {
    let x: f64 = 5.0;
    let result: f64 = math.sqrt(x);
    io.printf("sqrt(%.2f) = %.2f\n", x, result);
    return 0;
}
```

**Future implementation:**
```rust
// math.rs (runtime)
#[unsafe(no_mangle)]
pub extern "C" fn koa_sqrt(x: f64) -> f64 {
    x.sqrt()  // Uses Rust's libm binding
}
```

### String Manipulation

```koa
import from "std/string";

fn main(): i32 {
    let s: string = "Hello";
    let upper: string = string.to_upper(s);
    io.println(upper);
    return 0;
}
```

**Future implementation:**
```rust
// string.rs (runtime)
#[unsafe(no_mangle)]
pub extern "C" fn koa_to_upper(s: *const c_char) -> *mut c_char {
    // Use Rust std::str::to_uppercase()
}
```

## Further Reading

- [Koa Syntax Guide](../docs/02-syntax-guide.md)
- [Module System](../docs/07-modules.md)
- [Implementation Plan](../docs/10-implementation-plan.md)
- [Runtime Library](../crates/koa-runtime/)
