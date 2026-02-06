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

```typescript
i8, i16, i32, i64      // Signed: 8, 16, 32, 64 bit
u8, u16, u32, u64      // Unsigned: 8, 16, 32, 64 bit
isize, usize           // Pointer-sized
```

### Floats

```typescript
f32  // IEEE 754 binary32
f64  // IEEE 754 binary64
```

### Other

```typescript
bool    // true or false
string  // UTF-8 string
void    // Unit type, single value ()
```

---

## Type Compatibility

### Structural Typing (Primitives)

```typescript
// Primitives: structural compatibility
let x: i32 = 42
let y: i64 = x  // OK: widening conversion

let a: f64 = 3.14
let b: f32 = a  // ERROR: narrowing, requires explicit cast
```

### Nominal Typing (Structs/Enums)

```typescript
struct Point2D {
    x: f64,
    y: f64,
}

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// ERROR: Nama type berbeda, tidak kompatible
fn accept(p: Point2D): void { }
let p3d: Point3D = Point3D { x: 1.0, y: 2.0, z: 3.0 }
accept(p3d)  // ERROR: expected Point2D, found Point3D
```

---

## Nullable Types

Koa doesn't have `null` as an implicit value. Use explicit nullable:

```typescript
// Explicit nullable
fn find_user(id: i32): User | null {
    switch database.query(id) {
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

```typescript
// Fixed-size array
let arr: [i32; 3] = [1, 2, 3]

// Slice (dynamic size)
let slice: []i32 = arr

// Vec (dynamic, growable)
let vec: Vec<i32> = Vec::new()
```

---

## Pointer Types

```typescript
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

```typescript
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

## Type Annotations

### Always Explicit

```typescript
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

### Type Parameters

```typescript
// Generic function
fn identity<T>(x: T): T {
    x
}

// Multiple type parameters
fn pair<T, U>(x: T, y: U): (T, U) {
    (x, y)
}
```

### Generic Structs

```typescript
struct Vec<T> {
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
}
```

### Constraints (Future)

Type constraints will be added in Phase 2:

```typescript
// Example future syntax
fn max<T: Comparable>(a: T, b: T): T {
    if a.compare(b) > 0 {
        a
    } else {
        b
    }
}
```

---

## Type Casting

### Safe Coercions (Implicit)

```typescript
// Widening: OK
let x: i32 = 42
let y: i64 = x  // OK

// Integer to float: OK
let a: i32 = 42
let b: f64 = a  // OK
```

### Unsafe Casts (Explicit)

```typescript
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

```typescript
// Function return inference (minimal)
fn add(x: i32, y: i32): i32 {
    x + y  // Return type inferred from body
}

// Variable inference: NOT allowed
let x = 42  // ERROR: type annotation required
```

---

## Enum Types

```typescript
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

```typescript
// Pair
let pair: (i32, string) = (42, "hello")

// Triple
let triple: (i32, f64, bool) = (1, 2.0, true)

// Destructuring
let (x, y) = pair
```

---

## Type Aliases

```typescript
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
