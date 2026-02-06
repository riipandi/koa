# Lockfile Specification - Koa.lock (JSON)

The `Koa.lock` file is a machine-readable lockfile that ensures reproducible builds. It uses JSON format for fast parsing and better tooling support.

---

## File Format

The lockfile is a JSON object with the following structure:

```json
{
  "version": "1.0.0",
  "packages": [
    {
      "name": "http",
      "version": "0.1.0",
      "source": {
        "type": "git",
        "url": "https://github.com/riipandi/koa-http",
        "rev": "a1b2c3d4e5f6...",
        "checksum": "sha256:abc123..."
      }
    }
  ],
  "metadata": {
    "generated_at": "2024-02-06T10:30:00Z",
    "compiler_version": "0.1.0"
  }
}
```

---

## Root Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | string | Yes | Lockfile format version |
| `packages` | array | Yes | List of package dependencies |
| `metadata` | object | Yes | Metadata about the lockfile |

---

## Packages Array

Each package in the `packages` array has the following structure:

### Common Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Package name |
| `version` | string | Yes | Semantic version |
| `source` | object | Yes | Source information |

### Source Object

The `source` object describes where the package comes from.

#### Git Source

```json
{
  "type": "git",
  "url": "https://github.com/riipandi/koa-http",
  "rev": "a1b2c3d4e5f6...",
  "branch": "main",
  "checksum": "sha256:abc123..."
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Must be "git" |
| `url` | string | Yes | Git repository URL |
| `rev` | string | Yes | Git commit hash (SHA-1) |
| `branch` | string | No | Branch name (for reference only) |
| `tag` | string | No | Tag name (for reference only) |
| `checksum` | string | Yes | SHA256 checksum |

#### Path Source

```json
{
  "name": "utils",
  "version": "0.1.0",
  "source": {
    "type": "path",
    "path": "./utils",
    "checksum": "sha256:def456..."
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Must be "path" |
| `path` | string | Yes | Relative or absolute path |
| `checksum` | string | Yes | SHA256 checksum |

#### Registry Source (Future)

```json
{
  "name": "http",
  "version": "0.1.0",
  "source": {
    "type": "registry",
    "url": "https://registry.koa-lang.com",
    "checksum": "sha256:789012..."
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Must be "registry" |
| `url` | string | Yes | Registry URL |
| `checksum` | string | Yes | SHA256 checksum |

---

## Metadata Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generated_at` | string | Yes | ISO 8601 timestamp |
| `compiler_version` | string | Yes | Compiler version that generated the lockfile |

---

## Checksum Format

Checksums use the format `sha256:<hex>` where `<hex>` is the hexadecimal representation of the SHA-256 hash.

**What is checksummed:**
- **Git sources**: SHA-256 of the tarball of the repository at the specified revision
- **Path sources**: SHA-256 of the directory contents (recursive, sorted by filename)

**Example:**

```bash
# Generate checksum for a Git repository
git archive --format=tar HEAD | sha256sum

# Generate checksum for a path
find . -type f | sort | xargs sha256sum | sha256sum
```

---

## Complete Example

```json
{
  "version": "1.0.0",
  "packages": [
    {
      "name": "http",
      "version": "0.1.0",
      "source": {
        "type": "git",
        "url": "https://github.com/riipandi/koa-http",
        "rev": "a1b2c3d4e5f6789012345678901234567890abcd",
        "branch": "main",
        "checksum": "sha256:1a2b3c4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f7890"
      }
    },
    {
      "name": "json",
      "version": "0.1.0",
      "source": {
        "type": "git",
        "url": "https://github.com/riipandi/koa-json",
        "rev": "f6e5d4c3b2a109876543210987654321098765ab",
        "branch": "main",
        "checksum": "sha256:2b3c4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f78901a"
      }
    },
    {
      "name": "utils",
      "version": "0.1.0",
      "source": {
        "type": "path",
        "path": "./utils",
        "checksum": "sha256:3c4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f78901a2b"
      }
    },
    {
      "name": "std",
      "version": "0.1.0",
      "source": {
        "type": "builtin",
        "checksum": "sha256:4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f78901a2b3c4d5e6f78901a2b3c"
      }
    }
  ],
  "metadata": {
    "generated_at": "2024-02-06T10:30:00Z",
    "compiler_version": "0.1.0"
  }
}
```

---

## Versioning

The lockfile format version (`version` field) follows Semantic Versioning. Breaking changes increment the MAJOR version.

**Current version:** `1.0.0`

**Version history:**
- `1.0.0` - Initial format

---

## Auto-Generation

The lockfile is automatically generated by the package manager:

```bash
# Generate lockfile from Koa.toml
koa fetch

# Update lockfile
koa update

# Verify lockfile
koa verify
```

**DO NOT edit the lockfile manually!** It will be overwritten.

---

## Reproducible Builds

The lockfile ensures reproducible builds by:

1. **Pinning revisions** - Git commits are identified by SHA-1
2. **Checksums** - SHA-256 checksums verify integrity
3. **Version constraints** - Semantic version constraints are resolved to exact versions
4. **Order independence** - JSON arrays preserve order

**Example:**

```bash
# Clone repository
git clone https://github.com/user/myapp
cd myapp

# Build (uses Koa.lock)
koa build

# Result: Bit-for-bit identical binary
```

---

## Validation

The lockfile can be validated:

```bash
# Validate lockfile format
koa validate --lockfile

# Verify checksums
koa verify

# Output:
# ✓ Valid Koa.lock
#   - Format version: 1.0.0
#   - Packages: 4
#   - All checksums verified
```

---

## JSON Schema

A JSON Schema is available for validation:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["version", "packages", "metadata"],
  "properties": {
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "packages": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["name", "version", "source"],
        "properties": {
          "name": { "type": "string" },
          "version": { "type": "string" },
          "source": {
            "oneOf": [
              { "$ref": "#/definitions/gitSource" },
              { "$ref": "#/definitions/pathSource" },
              { "$ref": "#/definitions/registrySource" },
              { "$ref": "#/definitions/builtinSource" }
            ]
          }
        }
      }
    },
    "metadata": {
      "type": "object",
      "required": ["generated_at", "compiler_version"],
      "properties": {
        "generated_at": { "type": "string", "format": "date-time" },
        "compiler_version": { "type": "string" }
      }
    }
  },
  "definitions": {
    "gitSource": {
      "type": "object",
      "required": ["type", "url", "rev", "checksum"],
      "properties": {
        "type": { "const": "git" },
        "url": { "type": "string", "format": "uri" },
        "rev": { "type": "string", "pattern": "^[a-f0-9]{40}$" },
        "branch": { "type": "string" },
        "tag": { "type": "string" },
        "checksum": { "type": "string", "pattern": "^sha256:[a-f0-9]{64}$" }
      }
    },
    "pathSource": {
      "type": "object",
      "required": ["type", "path", "checksum"],
      "properties": {
        "type": { "const": "path" },
        "path": { "type": "string" },
        "checksum": { "type": "string", "pattern": "^sha256:[a-f0-9]{64}$" }
      }
    },
    "registrySource": {
      "type": "object",
      "required": ["type", "url", "checksum"],
      "properties": {
        "type": { "const": "registry" },
        "url": { "type": "string", "format": "uri" },
        "checksum": { "type": "string", "pattern": "^sha256:[a-f0-9]{64}$" }
      }
    },
    "builtinSource": {
      "type": "object",
      "required": ["type", "checksum"],
      "properties": {
        "type": { "const": "builtin" },
        "checksum": { "type": "string", "pattern": "^sha256:[a-f0-9]{64}$" }
      }
    }
  }
}
```

---

## Migration

If the lockfile format changes, a migration command is available:

```bash
# Migrate to latest format
koa migrate --lockfile

# Show what would change
koa migrate --lockfile --dry-run
```

---

## See Also

- [Build System](16-build-system.md) - `Koa.toml` format
- [Package Manager](11-package-manager.md) - Dependency management
- [Build Cache](18-build-cache.md) - Incremental compilation
