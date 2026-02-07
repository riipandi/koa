# Koa Standard Library

The Koa standard library provides essential functionality for writing Koa programs, including I/O operations, string manipulation, mathematical functions, and more.

## Structure

```
std/
├── mod.koa      # Root module (re-exports everything)
├── io.koa       # Input/output operations
├── string.koa   # String manipulation
├── convert.koa  # Type conversions
└── math.koa     # Mathematical functions
```

## Modules

### I/O (`io.koa`)

Input and output operations:

```koa
import from "std/io";

io.println("Hello, World!");
let name: string = io.readline();
io.printf("Hello, %s\n", name);
```

### String (`string.koa`)

String manipulation functions:

```koa
import from "std/string";

let upper: string = string.to_upper("hello");
let trimmed: string = string.trim("  hello  ");
```

### Convert (`convert.koa`)

Type conversion utilities:

```koa
import from "std/convert";

let num: i32 = convert.string_to_i32("42");
let str: string = convert.i32_to_string(42);
```

### Math (`math.koa`)

Mathematical functions and constants:

```koa
import from "std/math";

let pi: f64 = math.PI;
let result: f64 = math.sqrt(16.0);
let abs_val: f64 = math.abs(-5.0);
```

## Usage

### Importing the Entire Library

```koa
import from "std";

io.println("Hello");
math.sqrt(16.0);
```

### Importing Specific Modules

```koa
import from "std/io";
import from "std/math";

io.println("Hello");
math.sqrt(16.0);
```

### Importing Specific Items

```koa
import from "std/io/println";

println("Hello");
```

## Implementation Notes

### External Functions

Most I/O functions are declared as `extern` and implemented in the runtime library:

```koa
extern fn koa_println(s: string): void;
```

These functions link to Rust implementations in `cr/koa-runtime/src/io.rs`.

### Stub Functions

Many functions in `string.koa`, `convert.koa`, and `math.koa` are currently stubs that return default values:

```koa
pub fn sqrt(x: f64): f64 {
    // TODO: Implement actual sqrt
    return 0.0;
}
```

These will be implemented in future versions.

## Building

The standard library is automatically included when compiling Koa programs. No separate build step is required.

## Contributing

When adding new standard library functions:

1. **Add declaration** in appropriate `.koa` file
2. **Add implementation** in `crates/koa-runtime/src/`
3. **Export** from `crates/koa-runtime/src/lib.rs`
4. **Document** with examples
5. **Add tests** if applicable

## TODO

### I/O Module
- [ ] File operations (open, read, write, close)
- [ ] Path manipulation
- [ ] Environment variables
- [ ] Command-line arguments

### String Module
- [ ] Implement actual string operations
- [ ] Unicode support
- [ ] Regular expressions
- [ ] String formatting

### Convert Module
- [ ] Implement actual conversions
- [ Handle parsing errors
- [ ] Support for more types

### Math Module
- [ ] Implement actual math functions (sin, cos, tan, etc.)
- [ ] Trigonometry functions
- [ ] Logarithms
- [ ] Random number generation

## See Also

- [Standard Library Architecture](../../docs/16-stdlib-architecture.md)
- [Runtime Library](../koa-runtime/)
- [Language Guide](../../docs/02-syntax-guide.md)
