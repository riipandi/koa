# Hot Module Reload (HMR)

Koa supports **Hot Module Reload** for fast development iteration with automatic code reloading during development.

## Philosophy

- **Fast iteration** - No need to restart after every change
- **State preservation** - Maintain application state during reload
- **Incremental compilation** - Only recompile changed modules
- **Error recovery** - Graceful handling of compilation errors

## Usage

### Basic HMR

```bash
# Start HMR server
koa watch

# Or with hot reload
koa watch --hot
```

### File Watching

The compiler watches for file changes and automatically recompiles:

```typescript
// Watching for changes...
// ✓ main.koa changed
//   Compiling... (0.3s)
//   Hot reloading... ✓
// ✓ State preserved
```

### Manual Reload

```bash
# Trigger manual reload
koa reload
```

## Features

### 1. Incremental Compilation

Only changed modules are recompiled:

```typescript
// main.koa changed
// Only main.koa is recompiled
// Dependencies (utils.koa, etc.) remain cached
```

### 2. State Preservation

Application state is preserved across reloads:

```typescript
// Before reload
let counter: i32 = 42;

// After code change and reload
// counter is still 42, not reset
```

### 3. Error Recovery

Compilation errors don't crash the running application:

```typescript
// main.koa has syntax error
// ✗ main.koa:3: unexpected token
// Application continues running with previous version
```

### 4. Debouncing

Rapid file changes are debounced to avoid excessive recompilation:

```typescript
// Saving multiple times quickly
// main.koa changed (debouncing...)
// main.koa changed (debouncing...)
// ✓ main.koa changed
//   Compiling...
```

## Module Tracking

The compiler tracks module dependencies:

```
main.koa
├── imports utils.koa
├── imports collections/vec.koa
└── imports driver/sqlite

// When main.koa changes:
// - Only main.koa is recompiled
// - Cached modules remain untouched
```

## Configuration

### Koa.toml

```toml
[package]
name = "myapp"

[watch]
# Enable HMR
enabled = true

# Paths to watch
paths = ["src", "examples"]

# Ignore patterns
ignore = ["*.tmp", "*.bak"]

# Debounce delay (ms)
debounce = 300

# Hot reload (preserve state)
hot = true
```

## CLI Commands

```bash
# Watch and hot reload
koa watch --hot

# Watch without hot reload (manual reload)
koa watch

# Watch with custom paths
koa watch --path src --path examples

# Watch with filter
koa watch --filter "*.koa"
```

## Implementation Details

### Compilation Cache

Compiled modules are cached:

```
.koa/cache/
├── main.koa.o
├── utils.koa.o
└── collections/
    └── vec.koa.o
```

### Dependency Graph

The compiler builds a dependency graph:

```
main → utils → collections
      ↓
      driver/sqlite
```

When a module changes, only it and modules that depend on it are recompiled.

### State Serialization

Application state is serialized before reload:

```typescript
// Before reload
let app = App { state: 42 };

// State is serialized
// serialize(app) → JSON

// After reload
// deserialize(JSON) → app
```

## Limitations

### Not Supported

- Function signature changes (require full restart)
- Struct layout changes (require full restart)
- Module additions/removals (require full restart)
- Global variable changes (reset to default)

### Supported

- Function body changes
- Method implementations
- Control flow logic
- Expression changes

## Best Practices

### 1. Preserve State

```typescript
// Use structures that can be serialized
pub struct AppState {
    counter: i32,
    users: Vec<User>,
}

// Avoid:
static mut CACHED_VALUE: i32 = 0;  // Reset on reload
```

### 2. Stable Interfaces

```typescript
// Keep function signatures stable
pub fn process(data: Data): Result {
    // Implementation can change
    // Signature must stay the same
}
```

### 3. Error Handling

```typescript
// Handle compilation errors gracefully
// Continue with previous version on error
fn main(): i32 {
    match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error: {}", err);
            1;
        },
    };
}
```

## Comparison

| Feature            | Go  | Rust | Koa            |
|--------------------|-----|------|----------------|
| **HMR**            | No  | No   | ✅ Built-in     |
| **Incremental**    | No  | No   | ✅ Module-level |
| **State Preserve** | N/A | N/A  | ✅ Yes          |
| **Error Recovery** | N/A | N/A  | ✅ Yes          |

## Next Steps

- [Implementation Plan](10-implementation-plan.md) - Phase 8: HMR
- [Syntax Guide](02-syntax-guide.md) - Back to syntax

---

HMR enables rapid development with instant feedback! 🚀
