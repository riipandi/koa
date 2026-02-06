# Standard Library

The Koa standard library ("std") provides common types, functions, and modules for daily programming.

## Philosophy

- **Batteries included** - Common utilities built-in
- **Minimal but complete** - Essentials only, no bloat
- **Idiomatic** - Follow Koa conventions
- **Well-tested** - Comprehensive test coverage

---

## Module Structure

```
std/
├── io/
│   ├── mod.koa           # I/O operations
│   └── file.koa          # File I/O
├── collections/
│   ├── vec.koa           # Dynamic array
│   ├── hashmap.koa       # Hash map
│   └── hashset.koa       # Hash set
├── net/
│   ├── http.koa          # HTTP client
│   └── tcp.koa           # TCP networking
├── sync/
│   ├── mutex.koa         # Mutex
│   └── channel.koa       # Channels (future)
├── async/
│   └── runtime.koa       # Async runtime
├── error/
│   └── mod.koa           # Error types
├── time/
│   └── mod.koa           # Time operations
└── math/
    └── mod.koa           # Math functions
```

---

## Core Modules

### std/io

Input/output operations:

```typescript
import { print, println, File } from "std/io/mod"

// Print functions
fn main(): i32 {
    println!("Hello, World!")
    print!("Number: {}", 42)
    0
}

// File I/O
async fn read_file(path: string): !string {
    let file: File = try File::open(path)
    defer file.close()
    file.read_to_string()
}
```

### std/collections

#### Vec<T>

Dynamic array:

```typescript
import { Vec } from "std/collections/vec"

fn main(): i32 {
    let numbers: Vec<i32> = Vec::new()
    try numbers.push(1)
    try numbers.push(2)
    try numbers.push(3)

    match numbers.get(0) {
        Some(value) => println!("First: {}", value),
        None => {},
    }

    0
}
```

#### HashMap<K, V>

Hash map:

```typescript
import { HashMap } from "std/collections/hashmap"

fn main(): i32 {
    let map: HashMap<string, i32> = HashMap::new()
    try map.insert("one", 1)
    try map.insert("two", 2)

    match map.get("one") {
        Some(value) => println!("Found: {}", value),
        None => println!("Not found"),
    }

    0
}
```

### std/net

HTTP client:

```typescript
import { http_get, http_post } from "std/net/http"

async fn fetch_example(): !string {
    let response: HttpResponse = await http_get("https://example.com")
    response.body
}

async fn post_data(): !HttpResponse {
    let data: string = "{\"key\":\"value\"}"
    await http_post("https://api.example.com", data)
}
```

### std/error

Common error types:

```typescript
import { IOError, ParseError } from "std/error/mod"

fn read_config(): !(IOError | ParseError) {
    // ...
}
```

### std/time

Time operations:

```typescript
import { sleep, timestamp } from "std/time/mod"

async fn wait(): !void {
    await sleep(1000)  // Sleep 1000ms
    println!("Done")
}

fn measure<T>(f: fn(): T): T {
    let start: i64 = timestamp()
    let result: T = f()
    let elapsed: i64 = timestamp() - start
    println!("Took {} ms", elapsed)
    result
}
```

---

## Common Types

### Option<T>

Optional value (explicit nullable):

```typescript
enum Option<T> {
    Some(T),
    None,
}

fn find_user(id: i32): Option<User> {
    match database.query(id) {
        Ok(user) => Option::Some(user),
        Err(_) => Option::None,
    }
}
```

### Result<T, E>

Error handling:

```typescript
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: f64, b: f64): Result<f64, string> {
    if b == 0.0 {
        return Result::Err("Division by zero")
    }
    Result::Ok(a / b)
}
```

---

## Utility Functions

### String Operations

```typescript
// Concatenation
let message: string = "Hello" + " " + "World"

// Length
let len: usize = message.len()

// Comparison
if message == "Hello World" {
    // ...
}
```

### Math Functions

```typescript
import { sqrt, pow, sin, cos } from "std/math/mod"

fn calculate(): f64 {
    let x: f64 = 2.0
    let result: f64 = sqrt(x) + pow(x, 2.0)
    result
}
```

---

## Examples

### 1. Web Server

```typescript
import { println, http_get } from "std/io"
import { HashMap } from "std/collections"

async fn handle_request(req: HttpRequest): !HttpResponse {
    match req.path {
        "/" => HttpResponse::ok("Hello"),
        "/api/users" => {
            let users: Vec<User> = await fetch_users()
            HttpResponse::json(users)
        },
        _ => HttpResponse::not_found(),
    }
}

async fn main(): !void {
    let server: Server = Server::bind("0.0.0.0:8080")
    server.run(handle_request).await
}
```

### 2. File Processing

```typescript
import { File } from "std/io/mod"
import { Vec } from "std/collections/vec"

async fn process_lines(path: string): !Vec<string> {
    let file: File = try File::open(path)
    defer file.close()

    let lines: Vec<string> = Vec::new()
    let reader: LineReader = file.lines()

    loop {
        match await reader.next_line() {
            Option::Some(line) => try lines.push(line),
            Option::None => break,
        }
    }

    lines
}
```

### 3. Configuration

```typescript
import { HashMap } from "std/collections/hashmap"

struct Config {
    port: i32,
    debug: bool,
}

fn load_config(): !Config {
    let map: HashMap<string, string> = try read_config_file("config.toml")

    let port: i32 = match map.get("port") {
        Some(value) => try parse_i32(value),
        None => 8080,
    }

    let debug: bool = match map.get("debug") {
        Some(value) => value == "true",
        None => false,
    }

    Config { port, debug }
}
```

---

## Design Principles

### 1. Explicit Error Handling

All stdlib functions return error unions:

```typescript
fn read_file(path: string): !string  // NOT: string
fn parse<T>(s: string): !T          // NOT: T
```

### 2. Resource Cleanup

Resources are returned and caller must cleanup:

```typescript
fn open_file(path: string): !File {
    // ...
}

fn main(): !void {
    let file: File = try open_file("data.txt")
    defer file.close()  // Caller cleanup
    // ...
}
```

### 3. Async by Default

I/O operations are async:

```typescript
async fn read_file(path: string): !string {
    // ...
}
```

---

## Planned Modules (Future)

Phase 6+ additions:

- `std/crypto` - Cryptography functions
- `std/compress` - Compression (gzip, zlib)
- `std/json` - JSON encoding/decoding
- `std/xml` - XML parsing
- `std/database` - Database connections
- `std/testing` - Testing utilities

---

## Comparison with Other Stdlibs

| Feature         | Rust             | Go             | Koa            |
|-----------------|------------------|----------------|----------------|
| **Collections** | ✅ Vec, HashMap   | ✅ slice, map   | ✅ Vec, HashMap |
| **I/O**         | ✅ std::io        | ✅ io package   | ✅ std::io      |
| **Net**         | ✅ std::net       | ✅ net/http     | ✅ std::net     |
| **Async**       | Tokio (external) | goroutines     | ✅ Built-in     |
| **Time**        | ✅ std::time      | ✅ time package | ✅ std::time    |
| **Crypto**      | External         | ✅ crypto       | Planned        |

---

## Next Steps

- [Implementation Plan](10-implementation-plan.md) - Roadmap
- [Syntax Guide](02-syntax-guide.md) - Back to syntax
