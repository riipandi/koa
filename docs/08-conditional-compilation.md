# Conditional Compilation

Koa supports conditional compilation with simple annotation syntax: `[@annotation]`.

## Philosophy

- **Simple syntax** - `[@debug]`, `[@release]`, etc
- **Auto-detect target** - No manual configuration needed
- **Explicit** - Clear what is compiled
- **Build-time only** - No runtime overhead

---

## Build Modes

Koa supports 4 build modes:

1. **debug** - Development, safety checks ON, optimization OFF
2. **release-safe** - Production with safety checks
3. **release-fast** - Maximum performance, safety checks OFF
4. **release-small** - Minimum binary size

---

## Annotations

### Build Mode Annotations

```typescript
[@debug]
fn log_debug(msg: string): void {
    println!("DEBUG: {}", msg)
}

[@release]
fn optimized_path(): void {
    // Release-only optimizations
}
```

### Platform Annotations

```typescript
[@os_linux]
fn linux_specific(): void {
    // Linux only
}

[@os_windows]
fn windows_specific(): void {
    // Windows only
}

[@os_macos]
fn macos_specific(): void {
    // macOS only
}
```

### Architecture Annotations

```typescript
[@arch_x86_64]
fn x86_64_optimized(): void {
    // x86_64 specific
}

[@arch_aarch64]
fn arm_optimized(): void {
    // ARM64 specific
}
```

### Feature Annotations

```typescript
[@feature_sqlite]
fn with_sqlite(): void {
    // Only if --feature sqlite
}
```

### Test Annotations

```typescript
[@test]
fn test_helper(): void {
    // Only in test builds
}
```

---

## Negation

```typescript
[@not_debug]
fn release_only(): void {
    // Anything except debug
}

[@not_os_windows]
fn non_windows(): void {
    // Any OS except Windows
}
```

---

## Combinations

```typescript
[@debug @os_linux]
fn linux_debug(): void {
    // Debug + Linux
}

[@release_fast @arch_x86_64]
fn x86_release(): void {
    // Release-fast + x86_64
}
```

---

## Practical Examples

### 1. Debug Logging

```typescript
struct Logger {
    [@debug]
    enabled: bool,

    [@debug]
    pub fn new(): Logger {
        Logger { enabled: true }
    }

    [@debug]
    pub fn log(self: Logger, msg: string): void {
        if self.enabled {
            println!("LOG: {}", msg)
        }
    }

    [@not_debug]
    pub fn new(): Logger {
        Logger {}  // Empty in release
    }

    [@not_debug]
    pub fn log(self: Logger, msg: string): void {
        // No-op, optimized away
    }
}
```

### 2. Platform-Specific Code

```typescript
fn get_temp_dir(): string {
    [@os_windows]
    return "C:\\Temp"

    [@os_linux]
    return "/tmp"

    [@os_macos]
    return "/tmp"
}
```

### 3. Performance Measurements

```typescript
[@debug]
fn measure_time<T>(name: string, f: fn(): T): T {
    let start: i64 = timestamp()
    let result: T = f()
    let elapsed: i64 = timestamp() - start
    println!("{} took {} ms", name, elapsed)
    result
}

[@not_debug]
fn measure_time<T>(name: string, f: fn(): T): T {
    f()  // Direct call, no overhead
}
```

### 4. Safety vs Performance

```typescript
fn array_get<T>(arr: []T, index: usize): T | null {
    [@debug]
    if index >= arr.len {
        println!("Bounds check failed")
        return null
    }

    arr[index]
}
```

---

## Build System

### Command-Line Interface

```bash
# Build modes
koa build                    # Debug (default)
koa build --mode debug       # Explicit debug
koa build --mode release-safe
koa build --mode release-fast
koa build --mode release-small

# Features
koa build --feature sqlite
koa build --feature sqlite,postgres,redis

# Target (auto-detect by default)
koa build --target x86_64-linux-gnu
koa build --target aarch64-windows-msvc
koa build                    # Auto-detect
```

### Target Auto-Detection

```bash
# Auto-detect based on host
koa build

# On Linux x86_64 → target: x86_64-linux-gnu
# On Windows ARM64 → target: aarch64-windows-msvc
```

---

## Annotation Reference

### Build Modes

| Annotation         | Description           |
|--------------------|-----------------------|
| `[@debug]`         | Debug mode only       |
| `[@release_safe]`  | Release with safety   |
| `[@release_fast]`  | Release max speed     |
| `[@release_small]` | Release min size      |
| `[@release]`       | Any release mode      |

### Negation

| Annotation          | Description           |
|---------------------|-----------------------|
| `[@not_debug]`      | Not debug mode        |
| `[@not_release]`    | Debug mode only       |
| `[@not_os_windows]` | Any OS except Windows |

### Platform

| Annotation      | Description  |
|-----------------|--------------|
| `[@os_linux]`   | Linux only   |
| `[@os_windows]` | Windows only |
| `[@os_macos]`   | macOS only   |

### Architecture

| Annotation        | Description |
|-------------------|-------------|
| `[@arch_x86_64]`  | 64-bit x86  |
| `[@arch_aarch64]` | 64-bit ARM  |
| `[@arch_arm]`     | 32-bit ARM  |

### Features

| Annotation        | Description       |
|-------------------|-------------------|
| `[@feature_name]` | If --feature name |

### Build Type

| Annotation | Description          |
|------------|----------------------|
| `[@test]`  | Test build           |
| `[@bench]` | Benchmark build      |
| `[@main]`  | Main build (default) |

---

## Best Practices

### 1. Use Debug Logging Sparingly

```typescript
// GOOD: Conditional debug logging
[@debug]
fn log_debug(msg: string): void {
    println!("{}", msg)
}

// BAD: Always logging (even in release)
fn log(msg: string): void {
    println!("{}", msg)
}
```

### 2. Platform Abstractions

```typescript
// GOOD: Platform-specific functions
fn get_config_path(): string {
    [@os_windows]
    return "C:\\Program Data"

    [@os_linux]
    return "/etc/config"
}

// BAD: Runtime checks (slower)
fn get_config_path(): string {
    if os == "windows" {
        return "C:\\Program Data"
    } else {
        return "/etc/config"
    }
}
```

### 3. Feature Flags

```typescript
// GOOD: Optional features
[@feature_sqlite]
struct Database {
    conn: *SqliteConn,
}

[@feature_postgres]
struct Database {
    conn: *PostgresConn,
}
```

---

## Comparison with Rust

| Rust                            | Koa                 | Description  |
|---------------------------------|---------------------|--------------|
| `#[cfg(debug_assertions)]`      | `[@debug]`          | Debug mode   |
| `#[cfg(not(debug_assertions))]` | `[@not_debug]`      | Release mode |
| `#[cfg(target_os = "linux")]`   | `[@os_linux]`       | Linux only   |
| `#[cfg(feature = "sqlite")]`    | `[@feature_sqlite]` | Feature flag |
| `#[cfg(test)]`                  | `[@test]`           | Test build   |

**Koa advantages:**
- ✅ 50% shorter: `[@debug]` vs `#[cfg(debug_assertions)]`
- ✅ No `cfg()` wrapper
- ✅ No quotes for simple cases
- ✅ Easier to type

---

## Next Steps

- [Standard Library](09-standard-library.md) - Standard library plan
- [Implementation Plan](10-implementation-plan.md) - Roadmap
