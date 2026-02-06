# Build Cache

Koa uses incremental compilation to speed up builds by only recompiling changed files.

---

## Overview

The build cache stores intermediate compilation artifacts:

```
.koa/cache/
├── build/
│   ├── main.koa.o           # Object files
│   ├── utils.koa.o
│   └── ...
├── deps/
│   └── graph.json           # Dependency graph
├── metadata.json            # Cache metadata
└── hashes/                  # File content hashes
    ├── main.koa
    └── utils.koa
```

---

## How It Works

### 1. Content-Based Hashing

Each source file is hashed based on its **content** (not timestamp):

```bash
# Hash is SHA-256 of file content
sha256sum src/main.koa
```

**Why content hashing?**
- More reliable than timestamps
- Detects changes across different machines
- Works with file systems without precise timestamps (e.g., NFS)

### 2. Dependency Tracking

The compiler tracks:
- Import dependencies
- Type dependencies
- Macro/annotation dependencies

**Example:**

```
// main.koa
import { utils } from "std/utils";  // Depends on std/utils
```

If `std/utils` changes, `main.koa` is recompiled.

### 3. Incremental Compilation

Only changed files and their dependents are recompiled:

```
Initial build (slow):
  main.koa → compile (100ms)
  utils.koa → compile (50ms)
  Total: 150ms

After changing utils.koa:
  utils.koa → compile (50ms)
  main.koa → recompile (100ms) [depends on utils]
  Total: 150ms

After changing unrelated file test.koa:
  test.koa → compile (20ms)
  main.koa → skipped (no dependency)
  utils.koa → skipped (no dependency)
  Total: 20ms
```

---

## Cache Invalidation

The cache is invalidated when:

1. **Source file changes** - Content hash differs
2. **Import changes** - Import graph changes
3. **Build mode changes** - Different optimization level
4. **Target changes** - Different target triple
5. **Compiler version changes** - Compiler upgrade

**Automatic:** Cache invalidation is automatic. No manual management needed.

---

## Commands

### Enable Cache (Default)

```bash
# Cache is enabled by default
koa build
```

### Disable Cache

```bash
# Disable incremental compilation
koa build --no-cache
```

### Clean Cache

```bash
# Clean all cache
koa cache clean

# Clean specific target
koa cache clean --target x86_64-unknown-linux-gnu

# Clean old cache (older than 7 days)
koa cache clean --old 7d
```

### Inspect Cache

```bash
# Show cache statistics
koa cache stats

# Output:
# Cache size: 256 MB
# Object files: 42
# Hit rate: 87%
# Last build: 2.3s (cached) vs 12.5s (full)
```

### Verify Cache

```bash
# Verify cache integrity
koa cache verify

# Output:
# ✓ Cache is valid
#   - All hashes verified
#   - All dependencies tracked
```

---

## Configuration

### Via Koa.toml

```toml
[build.cache]
enabled = true
directory = ".koa/cache"
max_size = "2GB"
max_age = "30d"
```

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable cache |
| `directory` | string | `.koa/cache` | Cache directory |
| `max_size` | string | `"2GB"` | Maximum cache size |
| `max_age` | string | `"30d"` | Maximum cache age |

### Via Environment Variables

| Variable | Description |
|----------|-------------|
| `KOA_CACHE_ENABLED` | Enable/disable cache |
| `KOA_CACHE_DIR` | Override cache directory |
| `KOA_CACHE_MAX_SIZE` | Maximum cache size |

**Example:**

```bash
export KOA_CACHE_ENABLED=false
koa build
```

---

## Performance

### Cache Hit Rate

The cache hit rate depends on:

- **Project size** - Larger projects benefit more
- **Change frequency** - Few changes = higher hit rate
- **Build mode** - Debug mode reuses less than release

**Typical hit rates:**

| Project Type | Hit Rate | Speedup |
|--------------|----------|---------|
| Small (<10 files) | 60-70% | 2-3x |
| Medium (10-50 files) | 75-85% | 4-6x |
| Large (>50 files) | 85-95% | 8-12x |

### Benchmarks

```bash
# Full build (no cache)
$ time koa build --no-cache
real    0m12.543s

# Incremental build (cache)
$ time koa build
real    0m2.312s

# Speedup: 5.4x
```

---

## Internals

### Dependency Graph

The dependency graph is stored in `.koa/cache/deps/graph.json`:

```json
{
  "nodes": [
    {
      "file": "src/main.koa",
      "hash": "sha256:abc123...",
      "imports": ["src/utils.koa", "std/io"],
      "dependents": []
    },
    {
      "file": "src/utils.koa",
      "hash": "sha256:def456...",
      "imports": ["std/collections"],
      "dependents": ["src/main.koa"]
    }
  ]
}
```

### Metadata

Cache metadata is stored in `.koa/cache/metadata.json`:

```json
{
  "version": "1.0.0",
  "compiler_version": "0.1.0",
  "last_build": "2024-02-06T10:30:00Z",
  "build_mode": "debug",
  "target": "x86_64-unknown-linux-gnu"
}
```

---

## Troubleshooting

### Issue: "Cache corruption detected"

**Solution:** Clean cache

```bash
koa cache clean
```

### Issue: "Incremental build slower than full build"

**Possible causes:**
1. Too many cache misses (frequently changing files)
2. High cache overhead (small project)
3. Corrupted cache

**Solutions:**
```bash
# Check cache stats
koa cache stats

# Clean and rebuild
koa cache clean
koa build
```

### Issue: "Stale cache causing bugs"

**Solution:** Disable cache temporarily

```bash
koa build --no-cache
```

---

## Best Practices

1. **Use cache by default** - Enabled automatically
2. **Clean cache before releases** - Ensure clean builds
3. **Monitor cache size** - Prevent disk space issues
4. **Use CI without cache** - Ensure reproducibility
5. **Exclude from version control** - Add `.koa/` to `.gitignore`

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Build

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Cache Koa build
        uses: actions/cache@v2
        with:
          path: .koa/cache
          key: ${{ runner.os }}-koa-${{ hashFiles('**/*.koa') }}
      - name: Build
        run: koa build
```

### GitLab CI

```yaml
build:
  cache:
    paths:
      - .koa/cache
  script:
    - koa build
```

---

## Advanced Usage

### Custom Cache Directory

```bash
# Use shared cache for multiple projects
export KOA_CACHE_DIR=/shared/koa-cache
koa build
```

### Remote Cache (Future)

```bash
# Use remote cache server
koa build --cache-url https://cache.example.com
```

### Distributed Cache (Future)

```bash
# Share cache across team
koa cache sync --team
```

---

## See Also

- [Build System](16-build-system.md) - Koa.toml configuration
- [Package Manager](11-package-manager.md) - Dependency management
