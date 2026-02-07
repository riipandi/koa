# Koa Runtime Library

The runtime library provides low-level support for Koa programs, including I/O operations, memory management, and system interactions.

## Important: We Use Rust, Not C!

**Common misconception:** "C FFI means we're using C functions"

**Reality:** We use **Rust's standard library**, but expose it with **C ABI** for FFI compatibility.

### Why C ABI?

```
Koa Code → LLVM IR → needs to call → Rust Functions
                           ↓
                    must use C ABI (standard FFI boundary)
```

**C ABI** is the lingua franca for system-level interoperability:
- LLVM IR knows how to call C ABI functions
- It's NOT about calling C functions
- It's about having a standard calling convention

### What We Actually Do

```rust
// We use Rust's println! INTERNALLY
#[unsafe(no_mangle)]
pub extern "C" fn koa_println(s: *const c_char) {
    // Convert C string from Koa
    let c_str = std::ffi::CStr::from_ptr(s);

    // Use RUST'S println! macro
    println!("{}", c_str.to_str().unwrap());
    //  ^^^^^^ This is Rust's standard library!
}
```

## Structure

```
src/
├── lib.rs             # Main entry point, exports all modules
├── io.rs              # I/O operations (Rust implementation with C ABI exports)
├── alloc/             # Memory allocators (stub)
├── gc/                # Garbage collector (stub)
└── async_runtime.rs   # Async runtime (stub)
```

## Building

```bash
# Build only the runtime library
cargo build -p koa-runtime

# Build in release mode
cargo build -p koa-runtime --release
```

## Using the Runtime

The runtime is automatically linked when compiling Koa programs. You don't need to manually link it.

### Exported Functions (with C ABI for FFI)

#### I/O Functions (using Rust's std::io)

- `koa_print(s: *const c_char)` - Uses Rust's `print!()` internally
- `koa_println(s: *const c_char)` - Uses Rust's `println!()` internally
- `koa_printf(format: *const c_char)` - Stub for formatted output
- `koa_strlen(s: *const c_char) -> i32` - Get string length
- `koa_readline() -> *mut c_char` - Uses Rust's `std::io::stdin()`
- `koa_free_string(s: *mut c_char)` - Free allocated string

#### Legacy Functions (for C compatibility)

- `puts(s: *const c_char) -> i32` - Wraps `koa_println()`
- `printf(format: *const c_char) -> i32` - Wraps `koa_printf()`

#### Runtime Lifecycle

- `koa_init()` - Initialize runtime (called automatically)
- `koa_cleanup()` - Cleanup runtime (called automatically)

## Memory Safety

All exported functions:
- Check for null pointers before dereferencing
- Use `unsafe` blocks only where necessary (FFI boundary)
- Handle UTF-8 conversion errors gracefully
- Use Rust's standard library for actual I/O

## Examples

### From Koa Side

```koa
// io.koa - declares extern function with C ABI
extern fn koa_println(s: string): void;

pub fn println(s: string): void {
    koa_println(s);  // calls into Rust runtime
    return;
}
```

### From Rust Side

```rust
// io.rs - implements function using Rust std::io
#[unsafe(no_mangle)]
pub extern "C" fn koa_println(s: *const c_char) {
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(s);
        // Using RUST'S println! macro
        println!("{}", c_str.to_str().unwrap());
        //  ^^^^^^ Standard Rust library!
    }
}
```

### The Flow

```
1. Koa code:          println("Hello")
2. Koa stdlib:        koa_println("Hello")
3. LLVM IR:           call @koa_println
4. Machine code:       CALL koa_println (C ABI)
5. Rust runtime:       println("Hello") (Rust std::io!)
                      ^^^^^^ Uses Rust's standard library
```

## TODO

- [ ] Implement proper format string parsing in `printf`
- [ ] Add file I/O operations using Rust's `std::fs`
- [ ] Implement memory allocators using Rust's `std::alloc`
- [ ] Implement garbage collector
- [ ] Add async runtime support using Rust's `tokio`
- [ ] Add error handling using Rust's `Result` type
- [ ] Add networking support using Rust's `std::net`

## License

MIT License - See LICENSE file for details
