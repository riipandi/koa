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
        shade(value);  // Mark gray
    }
    *slot = value;
}
```

The compiler automatically inserts write barriers for pointer stores.

---

## GC Configuration

### Overview

Koa provides **multiple ways** to configure garbage collection:

1. **Environment variables** - For deployment and operations
2. **CLI flags** - For one-off builds
3. **Programmatic API** - For runtime control

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `KOA_GC_PERCENT` | Heap growth trigger (%) | 100 |
| `KOA_GC_LIMIT` | Soft memory limit | None |
| `KOA_GC_DEBUG` | Enable GC logging | false |

**Examples:**

```bash
# Default: GC triggers when heap grows 100%
export KOA_GC_PERCENT=100

# Less frequent GC (200% heap growth)
export KOA_GC_PERCENT=200

# Disable GC (not recommended)
export KOA_GC_PERCENT=off

# Set memory limit
export KOA_GC_LIMIT=1GB

# Enable GC logging
export KOA_GC_DEBUG=1
```

### CLI Flags

```bash
# Set GC percentage
koa run --gc-percent=150

# Set memory limit
koa run --gc-limit=2GB

# Enable debug output
koa run --gc-debug
```

### Programmatic API

The `std/runtime/gc` module provides runtime control:

```typescript
import { gc } from "std/runtime";

fn main(): i32 {
    // Force GC run immediately
    gc::collect();

    // Get GC stats
    let stats: GCStats = gc::stats();
    println!("Heap size: {} bytes", stats.heap_size);
    println!("GC count: {}", stats.gc_count);

    // Set GC percentage
    gc::set_percent(150);

    // Set memory limit
    gc::set_limit("2GB");

    return 0;
}
```

### GC Percentage

The `gc_percent` parameter controls when GC triggers:

| Value | Description |
|-------|-------------|
| `off` | Disable GC (not recommended) |
| `50` | GC at 50% heap growth (more frequent) |
| `100` | GC at 100% heap growth (default) |
| `200` | GC at 200% heap growth (less frequent) |
| `400` | GC at 400% heap growth (minimal GC) |

**Formula:**

```
GC triggers when: live_heap > last_heap_size * (gc_percent / 100)
```

### Memory Limit

The `gc_limit` parameter sets a soft memory limit:

- **Soft limit** - GC tries to stay under limit, but can exceed
- **Hard limit** - Future feature: panic when exceeded

**Example:**

```bash
# Keep memory under 2GB
export KOA_GC_LIMIT=2GB
```

### GC Statistics

The `gc::stats()` function returns:

```typescript
struct GCStats {
    heap_size: u64,        // Current heap size (bytes)
    gc_count: u64,         // Number of GC runs
    last_gc_duration: u64, // Last GC duration (nanoseconds)
    pause_avg: u64,        // Average pause time (nanoseconds)
}
```

---

## GC Tuning Guide

### High-Throughput Applications

**Goal:** Maximize throughput, tolerate latency

```bash
export KOA_GC_PERCENT=200  # Less frequent GC
export KOA_GC_LIMIT=4GB    # Larger heap
```

### Low-Latency Applications

**Goal:** Minimize GC pauses

```bash
export KOA_GC_PERCENT=50   # More frequent GC
export KOA_GC_LIMIT=512MB  # Smaller heap
```

### Memory-Constrained Applications

**Goal:** Minimize memory usage

```bash
export KOA_GC_PERCENT=100  # Default
export KOA_GC_LIMIT=256MB  # Strict limit
```

---

## Removed Features

The following Go-style features are **NOT** supported in Koa:

- ❌ `GOGC` environment variable (use `KOA_GC_PERCENT` instead)
- ❌ `debug.SetGCPercent()` function (use `gc::set_percent()` instead)
- ❌ `debug.ReadGCStats()` function (use `gc::stats()` instead)
- ❌ `runtime.SetMemoryLimit()` function (use `gc::set_limit()` instead)

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
