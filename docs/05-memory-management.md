# Memory Management

Koa has **Concurrent Mark-Sweep Garbage Collector** (Go-style) for automatic memory management.

## Philosophy

- **No manual free** - No need to deallocate memory
- **Concurrent** - GC runs parallel with application
- **Low latency** - Tri-color marking for minimal pause times
- **Safe** - No dangling pointers, use-after-free, double free

---

## Overview

Concurrent Mark-Sweep GC with tri-color marking:

- **White** - Unreachable (candidate for collection)
- **Gray** - Live object, children not yet scanned
- **Black** - Live object, children already scanned

### GC Phases

1. **Mark Setup** (STW - microseconds)
   - Stop world
   - Mark stack roots as gray
   - Start mark workers

2. **Concurrent Marking** (with application running)
   - Background workers scan gray objects
   - Turn gray → black
   - Write barriers maintain tri-color invariant

3. **Mark Termination** (STW - short)
   - Re-scan roots
   - Finish remaining gray objects

4. **Sweeping** (Concurrent)
   - Free white objects
   - Return memory to allocator

---

## Allocation

### Heap Allocation

```typescript
fn create_node(value: i32): *Node {
    let node: *Node = alloc(Node)  // Heap allocation
    node.value = value
    node.left = null
    node.right = null
    node
}
```

### Stack Allocation

```typescript
fn example(): void {
    let x: i32 = 42  // Stack allocation
    let p: Point = Point::new(1.0, 2.0)  // Stack allocation
    // ...
}
```

---

## Write Barriers

Dijkstra write barrier (Go-style):

```typescript
// Write barrier: shade object when storing pointer
fn write_barrier<T>(slot: *T, value: T): void {
    if GC.is_marking() && is_white(value) {
        shade(value)  // Mark gray
    }
    *slot = value
}
```

Compiler otomatis insert write barriers untuk pointer stores.

---

## GC Configuration

### GOGC Parameter

Tunable GC trigger (like Go):

```typescript
// GOGC=100: default, GC triggers when heap grows 100%
// GOGC=off: disable GC
// GOGC=200: less frequent GC
```

### Soft Memory Limit

```bash
# Set soft memory limit (Go 1.19+)
koa build --gc-limit=512MB
```

---

## Best Practices

### 1. Minimize Allocations

```typescript
// BAD: Allocate in loop
for i in 0..1000 {
    let temp: Vec<i32> = Vec::new()  // 1000 allocations
    // ...
}

// GOOD: Reuse allocation
let temp: Vec<i32> = Vec::new()
for i in 0..1000 {
    temp.clear()
    // use temp...
}
```

### 2. Limit Object Lifetime

```typescript
fn process(): void {
    let data: Vec<u8> = Vec::new()
    // ...
    // data eligible for GC after function exit
}
```

### 3. Avoid Large Temporary Allocations

```typescript
// BAD: Large temporary
fn process(): Vec<i32> {
    let large: Vec<i32> = Vec::with_capacity(1000000)
    // ...
    large  // Returned, but possibly immediate discard
}

// GOOD: Reuse buffer
fn process(buffer: *mut Vec<i32>): void {
    buffer.clear()
    // reuse buffer...
}
```

---

## Escape Analysis

Compiler performs escape analysis to determine stack vs heap allocation:

```typescript
fn example(): *Point {
    // ESCAPES: Returned, must be heap
    let p: Point = Point::new(1.0, 2.0)
    &p  // Heap allocation
}

fn example2(): void {
    // NO ESCAPE: Local only, stack allocation
    let p: Point = Point::new(1.0, 2.0)
    println!("{}", p.x)
}
```

---

## Stack Maps

Compiler generates stack maps for precise root scanning:

```typescript
// Stack map entry:
// { fn: "example", offset: 0x10, slots: [8, 16, 24] }

fn example(): void {
    let a: *i32 = alloc(i32)  // Slot at offset 8
    let b: *i32 = alloc(i32)  // Slot at offset 16
    let c: *i32 = alloc(i32)  // Slot at offset 24

    // GC knows these are roots from stack map
}
```

---

## Finalizers

Optional: Finalizers for cleanup (rarely needed):

```typescript
// Future feature
fn register_finalizer<T>(obj: *T, finalizer: fn(*T)): void {
    // ...
}

fn cleanup_file(f: *File): void {
    f.close()
}

fn example(): void {
    let file: *File = alloc(File)
    register_finalizer(file, cleanup_file)
}
```

---

## Comparison

| Aspect         | Koa                   | Go                    | Rust         | Java               |
|----------------|-----------------------|-----------------------|--------------|--------------------|
| **GC Type**    | Concurrent Mark-Sweep | Concurrent Mark-Sweep | N/A (manual) | Generational       |
| **Latency**    | Low (µs pauses)       | Low (µs pauses)       | Zero         | Medium (ms pauses) |
| **Throughput** | High                  | High                  | Very High    | Medium             |
| **Memory**     | Managed               | Managed               | Manual       | Managed            |

---

## Next Steps

- [Concurrency](06-concurrency.md) - Async/await and goroutines
- [Syntax Guide](02-syntax-guide.md) - Back to syntax
