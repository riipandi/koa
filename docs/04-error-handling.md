# Error Handling

Koa uses **Zig-style error handling**: error sets and error unions. No exceptions.

## Philosophy

- **Errors are values** - Errors are values just like data
- **Explicit propagation** - Use `try` to propagate errors
- **Compile-time checked** - Compiler ensures errors are handled
- **No exceptions** - No surprise exceptions at runtime

---

## Error Sets

Error sets are enums for errors:

```
const FileError = error {
    NotFound,
    AccessDenied,
    OutOfMemory,
}
```

### Error Set Merging

```
const ReadError = error {
    InvalidData,
    Corrupted,
}

const FileError = error {
    NotFound,
    AccessDenied,
}

// Error union combines error sets
fn read_file(): FileError!string {
    // ...
}

fn parse(): (FileError | ReadError)!Data {
    // ...
}
```

---

## Error Union Type

Syntax `Error!T` means "Error or T":

```
fn divide(a: f64, b: f64): !f64 {
    // ^ Means anyerror!f64 (error union with any error set)
    if b == 0.0 {
        return error.DivideByZero
    }
    a / b
}

fn read_file(path: string): FileError!string {
    // ^ Means FileError | string
    if !exists(path) {
        return error.NotFound
    }
    // ...
}
```

---

## The `try` Keyword

`try` unwraps error union or returns error:

```
fn process(): !void {
    // If read_file errors, error is propagated
    let data: string = try read_file("data.txt")
    println!("{}", data)
}
```

Equivalent to:

```
fn process(): !void {
    let data: string = read_file("data.txt") catch |err| {
        return err
    }
    println!("{}", data)
}
```

---

## The `catch` Keyword

`catch` handle errors:

```
fn main(): i32 {
    let result: string = read_file("data.txt") catch |err| {
        println!("Error: {}", err)
        return 1
    }
    println!("{}", result)
    return 0;
}
```

### Default Values

```
fn get_config(): Config | null {
    read_config() catch {
        return null  // Return null on error
    }
}
```

---

## Error Return Traces

Koa stores error return trace for debugging:

```
error: NotFound
  /path/to/file.koa:10:5: main: error returned
  /path/to/file.koa:25:10: process: error returned
  /path/to/file.koa:40:15: read_file: error returned
```

---

## errdefer

`errdefer` only runs on error:

```
fn process_file(path: string): !void {
    let file: File = try File::open(path)
    errdefer file.close()  // Only close on error

    let data: string = try file.read_to_string()
    try parse_data(data)

    // On success, file.close() is NOT called here
    // Caller must close
}
```

---

## Practical Examples

### 1. File Operations

```
const FileError = error {
    NotFound,
    AccessDenied,
    Corrupted,
}

fn read_config(path: string): FileError!Config {
    let file: File = try File::open(path)
    defer file.close()

    let content: string = try file.read_to_string()
    parse_config(content)
}

fn main(): i32 {
    match read_config("config.toml") {
        Ok(config) => {
            println!("Loaded config")
    return 0;
        },
        Err(error.NotFound) => {
            println!("Config file not found")
    return 1;
        },
        Err(error.Corrupted) => {
            println!("Config file corrupted")
    return 2;
        },
        Err(err) => {
            println!("Error: {}", err)
    return 3;
        },
    }
}
```

### 2. HTTP Request

```
const HttpError = error {
    InvalidUrl,
    ConnectionFailed,
    Timeout,
}

async fn fetch(url: string): HttpError!string {
    let parsed: Url = try parse_url(url)
    let response: HttpResponse = await http_get(parsed) catch {
        return error.ConnectionFailed
    }
    response.body
}
```

### 3. Validation

```
const ValidationError = error {
    InvalidEmail,
    InvalidPhone,
    MissingField,
}

fn validate_user(user: User): ValidationError!void {
    if user.email == "" {
        return error.MissingField
    }
    if !is_valid_email(user.email) {
        return error.InvalidEmail
    }
    if user.phone != null && !is_valid_phone(user.phone) {
        return error.InvalidPhone
    }
}
```

---

## Error Handling Patterns

### Pattern 1: Propagate with `try`

```
fn process(): !void {
    let data: string = try read_file("data.txt")
    let parsed: Data = try parse(data)
    try save(parsed)
}
```

### Pattern 2: Handle with `match`

```
fn main(): i32 {
    match process() {
        Ok(()) => 0,
        Err(err) => {
            println!("Error: {}", err)
    return 1;
        },
    }
}
```

### Pattern 3: Provide Default

```
fn get_port(): i32 {
    get_env("PORT") catch |err| {
        8080  // Default port
    }
}
```

### Pattern 4: Wrap Error

```
fn load_config(): !Config {
    let content: string = read_file("config.toml") catch |err| {
        return error.ConfigFailed  // Wrap error
    }
    parse(content)
}
```

---

## Comparison with Other Languages

### TypeScript

```
// TypeScript: try/catch
try {
    const data = readFile(path)
} catch (err) {
    handleError(err)
}

// Koa: Error union
let data: string = read_file(path) catch |err| {
    handle_error(err)
    return
}
```

### Rust

```rust
// Rust: Result<T, E>
fn read_file() -> Result<String, io::Error> {
    Ok(content)
}

let data = read_file()?;  // ? operator

// Koa: Error union
fn read_file(): !string {
    return content;
}

let data = try read_file()  // try keyword
```

### Go

```go
// Go: Multiple return values
data, err := readFile(path)
if err != nil {
    return err
}

// Koa: Error union
let data: string = try read_file(path)
```

---

## Best Practices

### 1. Always Handle Errors

```
// BAD: Ignore error
let data: string = read_file(path) catch { }
// ^ Error discarded

// GOOD: Handle error
let data: string = read_file(path) catch |err| {
    println!("Error: {}", err)
    return
}
```

### 2. Use Descriptive Error Names

```
// BAD
const E = error {
    A,
    B,
}

// GOOD
const FileError = error {
    NotFound,
    AccessDenied,
}
```

### 3. Defer Cleanup

```
fn process(): !void {
    let file: File = try File::open(path)
    defer file.close()  // Always cleanup

    try process_file(file)
}
```

### 4. Use errdefer for Conditional Cleanup

```
fn process(): !void {
    let file: File = try File::open(path)
    errdefer file.close()  // Only cleanup on error

    try validate(file)
    // If successful, file remains open

    return file
}
```

---

## Summary

| Concept         | Syntax                       | Description           |
|-----------------|------------------------------|-----------------------|
| **Error Set**   | `error { A, B }`             | Enum of errors        |
| **Error Union** | `E!T`                        | E or T                |
| **try**         | `try expr`                   | Unwrap or propagate   |
| **catch**       | `expr catch \|err\| handler` | Handle error          |
| **errdefer**    | `errdefer stmt`              | Cleanup on error only |

---

## Next Steps

- [Type System](03-type-system.md) - Type system and generics
- [Syntax Guide](02-syntax-guide.md) - Back to syntax
