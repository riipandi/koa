# Koa vs Other Languages: Runtime Architecture

## Comparison Table

| Language | Stdlib Implementation | FFI Boundary | Actual I/O Used |
|----------|---------------------|--------------|----------------|
| **Koa** | **Rust** | C ABI | **Rust std::io** ✅ |
| Rust | Rust | C ABI | Rust std::io |
| Go | Go | C ABI | Go fmt package |
| C# | C# | P/Invoke | .NET System.Console |
| Python | C | Python C API | C puts/fprintf |
| C++ | C++ | C ABI | C++ std::cout |

## Key Insight

**Koa is like Rust**: We use our own language's standard library (Rust), not C!

```text
┌─────────────────────────────────────────────────────────────────┐
│                    MISCONCEPTION                                │
└─────────────────────────────────────────────────────────────────┘

❌ "C ABI means calling C functions"
❌ "We use C's printf and puts"
❌ "Runtime is written in C"

┌─────────────────────────────────────────────────────────────────┐
│                      REALITY                                    │
└─────────────────────────────────────────────────────────────────┘

✅ "C ABI is just the calling convention (FFI boundary)"
✅ "We use Rust's println! and print! macros"
✅ "Runtime is written in Rust, using Rust std::io"
```

## Code Examples

### What People Think We Do

```text
Koa → extern "C" → C printf() ❌ WRONG!
```

### What We Actually Do

```text
Koa → extern "C" → Rust println!() ✅ CORRECT!
                  ↑
            C ABI boundary only
```

### Actual Code Flow

```rust
// This is what ACTUALLY happens in runtime:

#[unsafe(no_mangle)]
pub extern "C" fn koa_println(s: *const c_char) {
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(s);
        println!("{}", c_str.to_str().unwrap());
        //  ^^^^^^ ^^^^ ← RUST'S OWN STANDARD LIBRARY!
    }
}
```

**Breakdown:**
1. `extern "C"` = C calling convention (for FFI compatibility)
2. `println!()` = Rust's macro from `std::io`
3. We use Rust's standard library, NOT C functions

## Why C ABI Then?

### C ABI is the Universal Language

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Koa (LLVM)  │     │    Rust      │     │     Go       │
│              │     │              │     │              │
│  Uses C ABI  │ ←→ │  Uses C ABI  │ ←→ │  Uses C ABI  │
└──────────────┘     └──────────────┘     └──────────────┘
                           ↓
                  "Universal Language"
                   For FFI Interoperability
```

**Analogy:** English is the universal language of air traffic control,
not because all pilots are American, but because it's a standard everyone agrees on.

Similarly, C ABI is the standard for FFI, not because we use C functions,
but because it's a calling convention everyone (LLVM, Rust, Go, etc.) understands.

## Examples from Other Languages

### Rust Itself

```rust
// Rust code calling C library
extern "C" {
    fn C_function(x: i32) -> i32;  // C ABI declaration
}

// But C_function might be written in C++, Rust, anything!
```

### Go (cgo)

```go
/*
#include <stdio.h>
*/
import "C"

func Print() {
    C.puts(C.CString("Hello"))  // Uses C puts
    // ↑ Actually calling C here
}
```

### C# (P/Invoke)

```csharp
[DllImport("kernel32.dll")]
static extern bool Beep(int frequency, int duration);
// Calling Windows API (C ABI)
```

### Koa (Our Approach)

```rust
// Rust runtime
#[unsafe(no_mangle)]
pub extern "C" fn koa_println(s: *const c_char) {
    println!("{}", convert_to_string(s));
    // ↑ Using RUST'S standard library
    // C ABI is ONLY for the FFI boundary
}
```

## Summary

| Aspect | Implementation |
|--------|----------------|
| **Language** | Rust |
| **Standard Library** | Rust std::io, std::alloc, etc. |
| **FFI Boundary** | C ABI (extern "C") |
| **Memory Safety** | Rust's borrow checker |
| **Performance** | Native Rust performance |
| **Safety** | No C buffer overflows, use Rust instead |

## Takeaway

**We use C ABI for interoperability, not for implementation.**

The runtime is **100% Rust**, using **Rust's standard library**.
The C ABI is just the "protocol" for Koa to call into Rust.

Think of it like this:
- **C ABI** = USB protocol (standard connector)
- **Rust std** = The actual device (what does the work)
- **Koa** = Computer that uses the USB device
