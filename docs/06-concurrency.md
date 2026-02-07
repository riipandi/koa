# Concurrency

Koa uses **async/await** model (TypeScript-style) for single-threaded concurrent programming.

## Philosophy

- **Simple mental model** - Single-threaded, no race conditions
- **Cooperative multitasking** - Tasks yield explicitly
- **Event loop** - Async runtime with event loop
- **Low Overhead** - Green threads, not OS threads
- **Runtime Safety** - Race detection via tooling (ThreadSanitizer)

## Memory Safety

Koa adopts a **Runtime Safety** model for concurrency (similar to Go):

1.  **Heap Safety**: Automatic (GC).
2.  **Data Races**: Detected at runtime using the Race Detector.
    - Run tests with `koa test --race`.
    - Compiler does not have a Borrow Checker (by design).

### Essential Primitives (`std/sync`)

To share state safely, use standard synchronization primitives:

- `Mutex<T>`: Mutual exclusion.
- `RwLock<T>`: Read-write lock.
- `WaitGroup`: Wait for tasks to complete.
- `AtomicI32`, `AtomicBool`: Lock-free operations.
- `Channel<T>`: Message passing (Preferred).

---

## Async Functions

### Declaring Async Functions

```
async fn fetch_data(url: string): !Data {
    let response: HttpResponse = await http_get(url)
    response.data
}
```

### Calling Async Functions

```
async fn main(): !void {
    let data: Data = await fetch_data("https://api.example.com")
    println!("{}", data)
}
```

---

## Async Runtime

### Event Loop

Single-threaded event loop for async tasks:

```
// Event loop processes tasks sequentially
fn event_loop(): void {
    loop {
        select {
            task = next_task() => execute(task),
            timeout = next_timer() => handle_timeout(timeout),
            io_event = wait_for_io() => handle_io(io_event),
        }
    }
}
```

### Task Scheduling

Tasks are scheduled in the event loop:

```
fn spawn_async<T>(future: Future<T>): void {
    runtime.schedule(future)
}
```

---

## Async I/O

Non-blocking I/O operations:

```
async fn read_file(path: string): !string {
    let file: File = await File::open_async(path)
    let content: string = await file.read_to_string_async()
    return content;
}
```

---

## Practical Examples

### 1. HTTP Server

```
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
    server.run(handle_request).await;
}
```

### 2. Concurrent Requests

```
async fn fetch_multiple(urls: []string): !Vec<string> {
    let results: Vec<string> = Vec::new()

    for url in urls {
        let data: string = await http_get(url)
        try results.push(data)
    }

    return results;
}
```

### 3. Timeout

```
async fn with_timeout<T>(
    future: Future<T>,
    timeout_ms: i32
): T | null {
    let result: T | TimeoutError = select {
        res = await future => res,
        _ = await sleep(timeout_ms) => null,
    }
    return result;
}
```

---

## No Goroutines (Initial Version)

Koa **doesn't** have goroutines in Phase 1. Only async/await.

Future considerations (Phase 5+):
- Goroutines + channels (Go-style)
- Or remain single-threaded async

---

## Comparison with Other Languages

### TypeScript

```
// TypeScript
async function fetch(): Promise<Data> {
    const response = await httpGet(url)
    return response.data
}

// Koa
async fn fetch(): !Data {
    let response: HttpResponse = await http_get(url)
    response.data
}
```

### Rust

```rust
// Rust: Async with futures
async fn fetch() -> Result<Data, Error> {
    let response = http_get(url).await?
    Ok(response.data)
}

// Koa
async fn fetch(): !Data {
    let response: HttpResponse = await http_get(url)
    response.data
}
```

### Go

```go
// Go: Goroutines
go func() {
    data := fetch(url)
    process(data)
}()

// Koa: Async (future)
spawn_async(async || {
    let data: Data = await fetch(url)
    process(data)
})
```

---

## Best Practices

### 1. Always Mark Async Functions

```
// BAD: Lupa async
fn fetch(): Data {
    await http_get(url)  // ERROR: await outside async
}

// GOOD: Async
async fn fetch(): Data {
    await http_get(url)
}
```

### 2. Handle Errors in Async

```
async fn process(): !void {
    let data: Data = await fetch() catch {
        return error.FetchFailed
    }
    // ...
}
```

### 3. Avoid Blocking in Async

```
// BAD: Blocking call
async fn fetch(): Data {
    let data: string = read_file_blocking(path)  // Blocks event loop!
    parse(data)
}

// GOOD: Async call
async fn fetch(): Data {
    let data: string = await read_file_async(path)
    parse(data)
}
```

---

## Future: Goroutines + Channels (Phase 5)

Potential addition in Phase 5:

```
// Goroutine
go worker(1, channel)

// Channel
let (tx, rx): (Chan<i32>, Chan<i32>) = chan(i32, 0)

// Send
tx.send(42)

// Receive
let value: i32 = rx.recv()
```

---

## Summary

| Concept | Syntax | Description |
|---------|--------|-------------|
| **Async Function** | `async fn(): T` | Function that returns Future |
| **Await** | `await expr` | Yield sampai Future ready |
| **Event Loop** | Runtime | Single-threaded scheduler |
| **Spawn** | `spawn_async(future)` | Schedule async task |

---

## Next Steps

- [Module System](07-modules.md) - Modules and imports
- [Memory Management](05-memory-management.md) - GC and allocation
