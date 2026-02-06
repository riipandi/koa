# Build System - Koa.toml Specification

The `Koa.toml` file is the manifest for a Koa project, containing metadata, dependencies, and build configuration.

---

## File Structure

The `Koa.toml` file uses TOML format and is placed in the project root.

```toml
[package]
name = "myapp"
version = "0.1.0"
type = "executable"

[build]
target = "x86_64-unknown-linux-gnu"
mode = "debug"

[dependencies]
http = { git = "https://github.com/riipandi/koa-http", version = "0.1.0" }
```

---

## Sections

### [package]

Package metadata (required).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Package name (snake_case) |
| `version` | string | Yes | Semantic version (MAJOR.MINOR.PATCH) |
| `type` | string | No | "executable" (default) or "library" |
| `authors` | array | No | List of authors |
| `description` | string | No | Short description |
| `license` | string | No | SPDX license identifier |
| `repository` | string | No | Git repository URL |

**Example:**

```toml
[package]
name = "myapp"
version = "0.1.0"
type = "executable"
authors = ["Your Name <you@example.com>"]
description = "My awesome Koa application"
license = "MIT"
repository = "https://github.com/user/myapp"
```

---

### [build]

Build configuration (optional).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `target` | string | host | Target triple (e.g., `x86_64-unknown-linux-gnu`) |
| `mode` | string | "debug" | Build mode: "debug", "release", "release-fast", "release-small" |
| `opt-level` | number | Varies | Optimization level (0-3, "s" for size, "z" for size-opt) |
| `lto` | boolean | false | Link-time optimization |
| `strip` | boolean | false | Strip symbols from binary |
| `debug-symbols` | boolean | true in debug | Include debug symbols |

**Build Modes:**

| Mode | Opt Level | LTO | Strip | Debug Symbols | Description |
|------|-----------|-----|-------|---------------|-------------|
| `debug` | 0 | false | false | true | Fast compilation, no optimization |
| `release` | 2 | false | false | false | Balanced |
| `release-fast` | 3 | true | false | false | Maximum performance |
| `release-small` | "s" | true | true | false | Minimum size |

**Example:**

```toml
[build]
target = "x86_64-unknown-linux-gnu"
mode = "release-fast"
opt-level = 3
lto = true
strip = false
debug-symbols = false
```

---

### [features]

Conditional compilation flags (optional).

Features can be used to enable optional functionality:

```toml
[features]
default = ["database", "redis"]
database = []
redis = []
test-utils = []
```

**Usage in code:**

```
[@feature("database")]
pub fn connect(): !Connection {
    // ...
}

[@feature("test-utils")]
pub fn test_helper(): void {
    // ...
}
```

**Command line:**

```bash
# Enable default features
koa build

# Enable specific features
koa build --features "database,redis"

# Enable all features
koa build --features-all

# Disable default features
koa build --no-default-features
```

---

### [[bin]]

Multiple binary targets (optional, for executable packages).

**Example:**

```toml
[[bin]]
name = "myapp"
path = "src/main.koa"

[[bin]]
name = "mytool"
path = "src/tool.koa"
```

---

### [[lib]]

Library configuration (optional, for library packages).

**Example:**

```toml
[lib]
name = "mylib"
path = "src/lib.koa"
```

---

### [dependencies]

External dependencies (optional).

#### Git Dependency

```toml
[dependencies]
http = { git = "https://github.com/riipandi/koa-http", version = "0.1.0" }
```

**Fields:**
- `git`: Git repository URL (required)
- `version`: Semantic version constraint (required)
- `branch`: Branch name (optional, defaults to "main")
- `tag`: Tag name (optional, overrides version)

#### Local Path Dependency

```toml
[dependencies]
utils = { path = "./utils" }
```

#### With Features

```toml
[dependencies]
auth = { git = "https://github.com/user/koa-auth", features = ["oauth2"] }
```

#### Stdlib (Always Available)

```toml
[dependencies]
std = { version = "0.1.0" }
```

The standard library is always available but version-locked to the compiler.

---

### [dev-dependencies]

Development-only dependencies (optional).

These are only used when running tests or benchmarks:

```toml
[dev-dependencies]
testutil = { path = "./testutil" }
benchmark = { git = "https://github.com/user/koa-benchmark" }
```

---

### [workspace]

Workspace configuration (optional, for multi-crate projects).

```toml
[workspace]
members = ["utils", "auth"]
```

This creates a workspace with member crates in the specified directories.

---

## Complete Example

```toml
[package]
name = "webapp"
version = "0.1.0"
type = "executable"
authors = ["Jane Doe <jane@example.com>"]
description = "A web application built with Koa"
license = "MIT"
repository = "https://github.com/janedoe/webapp"

[build]
target = "x86_64-unknown-linux-gnu"
mode = "release"
opt-level = 2
lto = false
strip = false

[features]
default = ["database", "redis"]
database = []
redis = []
tls = []

[[bin]]
name = "webapp"
path = "src/main.koa"

[[bin]]
name = "migrate"
path = "src/migrate.koa"

[dependencies]
http = { git = "https://github.com/riipandi/koa-http", version = "0.1.0" }
json = { git = "https://github.com/riipandi/koa-json", version = "0.1.0" }

[@feature("database")]
database = { git = "https://github.com/riipandi/koa-sqlite", version = "0.1.0" }

[@feature("redis")]
redis = { git = "https://github.com/riipandi/koa-redis", version = "0.1.0" }

[dev-dependencies]
testutil = { path = "./testutil" }

[workspace]
members = ["utils", "auth"]
```

---

## Environment Variables

Build configuration can also be overridden via environment variables:

| Variable | Description |
|----------|-------------|
| `KOA_BUILD_TARGET` | Override target triple |
| `KOA_BUILD_MODE` | Override build mode |
| `KOA_BUILD_OPT_LEVEL` | Override optimization level |
| `KOA_FEATURES` | Comma-separated features to enable |

**Example:**

```bash
export KOA_BUILD_MODE=release-fast
export KOA_FEATURES="database,redis"
koa build
```

---

## Validation

The `koa` command validates `Koa.toml` before building:

```bash
# Validate Koa.toml
koa validate

# Output:
# ✓ Valid Koa.toml
#   - Package: myapp 0.1.0
#   - Dependencies: 2
#   - Features: 3
```

---

## Migration from Earlier Versions

If you need to migrate `Koa.toml` from an earlier version:

```bash
# Automatically migrate
koa migrate

# Show what would change
koa migrate --dry-run
```

---

## See Also

- [Lockfile Specification](17-lockfile-spec.md) - `Koa.lock` format
- [Package Manager](11-package-manager.md) - Dependency management
- [Build Cache](18-build-cache.md) - Incremental compilation
