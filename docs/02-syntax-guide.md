# Syntax Guide

Complete syntax reference for Koa.

## Semicolons

Semicolons are **required** at the end of statements, with exceptions:

- **Required**: Variable declarations, function calls, assignments, control flow statements
- **Not required**: After function definitions, struct definitions, enum definitions, if/else, loops, match arms

```typescript
// Required
let x: i32 = 42;
println!("Hello");
x + 1;

// Not required (function definition)
fn add(x: i32, y: i32): i32 {
    x + y
}

// Not required (control flow)
if x > 0 {
    x;
} else {
    0;
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

```typescript
// Immutable (default, recommended)
const name: string = "Koa"
const pi: f64 = 3.14159
const numbers: [i32; 3] = [1, 2, 3]

// Mutable (only when needed to change)
let counter: i32 = 0
counter += 1

// Variables MUST be initialized
const x: i32 = 42        // OK
let y: f64 = 3.14        // OK
let z: i32               // ERROR: must be initialized
```

### Variable Naming

- `snake_case` for variables and functions
- `PascalCase` for types
- No shadowing variables allowed

```typescript
const user_name: string = "Alice"   // OK
const UserName: string = "Bob"      // AVOID (use for types)
fn calculate_sum(): i32 { }          // OK

// Shadowing ERROR
fn example(): void {
    const x: i32 = 42
    if condition {
        const x: string = "error"   // ERROR: no shadowing
    }
}
```

---

## Primitive Types

### Integers

```typescript
// Signed integers
const a: i8 = 127
const b: i16 = 32767
const c: i32 = 2147483647
const d: i64 = 9223372036854775807

// Unsigned integers
const e: u8 = 255
const f: u16 = 65535
const g: u32 = 4294967295
const h: u64 = 18446744073709551615

// Pointer-sized
const i: isize = 42
const j: usize = 100
```

### Floats

```typescript
const x: f32 = 3.14
const y: f64 = 3.14159265359
```

### Other Primitives

```typescript
const b: bool = true
const s: string = "Hello"
const empty: void = ()
```

---

## Structs

Struct is a data structure with fields. Methods are defined directly in the struct body (Zig-style):

```typescript
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
        (dx * dx + dy * dy).sqrt()
    }

    // Static method
    pub fn origin(): Point {
        Self { x: 0.0, y: 0.0 }
    }
}
```

### Struct Initialization

```typescript
// Positional (if fields are in order)
let p1: Point = Point::new(1.0, 2.0)

// Named fields
let p2: Point = Point {
    x: 3.0,
    y: 4.0,
}

// Field access
println!("{}", p2.x)
println!("{}", p2.y)
```

### Generic Structs

```typescript
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

```typescript
// No parameters, no return
fn greet(): void {
    println!("Hello, World!")
}

// With parameters
fn add(x: i32, y: i32): i32 {
    x + y
}

// With return type
fn divide(x: f64, y: f64): f64 {
    x / y
}
```

### Async Functions

```typescript
async fn fetch_data(url: string): !Data {
    let response: HttpResponse = await http_get(url)
    response.data
}
```

### Generic Functions

```typescript
fn identity<T>(x: T): T {
    x
}

fn first<T>(arr: []T): T | null {
    if arr.len == 0 {
        return null
    }
    arr[0]
}
```

### Function Calls

```typescript
// Regular call
let result: i32 = add(10, 20)

// Async call
let data: Data = await fetch_data(url)

// Generic call (type inference)
let x: i32 = identity(42)
let y: string = identity("hello")
```

---

## Control Flow

### If/Else

```typescript
fn max(a: i32, b: i32): i32 {
    if a > b {
        a;
    } else {
        b;
    }
}
```

### Loops

```typescript
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

```typescript
fn process_file(path: string): !void {
    let file: File = try File::open(path);
    defer file.close();  // Cleanup on exit

    // Process file...
    // file.close() otomatis dipanggil
}
```

---

## Pattern Matching

### Switch Expression

```typescript
enum Color {
    Red,
    Green,
    Blue,
}

fn describe(c: Color): string {
    switch c {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
    }
}
```

### Destructuring

```typescript
struct Point {
    x: i32,
    y: i32,
}

fn match_point(p: Point): string {
    switch p {
        Point { x: 0, y: 0 } => "Origin",
        Point { x, y: 0 } => "On x-axis: " + x,
        Point { x: 0, y } => "On y-axis: " + y,
        Point { x, y } => "Point: (" + x + ", " + y + ")",
    }
}
```

### Wildcard

```typescript
fn classify(n: i32): string {
    switch n {
        0 => "Zero",
        1 | 2 | 3 => "Small",
        _ => "Other",  // Wildcard
    }
}
```

---

## Error Handling

### Error Sets

```typescript
const FileError = error {
    NotFound,
    AccessDenied,
    OutOfMemory,
}
```

### Error Union Type

```typescript
fn read_file(path: string): FileError!string {
    if !exists(path) {
        return error.NotFound
    }
    // ...
}
```

### Try Operator

```typescript
fn process(): !void {
    let data: string = try read_file("data.txt")
    println!("{}", data)
}
```

### Catch Handler

```typescript
fn main(): i32 {
    match process() {
        Ok(()) => 0,
        Err(error.NotFound) => {
            println!("File not found")
            1
        },
        Err(err) => {
            println!("Error: {}", err)
            2
        },
    }
}
```

---

## Generics

### Type Parameters

```typescript
fn identity<T>(x: T): T {
    x
}

fn pair<T, U>(x: T, y: U): (T, U) {
    (x, y)
}
```

### Generic Structs

```typescript
struct Vec<T> {
    data: *mut T,
    len: usize,

    pub fn push<T>(self: *mut Self, item: T): !void {
        // ...
    }
}
```

### Type Inference

```typescript
// Type inferred from usage
let x: i32 = identity(42)
let y: string = identity("hello")
```

---

## Annotations

Conditional compilation annotations:

```typescript
[@debug]
fn log_debug(msg: string): void {
    println!("DEBUG: {}", msg)
}

[@not_debug]
fn log_debug(msg: string): void {
    // No-op in release
}

[@release]
fn optimized(): void {
    // Release-only code
}

[@os_linux]
fn linux_only(): void {
    // Linux specific
}

[@feature_sqlite]
fn with_sqlite(): void {
    // If --feature sqlite
}
```

---

## Comments

```typescript
// Single-line comment

///
/// Doc comment for the next item
/// Can be multiple lines
///
pub fn documented(): void {
    //!
    //! Top-level doc comment (module documentation)
    //!
}

// No multiline comments (like Zig)
// Use multiple single-line comments
```

---

## Complete Example

```typescript
import { println, Vec } from "std/io"

struct Point {
    x: f64,
    y: f64,

    pub fn new(x: f64, y: f64): Self {
        Self { x, y }
    }

    pub fn distance(self, other: Point): f64 {
        let dx: f64 = self.x - other.x
        let dy: f64 = self.y - other.y
        (dx * dx + dy * dy).sqrt()
    }
}

fn main(): i32 {
    const p1: Point = Point::new(0.0, 0.0)
    const p2: Point = Point::new(3.0, 4.0)

    let dist: f64 = p1.distance(p2)

    println!("Distance: {}", dist)

    0
}
```
