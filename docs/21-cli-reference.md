# Koa CLI Quick Reference

## Core Commands

### Version
```bash
# Show version information
koa --version
# or
koa -V

# Output format:
# koa VERSION OS/ARCH (GIT_HASH TIMESTAMP)
# Example: koa 0.1.0 macos/aarch64 (9081a89 2026-02-06T21:53Z)
```

### Compilation
```bash
# Compile a Koa file
koa build <input.koa>

# Build with specific output
koa build <input.koa> -o <output>

# Build in release mode
koa build <input.koa> --mode release

# Build project (auto-detects Koa.toml)
koa build

# Build with custom working directory
koa --cwd /path/to/project build
```

### Running
```bash
# Run a Koa file (build + execute)
koa run <input.koa>

# Run project in current directory
koa run

# Run with custom working directory
koa --cwd /path/to/project run
```

### Testing
```bash
# Run all tests
koa test

# Run specific tests
koa test --filter <test_name>
```

### Project Initialization

#### Standard Usage
```bash
# Create new project in directory
koa init myproject

# Create in current directory (must be empty)
# Prompts for project name interactively
koa init
```

#### Interactive Mode (Empty Directory)
When you run `koa init` in an empty directory, it will prompt for project name:
```
? What is your project's name? › my-awesome-project
```

The project name must:
- Not be empty
- Contain only letters, numbers, underscores, and hyphens

#### Generated Files
The `init` command creates:
```
myproject/
├── .gitignore      # Git ignore patterns
├── Koa.toml        # Project configuration
├── README.md       # Project documentation
└── src/
    └── main.koa    # Entry point with hello world
```

#### .gitignore Contents
```
.DS_Store
.DS_Store?
Thumbs.db
ehthumbs.db
Desktop.ini
$RECYCLE.BIN/
*.sqlite*
*.sqlite3*
*.db
.cache/
.temp/
/build/
/temp
```

---

## Global Options

### --cwd (Change Working Directory)
```bash
# Format
koa [OPTIONS] <COMMAND>

# Short form
koa -C <path> <command>

# Long form
koa --cwd <path> <command>

# Examples
koa --cwd examples/basic run
koa -C examples/simple build factorial.koa
```

**Use Cases:**
- Build/run projects without changing directories
- Useful for scripts and automation
- Works with all subcommands

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
# Method 1: Create in new directory
koa init myproject
cd myproject

# Method 2: Create in current empty directory (interactive)
mkdir myproject && cd myproject
koa init
# Prompts for project name

# Build and run
koa build
# or
koa run
```

### Working with Multiple Projects
```bash
# Build project in different directory without cd
koa --cwd ~/projects/myapp build

# Run tests in another directory
koa -C ~/projects/mylib test

# Run examples
koa --cwd examples/basic run
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

# Show version
koa --version
# or
koa -V

# Package management help
koa pkg --help

# Specific command help
koa pkg add --help
koa init --help
```

---

## Colored Output

The CLI uses colors for better readability:
- 🟢 Green - Success messages
- 🔵 Cyan - Package names, files
- 🟡 Yellow - Versions, warnings
- 🔴 Red - Errors
- ⚪ Dim - URLs, paths
