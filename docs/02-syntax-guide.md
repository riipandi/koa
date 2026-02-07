# Syntax Guide

Complete syntax reference for Koa.

## Semicolons

Semicolons are **required** at the end of statements, with exceptions:

- **Required**: Variable declarations, function calls, assignments, control flow statements, return statements
- **Not required**: After function definitions, struct definitions, enum definitions, if/else, loops, match arms

```
// Required
let x: i32 = 42;
println!("Hello");
return x + 1;

// Not required (function definition)
fn add(x: i32, y: i32): i32 {
    return x + y;
}

// Not required (control flow)
if x > 0 {
    return x;
} else {
    return 0;
}
```

## Table of Contents

- [Semicolons](#semicolons)
- [Variables](#variables)
- [Primitive Types](#primitive-types)
- [Structs](#structs)
- [Functions](#functions)
- [Control Flow](#control-flow)
- [Pattern Matching](#pattern-matching)
- [Error Handling](#error-handling)
- [Generics](#generics)
- [Annotations](#annotations)
- [Comments](#comments)

---

## Variables

Koa uses `const` for immutable and `let` for mutable (like TypeScript's const/let):

```
// Immutable (default, recommended)
const name: string = "Koa";
const pi: f64 = 3.14159;
const numbers: [i32; 3] = [1, 2, 3];

// Mutable (only when needed to change)
let counter: i32 = 0;
counter += 1;

// Variables MUST be initialized
const x: i32 = 42;        // OK
let y: f64 = 3.14;        // OK
let z: i32;               // ERROR: must be initialized
```

### Variable Naming

- `snake_case` for variables and functions
- `PascalCase` for types
- No shadowing variables allowed

```
const user_name: string = "Alice";   // OK
const UserName: string = "Bob";      // AVOID (use for types)
fn calculate_sum(): i32 { }          // OK

// Shadowing ERROR
fn example(): void {
    const x: i32 = 42;
    if condition {
        const x: string = "error";   // ERROR: no shadowing
    }
}
```

---

## Primitive Types

### Integers

```
// Signed integers
const a: i8 = 127;
const b: i16 = 32767;
const c: i32 = 2147483647;
const d: i64 = 9223372036854775807;

// Unsigned integers
const e: u8 = 255;
const f: u16 = 65535;
const g: u32 = 4294967295;
const h: u64 = 18446744073709551615;

// Pointer-sized
const i: isize = 42;
const j: usize = 100;
```

### Floats

```
const x: f32 = 3.14;
const y: f64 = 3.14159265359;
```

### Other Primitives

```
const b: bool = true;
const s: string = "Hello";
const empty: void = ();
```

---

## Structs

Struct is a data structure with fields. Methods are defined directly in the struct body (Zig-style):

```
// Basic struct
pub struct Point {
    x: f64,
    y: f64,
}

// Struct with methods
pub struct Point {
    x: f64,
    y: f64,

    // Constructor
    pub fn new(x: f64, y: f64): Self {
        Self { x, y }
    }

    // Instance method
    pub fn distance(self, other: Point): f64 {
        let dx: f64 = self.x - other.x
        let dy: f64 = self.y - other.y
        (dx * dx + dy * dy).sqrt();
    }

    // Static method
    pub fn origin(): Point {
        Self { x: 0.0, y: 0.0 }
    }
}
```

### Struct Initialization

```
// Positional (if fields are in order)
let p1: Point = Point::new(1.0, 2.0)

// Named fields
let p2: Point = Point {
    x: 3.0,
    y: 4.0,
}

// Field access
println!("{}", p2.x);
println!("{}", p2.y);
```

### Generic Structs

```
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    cap: usize,

    pub fn new<T>(): Vec<T> {
        Vec {
            data: null,
            len: 0,
            cap: 0,
        }
    }

    pub fn push<T>(self: *mut Self, item: T): !void {
        // ...
    }
}
```

---

## Functions

### Basic Functions

**IMPORTANT:** The `return` keyword is **required** for all return values. No implicit returns.

```
// No parameters, no return
fn greet(): void {
    println!("Hello, World!");
}

// With parameters
fn add(x: i32, y: i32): i32 {
    return x + y;
}

// With return type
fn divide(x: f64, y: f64): f64 {
    return x / y;
}

// ERROR: No implicit return
fn add(x: i32, y: i32): i32 {
    x + y  // ERROR: use 'return' keyword
}
```

### Async Functions

```
async fn fetch_data(url: string): !Data {
    let response: HttpResponse = await http_get(url)
    response.data
}
```

### Generic Functions

```
fn identity<T>(x: T): T {
    return x;
}

fn first<T>(arr: []T): T | null {
    if arr.len == 0 {
        return null;
    }
    return arr[0];
}
```

### Function Calls

```
// Regular call
let result: i32 = add(10, 20);

// Async call
let data: Data = await fetch_data(url)

// Generic call (type inference)
let x: i32 = identity(42)
let y: string = identity("hello")
```

---

## Control Flow

### If/Else

```
fn max(a: i32, b: i32): i32 {
    if a > b {
        a;
    } else {
        b;
    }
}
```

### Loops

```
// While loop
while condition {
    // ...
}

// For loop (range)
for i in 0..10 {
    println!("{}", i);
}

// Infinite loop
loop {
    // ...
    if should_break {
        break;
    }
}
```

### Defer

```
fn process_file(path: string): !void {
    let file: File = try File::open(path);
    defer file.close();  // Cleanup on exit

    // Process file...
    // file.close() otomatis dipanggil
}
```

---

## Pattern Matching

### Match Expression

```
enum Color {
    Red,
    Green,
    Blue,
}

fn describe(c: Color): string {
    match c {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
    }
}
```

### Destructuring

```
struct Point {
    x: i32,
    y: i32,
}

fn match_point(p: Point): string {
    match p {
        Point { x: 0, y: 0 } => "Origin",
        Point { x, y: 0 } => "On x-axis: " + x,
        Point { x: 0, y } => "On y-axis: " + y,
        Point { x, y } => "Point: (" + x + ", " + y + ")",
    }
}
```

### Wildcard

```
fn classify(n: i32): string {
    match n {
        0 => "Zero",
        1 | 2 | 3 => "Small",
        _ => "Other",  // Wildcard
    }
}
```

---

## Error Handling

### Error Sets

```
const FileError = error {
    NotFound,
    AccessDenied,
    OutOfMemory,
}
```

### Error Union Type

```
fn read_file(path: string): FileError!string {
    if !exists(path) {
        return error.NotFound
    }
    // ...
}
```

### Try Operator

```
fn process(): !void {
    let data: string = try read_file("data.txt");
    println!("{}", data);
}
```

### Catch Handler

```
fn main(): i32 {
    match process() {
        Ok(()) => 0,
        Err(error.NotFound) => {
            println!("File not found");
            1;
        },
        Err(err) => {
            println!("Error: {}", err);
            2;
        },
    }
}
```

---

## Generics

### Type Parameters

```
fn identity<T>(x: T): T {
    return x;
}

fn pair<T, U>(x: T, y: U): (T, U) {
    return (x, y);
}
```

### Generic Structs

```
struct Vec<T> {
    data: *mut T,
    len: usize,

    pub fn push<T>(self: *mut Self, item: T): !void {
        // ...
    }
}
```

### Type Inference

```
// Type inferred from usage
let x: i32 = identity(42)
let y: string = identity("hello")
```

---

## Annotations

Conditional compilation annotations:

```
[@debug]
fn log_debug(msg: string): void {
    println!("DEBUG: {}", msg);
}
```

---

## Comments

Koa uses Rust-style comments:

```
// Single-line comment for inline documentation

///
/// Documentation comment for the next item
///
/// # Examples
/// ```
/// const result: i32 = add(1, 2);
/// assert_eq!(result, 3);
/// ```
///
/// # Parameters
/// - `x`: First number
/// - `y`: Second number
///
/// # Returns
/// The sum of x and y
///
/// # Errors
/// This function doesn't return errors
///
pub fn add(x: i32, y: i32): i32 {
    return x + y;
}

//!
//! Module-level documentation
//!
//! This module provides basic arithmetic operations.
//!`

// Regular comments for implementation details
fn helper(): void {
    // TODO: Improve this algorithm
    // FIXME: This has a bug when x < 0

    // NOTE: Using optimized approach
    let result: i32 = 0;
}
```

**Comment Guidelines:**
- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Use `//` for inline comments
- Support markdown in doc comments
- Include examples, parameters, returns, and errors sections
- Testable examples in doc comments (future feature)

---

## Complete Example

```
import from "std/io/println";
import from "std/math/sqrt";

///
/// Represents a 2D point
///
pub struct Point {
    x: f64,
    y: f64,

    ///
    /// Creates a new point
    ///
    pub fn new(x: f64, y: f64): Self {
        return Self { x, y };
    }

    ///
    /// Calculates the distance to another point
    ///
    pub fn distance(self, other: Point): f64 {
        let dx: f64 = self.x - other.x;
        let dy: f64 = self.y - other.y;
        return sqrt(dx * dx + dy * dy);
    }
}

fn main(): i32 {
    const p1: Point = Point::new(0.0, 0.0);
    const p2: Point = Point::new(3.0, 4.0);

    let dist: f64 = p1.distance(p2);

    println("Distance: {}", dist);

    return 0;
}
```

---

## The main() Function

Koa supports two signatures for the `main()` entry point:

### 1. Simple Programs (void return)

For simple scripts and beginners:

```
fn main(): void {
    println!("Hello, World!");
    // Automatically returns 0 to the operating system
}
```

**Use `void` when:**
- You don't need to signal specific error codes
- Simple scripts and examples
- Learning the language

**Behavior:** Always exits with code `0` (success)

### 2. Programs with Error Handling (i32 return)

For proper applications with error signaling:

```
fn main(): i32 {
    if args.len < 2 {
        eprintln!("Usage: {} <name>", args[0]);
        return 1;  // Return non-zero for errors
    }
    
    let name: string = args[1];
    println!("Hello, {}!", name);
    return 0;  // Return 0 for success
}
```

**Use `i32` when:**
- You need to signal different error types
- CLI tools that need proper exit codes
- Integration with shell scripts/CI/CD
- Production applications

**Exit Code Conventions:**
- `0` - Success
- `1` - General error
- `2` - Usage error (invalid arguments)
- `127` - Command not found

### Examples

**Success:**
```
fn main(): void {
    println!("Success!");
}
```

**Error Handling:**
```
fn main(): i32 {
    match load_config() {
        Ok(config) => {
            run_app(config);
            return 0;  // Success
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            return 1;  // Error
        },
    }
}
```

**Using Try Operator:**
```
fn main(): i32 {
    let config: Config = try load_config();
    run_app(config);
    return 0;
}
```
```
