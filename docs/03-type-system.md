# Type System

Koa has a static type system with hybrid approach: structural for primitives, nominal for structs/enums.

## Overview

- **Static typing** - All types are checked at compile time
- **Type inference** - Limited inference for function returns
- **Generics** - Type parameters in Phase 1
- **No null** - Explicit nullable types with `T | null`

---

## Primitive Types

### Integers

```
i8, i16, i32, i64      // Signed: 8, 16, 32, 64 bit
u8, u16, u32, u64      // Unsigned: 8, 16, 32, 64 bit
isize, usize           // Pointer-sized
```

### Floats

```
f32  // IEEE 754 binary32
f64  // IEEE 754 binary64
```

### Other

```
bool    // true or false
string  // UTF-8 string
void    // Unit type, single value ()
```

---

## Type Compatibility

### Structural Typing (Primitives)

```
// Primitives: structural compatibility
let x: i32 = 42
let y: i64 = x  // OK: widening conversion

let a: f64 = 3.14
let b: f32 = a  // ERROR: narrowing, requires explicit cast
```

### Nominal Typing (Structs/Enums)

```
struct Point2D {
    x: f64,
    y: f64,
}

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// ERROR: Type names differ, incompatible
fn accept(p: Point2D): void { }
let p3d: Point3D = Point3D { x: 1.0, y: 2.0, z: 3.0 }
accept(p3d)  // ERROR: expected Point2D, found Point3D
```

---

## Nullable Types

Koa doesn't have `null` as an implicit value. Use explicit nullable:

```
// Explicit nullable
fn find_user(id: i32): User | null {
    match database.query(id) {
        Ok(user) => user,
        Err(_) => null,
    }
}

// Usage
let user: User | null = find_user(42)
match user {
    Some(u) => println!("User: {}", u.name),
    None => println!("Not found"),
}
```

---

## Array and Slice Types

```
// Fixed-size array
let arr: [i32; 3] = [1, 2, 3]

// Slice (dynamic size)
let slice: []i32 = arr

// Vec (dynamic, growable)
let vec: Vec<i32> = Vec::new()
```

---

## Pointer Types

```
// Mutable pointer
let x: i32 = 42
let ptr: *mut i32 = &x

// Immutable pointer
let y: i32 = 42
let ptr: *const i32 = &y

// Optional pointer
let ptr: *mut User | null = get_user_pointer()
```

---

## Function Types

```
// Function type
type Callback = fn(i32, i32): i32

// Higher-order function
fn apply(f: fn(i32): i32, x: i32): i32 {
    f(x)
}

// Async function type
type AsyncCallback = async fn(string): !void
```

---

## Interfaces (Structural)

**Status**: ✅ Implemented (Phase 2)

Koa uses **Structural Interfaces** (similar to Go/TypeScript) to define behavior contracts. Types implicitly satisfy interfaces if they implement all required methods.

```
// Interface definition
interface Stringer {
    fn to_string(self): string;
}

// Struct implicitly implements if methods match
struct Point {
    x: i32;
    y: i32;

    pub fn to_string(self): string {
        return "Point(" + self.x + ", " + self.y + ")";
    }
}

// Point automatically satisfies Stringer
fn print_any<T: Stringer>(item: T): void {
    println!("{}", item.to_string());
    return;
}
```

### Generic Interfaces

```
interface Container<T> {
    fn get(self): T;
    fn set(self, value: T): void;
}

struct Box<T> {
    value: T;

    fn get(self): T {
        return self.value;
    }

    fn set(self, value: T): void {
        self.value = value;
        return;
    }
}
```

### Interface Satisfaction Checking

The compiler automatically verifies that types satisfy interface requirements:

```
interface Drawable {
    fn draw(self): void;
}

struct Circle {
    radius: f64;
    // Missing draw() method
}

fn render<T: Drawable>(shape: T): void {
    shape.draw();
    return;
}

render<Circle>(circle);  // ❌ ERROR: Circle does not implement 'draw'
```


---

## Type Annotations

### Always Explicit

```
// Variables: type annotation required
const x: i32 = 42           // OK
let y: f64 = 3.14           // OK

// ERROR: Type annotation required
let z = 42                  // ERROR

// Function parameters: always explicit
fn add(x: i32, y: i32): i32 {  // OK
    x + y
}

// Function returns: always explicit
fn identity<T>(x: T): T {  // OK
    x
}
```

---

## Generics

**Status**: ✅ Implemented (Phase 2)

Koa supports full generic programming with type parameters, constraints, and monomorphization.

### Type Parameters

```
// Generic function
fn identity<T>(x: T): T {
    return x;
}

// Usage with explicit type arguments
let x: i32 = identity<i32>(42);
let y: f64 = identity<f64>(3.14);

// Multiple type parameters
fn pair<T, U>(x: T, y: U): (T, U) {
    return (x, y);
}
```

### Generic Structs

```
struct Box<T> {
    value: T;
}

// Instantiation with type arguments
let int_box: Box<i32> = Box<i32> { value: 42 };
let str_box: Box<string> = Box<string> { value: "hello" };

// Generic struct with methods
struct Vec<T> {
    data: *mut T;
    len: usize;
    cap: usize;

    pub fn new(): Vec<T> {
        return Vec {
            data: null,
            len: 0,
            cap: 0,
        };
    }

    pub fn push(self, item: T): void {
        // Implementation
        return;
    }
}
```

### Generic Constraints (Interfaces)

Generics can be constrained using Interfaces to ensure type safety:

```
// Define an interface
interface Printable {
    fn print(self): void;
}

// Struct implementing the interface
struct Book {
    title: string;
    
    fn print(self): void {
        println!("Book: {}", self.title);
        return;
    }
}

// Generic function with constraint
fn show<T: Printable>(item: T): void {
    item.print();
    return;
}

// Valid: Book implements Printable
let book: Book = Book { title: "Koa Guide" };
show<Book>(book);  // ✅ OK

// Invalid: i32 does not implement Printable
show<i32>(42);  // ❌ ERROR: i32 does not implement Printable
```

### Multiple Constraints

```
interface Comparable {
    fn compare(self, other: Self): i32;
}

interface Equatable {
    fn equals(self, other: Self): bool;
}

// Multiple constraints with +
fn sort<T: Comparable + Equatable>(items: []T): void {
    // Can use both compare() and equals()
    return;
}
```

### Monomorphization

Koa uses **monomorphization** (like Rust) instead of runtime generics:

```
fn identity<T>(x: T): T {
    return x;
}

fn main(): void {
    identity<i32>(42);   // Generates: identity<I32>
    identity<f64>(3.14); // Generates: identity<F64>
    return;
}
```

**Compiled IR**:
```
fn identity<I32>(x: i32): i32 { return x; }
fn identity<F64>(x: f64): f64 { return x; }
fn main(): void { ... }
```

**Benefits**:
- Zero runtime overhead
- Full type safety at compile time
- Optimized code for each type
- No vtables or dynamic dispatch needed

**Trade-offs**:
- Larger binary size (one copy per type)
- Longer compilation time
- No runtime polymorphism



---

## Type Casting

### Safe Coercions (Implicit)

```
// Widening: OK
let x: i32 = 42
let y: i64 = x  // OK

// Integer to float: OK
let a: i32 = 42
let b: f64 = a  // OK
```

### Unsafe Casts (Explicit)

```
// Narrowing: explicit cast required
let x: i64 = 42
let y: i32 = @int_cast(i32, x)  // Explicit

// Pointer casting
let ptr: *mut i32 = &x
let bytes: *mut u8 = @ptr_cast(*mut u8, ptr)
```

---

## Type Inference

### Limited Inference

```
// Function return inference (minimal)
fn add(x: i32, y: i32): i32 {
    x + y  // Return type inferred from body
}

// Variable inference: NOT allowed
let x = 42  // ERROR: type annotation required
```

---

## Enum Types

```
enum Color {
    Red,
    Green,
    Blue,
}

enum Option<T> {
    Some(T),
    None,
}

// Enum with data
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

---

## Tuple Types

```
// Pair
let pair: (i32, string) = (42, "hello")

// Triple
let triple: (i32, f64, bool) = (1, 2.0, true)

// Destructuring
let (x, y) = pair
```

---

## Type Aliases

```
// Type alias
type UserId = i32
type Username = string

// With generics
type Optional<T> = T | null

// Complex alias
type Callback = fn(i32, i32): i32
```

---

## Summary Table

| Type | Example | Description |
|------|---------|-------------|
| **Integers** | `i32`, `u64` | Signed/unsigned integers |
| **Floats** | `f32`, `f64` | IEEE 754 floating point |
| **Bool** | `bool` | `true` or `false` |
| **String** | `string` | UTF-8 string |
| **Array** | `[T; N]` | Fixed-size array |
| **Slice** | `[]T` | Dynamic slice |
| **Pointer** | `*T`, `*const T` | Mutable/immutable pointer |
| **Nullable** | `T \| null` | Explicit nullable |
| **Function** | `fn(T): R` | Function type |
| **Tuple** | `(T, U)` | Fixed-size heterogeneous |
| **Enum** | `enum E { A, B }` | Enumeration |
| **Struct** | `struct S { x: T }` | Struct type |
| **Generic** | `Vec<T>` | Parameterized type |

---

## Next Steps

- [Error Handling](04-error-handling.md) - Learn Zig-style error handling
- [Syntax Guide](02-syntax-guide.md) - Back to syntax reference
