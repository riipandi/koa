# Koa CLI Quick Reference

## Core Commands

### Compilation
```bash
# Compile a Koa file
koa build <input.koa>

# Build with specific output
koa build <input.koa> -o <output>

# Build in release mode
koa build <input.koa> --mode release
```

### Running
```bash
# Run a Koa file (build + execute)
koa run <input.koa>
```

### Testing
```bash
# Run all tests
koa test

# Run specific tests
koa test --filter <test_name>
```

---

## Package Management (koa pkg)

All package management commands are under the `pkg` subcommand:

### Add Dependencies
```bash
# Add with version
koa pkg add http@0.1.0 --git https://github.com/riipandi/koa-http

# Add with separate options
koa pkg add json --git https://github.com/riipandi/koa-json --version "^0.1.0"

# Add local dependency
koa pkg add utils --path ./utils

# Add with branch
koa pkg add auth --git https://github.com/user/koa-auth --branch main
```

### Remove Dependencies
```bash
koa pkg remove <package_name>
```

### List Dependencies
```bash
# Show all dependencies
koa pkg list
```

Output:
```
Dependencies (3)
  http  v0.1.0  git+https://github.com/riipandi/koa-http
  json  v0.1.0  git+https://github.com/riipandi/koa-json
  utils v0.1.0  path+./utils
```

### Dependency Tree
```bash
# Show dependency tree
koa pkg tree
```

Output:
```
myapp v0.1.0

└── http v0.1.0
```

### Check Updates
```bash
# Check for outdated dependencies
koa pkg outdated
```

### Fetch Dependencies
```bash
# Download all dependencies
koa pkg fetch
```

### Update Dependencies
```bash
# Update all dependencies
koa pkg update

# Update specific dependency
koa pkg update http
```

---

## Build Modes

Available build modes:
- `debug` (default) - Fast compilation, no optimization
- `release` - Balanced optimization
- `release-fast` - Maximum performance
- `release-small` - Minimum size

```bash
koa build <input.koa> --mode release
```

---

## Progress Indicators

The CLI now shows progress for operations:

```
⠋ Building file.koa in debug mode
⠋ Reading source...
✓ Source read: 1024 bytes
⠋ Lexing...
✓ Tokens: 150
⠋ Parsing...
✓ Declarations: 10
✓ Type check passed
⠋ Lowering to IR...
✓ IR functions: 5
⠋ Generating LLVM IR...
✓ LLVM IR written to: "file.ll"
```

---

## Common Workflows

### New Project
```bash
# Initialize project
koa init myproject
cd myproject

# Add dependency
koa pkg add http --git https://github.com/riipandi/koa-http --version 0.1.0

# Fetch dependencies
koa pkg fetch

# Build
koa build src/main.koa

# Run
koa run src/main.koa
```

### Add Dependency
```bash
# Add to Koa.toml automatically
koa pkg add <package> --git <url> --version <version>

# Download dependencies
koa pkg fetch

# Build with new dependency
koa build
```

### Update Dependencies
```bash
# Check what's outdated
koa pkg outdated

# Update all
koa pkg update

# Or update specific package
koa pkg update <package>
```

---

## Koa.toml Management

The CLI automatically edits `Koa.toml` when using `pkg add` and `pkg remove`:

**Before manual editing:**
```toml
[dependencies]
http = { git = "...", version = "0.1.0" }
```

**After using CLI:**
```bash
koa pkg add http --git https://... --version 0.1.0
```

Both produce the same `Koa.toml`, but the CLI command validates inputs and formats correctly.

---

## Error Handling

The CLI provides helpful error messages:

```
⠋ Adding http...
✗ Error: Must specify --git or --path
Error: Package location not specified. Use --git URL or --path PATH
```

```
⠋ Fetching dependencies...
✗ Error: Koa.toml not found
Error: Koa.toml not found. Are you in a Koa project?
```

---

## Getting Help

```bash
# General help
koa --help

# Package management help
koa pkg --help

# Specific command help
koa pkg add --help
```

---

## Colored Output

The CLI uses colors for better readability:
- 🟢 Green - Success messages
- 🔵 Cyan - Package names, files
- 🟡 Yellow - Versions, warnings
- 🔴 Red - Errors
- ⚪ Dim - URLs, paths
