# FFI (Foreign Function Interface)

Koa has **C-compatible FFI** for interoperability with C libraries (like Rust's extern "C").

## Philosophy

- **C ABI compatible** - Call C functions directly
- **No wrapper generation** - Manual bindings
- **Type-safe** - Compile-time checking
- **Zero-cost** - Direct calls, no overhead

---

## Calling C Functions

### Extern Function Declaration

```typescript
// Declare external C function
extern "C" fn printf(format: *const u8, ...) -> i32;
extern "C" fn strlen(s: *const u8) -> usize;
extern "C" fn malloc(size: usize) -> *mut u8;
extern "C" fn free(ptr: *mut u8) -> void;
```

### Usage

```typescript
fn main(): i32 {
    // Call C function
    printf("Hello, World!\n".as_ptr())

    0
}
```

---

## C Types Mapping

| Koa Type   | C Type     | Description            |
|------------|------------|------------------------|
| `i8`       | `int8_t`   | Signed 8-bit           |
| `i16`      | `int16_t`  | Signed 16-bit          |
| `i32`      | `int32_t`  | Signed 32-bit          |
| `i64`      | `int64_t`  | Signed 64-bit          |
| `u8`       | `uint8_t`  | Unsigned 8-bit         |
| `u16`      | `uint16_t` | Unsigned 16-bit        |
| `u32`      | `uint32_t` | Unsigned 32-bit        |
| `u64`      | `uint64_t` | Unsigned 64-bit        |
| `isize`    | `intptr_t` | Signed pointer-sized   |
| `usize`    | `size_t`   | Unsigned pointer-sized |
| `f32`      | `float`    | 32-bit float           |
| `f64`      | `double`   | 64-bit float           |
| `bool`     | `bool`     | Boolean                |
| `*const T` | `const T*` | Immutable pointer      |
| `*mut T`   | `T*`       | Mutable pointer        |
| `fn(T): R` | `R (*)(T)` | Function pointer       |

---

## Passing Strings

### C String to Koa

```typescript
extern "C" fn strlen(s: *const u8) -> usize;

fn main(): i32 {
    let s: string = "Hello"
    let len: usize = strlen(s.as_ptr())
    println!("Length: {}", len)
    0
}
```

### Koa String to C

```typescript
extern "C" fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8;

fn copy_to_c(s: string): *mut u8 {
    let buf: *mut u8 = malloc(s.len + 1)
    strcpy(buf, s.as_ptr())
    buf
}
```

---

## Structs with C Layout

### extern struct

```typescript
// C-compatible struct (explicit layout)
extern struct Point {
    x: f64,
    y: f64,
}

// Use with C functions
extern "C" fn point_distance(p1: *const Point, p2: *const Point) -> f64;

fn main(): i32 {
    let p1: Point = Point { x: 0.0, y: 0.0 }
    let p2: Point = Point { x: 3.0, y: 4.0 }

    let dist: f64 = point_distance(&p1, &p2)
    println!("Distance: {}", dist)

    0
}
```

### C Header Equivalent

```c
// point.h
typedef struct Point {
    double x;
    double y;
} Point;

double point_distance(const Point* p1, const Point* p2);
```

---

## Callbacks

### Function Pointers

```typescript
// C function expecting callback
extern "C" fn qsort(
    base: *mut u8,
    num: usize,
    size: usize,
    compar: fn(*const u8, *const u8): i32
) -> void;

// Callback function
fn compare_ints(a: *const u8, b: *const u8): i32 {
    let a_val: i32 = *(a as *const i32)
    let b_val: i32 = *(b as *const i32)

    if a_val < b_val {
        -1
    } else if a_val > b_val {
        1
    } else {
        0
    }
}

fn sort_array(arr: []i32): void {
    qsearch(
        arr.as_mut_ptr() as *mut u8,
        arr.len,
        size_of(i32),
        compare_ints
    )
}
```

---

## Memory Management

### Allocating Memory for C

```typescript
extern "C" fn malloc(size: usize) -> *mut u8;
extern "C" fn free(ptr: *mut u8);

fn allocate_c_buffer(size: usize): *mut u8 {
    malloc(size)
}

fn free_c_buffer(ptr: *mut u8): void {
    free(ptr)
}
```

### Koa Objects to C

```typescript
// Expose Koa object to C
export "C" fn create_user(name: *const u8, age: i32) -> *mut User {
    let user: *mut User = alloc(User)
    user.name = copy_from_c_string(name)
    user.age = age
    user
}

export "C" fn free_user(user: *mut User): void {
    free_c_string(user.name)
    free(user)
}
```

---

## Exporting Functions to C

### export "C"

```typescript
// Exported function callable from C
export "C" fn add(a: i32, b: i32): i32 {
    a + b
}

export "C" fn greet(name: *const u8): void {
    println!("Hello, {}!", string_from_c(name))
}
```

### C Usage

```c
// main.c
#include <stdio.h>

// Declare external functions
extern int32_t add(int32_t a, int32_t b);
extern void greet(const char* name);

int main() {
    int result = add(10, 20);
    printf("10 + 20 = %d\n", result);

    greet("World");

    return 0;
}
```

### Compilation

```bash
# Compile Koa code to object file
koa build --emit-obj libkoa.o

# Compile C code
gcc -c main.c -o main.o

# Link
gcc main.o libkoa.o -o myapp

# Run
./myapp
```

---

## Linking C Libraries

### Linking System Libraries

```toml
# Koa.toml
[package]
name = "myapp"

[dependencies]
sqlite = { git = "...", version = "0.1.0" }

[link-system]
"ssl"      # OpenSSL
"crypto"   # Crypto library
```

### Linking Local Libraries

```toml
[link-local]
mylib = { path = "./lib/mylib" }
```

---

## Practical Examples

### 1. OpenSSL Integration

```typescript
extern "C" fn MD5_Init(c: *mut MD5_CTX) -> i32;
extern "C" fn MD5_Update(c: *mut MD5_CTX, data: *const u8, len: usize) -> i32;
extern "C" fn MD5_Final(md: *mut u8, c: *mut MD5_CTX) -> i32;

extern struct MD5_CTX {
    // ... opaque
}

fn compute_md5(data: []u8): [u8; 16] {
    let ctx: MD5_CTX
    MD5_Init(&ctx)

    MD5_Update(&ctx, data.as_ptr(), data.len)

    let hash: [u8; 16]
    MD5_Final(hash.as_mut_ptr(), &ctx)

    hash
}
```

### 2. libcurl HTTP

```typescript
extern "C" fn curl_easy_init() -> *mut CURL;
extern "C" fn curl_easy_setopt(curl: *mut CURL, option: CURLoption, ...) -> CURLcode;
extern "C" fn curl_easy_perform(curl: *mut CURL) -> CURLcode;
extern "C" fn curl_easy_cleanup(curl: *mut CURL) -> void;

extern struct CURL {}
extern enum CURLoption {}
extern enum CURLcode {}

async fn http_get(url: string): !string {
    let curl: *mut CURL = curl_easy_init()
    defer curl_easy_cleanup(curl)

    curl_easy_setopt(curl, CURLOPT_URL, url.as_ptr())

    let result: CURLcode = curl_easy_perform(curl)
    if result != CURLE_OK {
        return error.HttpFailed
    }

    // Parse response...
    "response"
}
```

### 3. SQLite FFI

```typescript
extern "C" fn sqlite3_open(filename: *const u8, db: **mut sqlite3) -> i32;
extern "C" fn sqlite3_exec(
    db: *mut sqlite3,
    sql: *const u8,
    callback: fn(*mut u8, i32, **mut u8, **mut u8): i32,
    arg: *mut u8,
    err: **mut u8
) -> i32;
extern "C" fn sqlite3_close(db: *mut sqlite3) -> i32;

extern struct sqlite3 {}

fn create_table(db: *mut sqlite3): !void {
    let sql: string = "CREATE TABLE users (id INTEGER, name TEXT)"

    let result: i32 = sqlite3_exec(
        db,
        sql.as_ptr(),
        null,  // No callback
        null,
        null
    )

    if result != SQLITE_OK {
        return error.SqlError
    }
}
```

---

## Best Practices

### 1. Use extern struct for C Compatibility

```typescript
// GOOD: C-compatible layout
extern struct Point {
    x: f64,
    y: f64,
}

// BAD: Not C-compatible
struct Point {
    x: f64,
    y: f64,
}
```

### 2. Check Return Values

```typescript
// GOOD: Check error codes
let result: i32 = sqlite3_open(db_path.as_ptr(), &db)
if result != SQLITE_OK {
    return error.CannotOpen
}

// BAD: Ignore errors
sqlite3_open(db_path.as_ptr(), &db)
```

### 3. Manual Memory Management

```typescript
// GOOD: Explicit cleanup
let ptr: *mut u8 = malloc(1024)
defer free(ptr)

// Use ptr...

// BAD: Memory leak
let ptr: *mut u8 = malloc(1024)
// No free!
```

### 4. Type Conversion Helpers

```typescript
// Helper for C string conversion
fn c_string_to_koa(s: *const u8, len: usize): string {
    // Convert C string to Koa string
}

fn koa_string_to_c(s: string): (*const u8, usize) {
    (s.as_ptr(), s.len)
}
```

---

## Comparison with Rust

| Feature               | Rust            | Koa               |
|-----------------------|-----------------|-------------------|
| **Declaration**       | `extern "C" fn` | `extern "C" fn`   |
| **Struct Layout**     | `#[repr(C)]`    | `extern struct`   |
| **Export**            | `#[no_mangle]`  | `export "C"`      |
| **String Conversion** | Manual          | Manual            |
| **Safety**            | unsafe required | unsafe (implicit) |

---

## Future Enhancements

### bindgen Integration (Phase 3)

Automatic C header parsing:

```bash
# Generate bindings from C headers
koa bindgen /usr/include/sqlite3.h --output sqlite3_bindings.koa
```

### C++ Integration (Phase 4)

```typescript
extern "C++" fn vector_push(v: *mut std::vector, val: i32) -> void;
```

---

## Next Steps

- [Implementation Plan](10-implementation-plan.md) - FFI in Phase 4
- [Database Drivers](12-database-drivers.md) - Driver implementations use FFI

---

## Implementation Notes

### Runtime Requirements

- FFI thunks for calling convention conversion
- C-compatible type definitions
- Linker integration for symbol resolution

### Safety Considerations

- FFI calls are unsafe by default
- No runtime checks for FFI
- User must ensure memory safety
