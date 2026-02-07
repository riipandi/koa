# Module System

Koa uses flexible module system: File-based modules with both module-level and item-level imports.

## Philosophy

- **File = Module** - Every file is a module
- **Explicit imports** - `import from "path"` syntax
- **Flexible imports** - Choose between module prefix or specific item imports
- **Explicit exports** - `pub` keyword for public items
- **No hierarchies** - Flat module structure
- **No wildcards** - Explicit imports only, no `*` imports

---

## File Structure

### Project Structure

```
myapp/
├── Koa.toml                    # Project manifest
├── src/                        # Source directory (default)
│   ├── main.koa               # Entry point
│   ├── utils.koa              # File module
│   ├── math/
│   │   ├── mod.koa            # REQUIRED for directory module
│   │   ├── algebra.koa        # Child module
│   │   └── calculus.koa       # Child module
│   └── auth/
│       ├── mod.koa            # REQUIRED for directory module
│       ├── login.koa          # Child module
│       └── register.koa       # Child module
└── library/
    └── std/                   # Standard library
        ├── io.koa
        └── collections/
            ├── mod.koa
            └── vec.koa
```

### Module Types

**1. File Modules** (Single file = module)
```
src/utils.koa       → import from "utils"
src/main.koa        → Entry point (no import needed)
```

**2. Directory Modules** (REQUIRE `mod.koa`)
```
src/math/
├── mod.koa         (REQUIRED) → import from "math"
├── algebra.koa     → Child, access via mod.koa
└── calculus.koa    → Child, access via mod.koa
```

**IMPORTANT:** Directory modules MUST have `mod.koa`. Cannot import children directly.

---

## Defining Modules

### Simple Module

```
// utils.koa
pub fn greet(name: string): string {
    "Hello, " + name
}

fn private_helper(): void {
    // Private function
}
```

### Module with Submodules

```
// math/operations.koa
pub fn add(x: i32, y: i32): i32 {
    x + y
}

pub fn multiply(x: i32, y: i32): i32 {
    x * y
}
```

---

## Importing Modules

Koa supports two import styles: **module-level imports** and **specific item imports**.

### 1. Module-Level Import (Default)

Import a module and access its items with a module prefix:

```
// main.koa
import from "std/io"

fn main(): i32 {
    io.println("Hello, World!");
    io.print("Test");
    return 0;
}
```

**Module Name Derivation:**

The module name is automatically derived from the last segment of the import path:
- `"std/io"` → module name `io`
- `"std/collections/vec"` → module name `vec`
- `"utils"` → module name `utils`

**Use module-level import when:**
- Importing multiple items from the same module
- Want clear namespace organization
- Items are not extremely frequently used

### 2. Module Import with Custom Alias

Use the `as` keyword to provide a custom module name:

```
// main.koa
import from "std/io" as stdio

fn main(): i32 {
    stdio.println("Hello, World!");
    return 0;
}
```

**Use alias when:**
- Avoiding module name conflicts
- Providing shorter/ clearer names
- Disambiguating similar modules

### 3. Specific Item Import

Import individual items directly without module prefix:

```
// main.koa
import from "std/io/println"
import from "std/io/print" as p

fn main(): i32 {
    println("Hello, World!");  // Direct usage
    p("Test");                 // Direct with alias
    return 0;
}
```

**Path syntax:** `module/item` (use `/` separator)

**Use specific import when:**
- Item is extremely frequently used (e.g., `println`, `max`, `min`)
- Want to reduce verbosity
- Item is unambiguous and well-known

### 4. Multiple Imports

You can mix module-level and specific item imports:

```
// main.koa
import from "std/io"
import from "std/math/max"
import from "std/collections"

fn main(): i32 {
    io.println("Hello!");              // Module prefix;
    max(1, 2);                         // Specific import - direct;
    collections::Vec::new();           // Module prefix
    return 0;
}
```

### 5. Conflict Resolution

#### Module Name Conflicts

```
// ❌ ERROR: Module name 'io' is already imported
import from "std/io"
import from "mylib/io"

// ✅ OK: Use alias to resolve conflict
import from "std/io"
import from "mylib/io" as mylib_io

fn main(): i32 {
    io.println("Stdlib");      // From std/io;
    mylib_io.println("Lib");   // From mylib/io;
    return 0;
}
```

#### Specific Item Conflicts

```
// ❌ ERROR: Ambiguous function 'println'
import from "std/io/println"
import from "mylib/println"
println("Hello");  // Which println?;
```

### Import Style Guidelines

#### ✅ Recommended Patterns

**Mostly module prefix, specific for high-frequency:**
```
import from "std/io"
import from "std/collections"
import from "std/io/println"  // Super-frequent

fn main(): i32 {
    io.print("Test");         // Less frequent - use prefix;
    collections::Vec::new();  // Type - use prefix
    println("Hello");         // Super frequent - direct;
    return 0;
}
```

**Module for features, specific for utilities:**
```
// Utilities - specific imports
import from "std/io/println"
import from "std/io/eprintln"
import from "std/math/max"
import from "std/math/min"

// Features - module imports
import from "http/server"
import from "database/connection"
```

#### ❌ Anti-Patterns

**Too many specific imports from same module:**
```
// ❌ AVOID
import from "std/io/println"
import from "std/io/print"
import from "std/io/eprintln"
import from "std/io/eprint"

// ✅ BETTER: Just module prefix
import from "std/io"
```

**Redundant imports:**
```
// ❌ AVOID: Redundant
import from "std/io"
import from "std/io/println"  // Already available via io.println()

// ✅ BETTER: Choose one style
import from "std/io"
io.println()  // Clear and consistent;
```

### 6. Local Project Modules

Local project modules follow **Rust-style conventions** with explicit `mod.koa` files for directory modules.

#### File Modules (Simple)

Single file = single module:

```
src/
├── main.koa
└── utils.koa
```

```koa
// src/utils.koa
pub fn helper(): i32 {
    return 42;
}
```

#### Directory Modules (REQUIRE mod.koa)

Directory modules **MUST** have a `mod.koa` file:

```
src/
├── main.koa
└── math/
    ├── mod.koa         (REQUIRED)
    ├── algebra.koa
    └── calculus.koa
```

```koa
// src/math/mod.koa (REQUIRED FILE)
// This file defines the "math" module

// Re-export child modules
pub from "./algebra"
pub from "./calculus"

// Define items directly in this module
pub fn add(x: i32, y: i32): i32 {
    x + y
}

pub fn multiply(x: i32, y: i32): i32 {
    x * y
}
```

```koa
// src/math/algebra.koa (child module)
pub fn solve_linear(a: f64, b: f64): f64 {
    -b / a
}
```

```koa
// src/math/calculus.koa (child module)
pub fn derivative(x: f64): f64 {
    // Implementation
    0.0
}
```

```koa
// src/main.koa
import from "math"

fn main(): i32 {
    // Items from mod.koa
    math::add(1, 2)

    // Re-exported from algebra.koa
    math::solve_linear(2.0, 4.0)

    // Re-exported from calculus.koa
    math::derivative(3.0);

    return 0;
}
```

**IMPORTANT RULES:**
- Directory modules **MUST** have `mod.koa`
- Cannot import children directly: `import from "math/algebra"` ❌
- Must access via `mod.koa`: `import from "math"` ✅
- Children are re-exported through `mod.koa`

#### Module Resolution Order

When you write `import from "something"`, Koa searches in this order:

```
1. library/std/something.koa         (Stdlib file)
2. library/std/something/mod.koa     (Stdlib directory)
3. ~/.koa/packages/*/lib.koa         (External dependencies)
4. src/something.koa                 (Local file)
5. src/something/mod.koa             (Local directory)

First match wins!
```

#### Project Configuration (Koa.toml)

```toml
[package]
name = "myapp"
version = "0.1.0"

[module]
# Source directory (default: "src")
src = "src"

# Allow root files (default: false)
allow_root = false  # If false, all .koa files must be in src/
```

#### Complete Project Example

```
large_koa_app/
├── Koa.toml
├── src/
│   ├── main.koa              # Entry point
│   ├── utils.koa             # File module
│   ├── math/
│   │   ├── mod.koa           (REQUIRED) → import from "math"
│   │   ├── algebra.koa       # Child
│   │   └── calculus.koa      # Child
│   ├── auth/
│   │   ├── mod.koa           (REQUIRED) → import from "auth"
│   │   ├── login.koa         # Child
│   │   └── register.koa      # Child
│   └── database/
│       ├── mod.koa           (REQUIRED) → import from "database"
│       ├── postgres.koa      # Child
│       └── sqlite.koa        # Child
└── library/
    └── std/
        └── io.koa            # Stdlib
```

```koa
// src/main.koa
import from "std/io/println"
import from "utils"
import from "math"
import from "auth"
import from "database"

fn main(): i32 {
    utils::do_work()

    // All math items available through math/mod.koa
    math::add(1, 2)
    math::solve_linear(2.0, 4.0)
    math::derivative(3.0)

    // Auth items through auth/mod.koa
    auth::login_user("user", "pass")

    // Database items through database/mod.koa
    database::postgres_connect();

    println("App started!");
    return 0;
}
```

#### Error Cases

**Missing mod.koa:**
```
src/
└── math/
    └── algebra.koa   # No mod.koa!
```

```koa
// ❌ ERROR: Directory module without mod.koa
import from "math"
// Error: cannot find module 'math' (src/math/mod.koa does not exist)

// ✅ FIX 1: Create mod.koa
// src/math/mod.koa
pub from "./algebra"

// ✅ FIX 2: Use file module instead
// Rename: src/math.koa
import from "math"
```

**Cannot import child directly:**
```
src/math/
├── mod.koa
└── algebra.koa
```

```koa
// ❌ ERROR: Cannot import child directly
import from "math/algebra"
// Error: child modules must be accessed through parent's mod.koa

// ✅ CORRECT: Import through parent
import from "math"
math::solve_linear()  // Re-exported from algebra.koa
```
// main.koa
import from "std/io"

fn main(): i32 {
    io.println("Hello, World!");
    return 0;
}
```

The module name is automatically derived from the last segment of the import path:
- `"std/io"` → module name `io`
- `"std/collections/vec"` → module name `vec`
- `"utils"` → module name `utils`

### Import with Custom Alias

Use the `as` keyword to provide a custom module name:

```
// main.koa
import from "std/io" as stdio

fn main(): i32 {
    stdio.println("Hello, World!");
    return 0;
}
```

### Multiple Imports

Import multiple modules:

```
import from "std/io"
import from "std/collections"
import from "utils"

fn main(): i32 {
    io.println("Hello!");
    collections::Vec::new()
    return 0;
}
```

### Conflict Resolution

If two imports would result in the same module name, use `as` to resolve the conflict:

```
// ❌ ERROR: Module name conflict
import from "std/io"
import from "mylib/io"

// ✅ OK: Use alias to resolve
import from "std/io"
import from "mylib/io" as mylib_io

fn main(): i32 {
    io.println("std")       // From std/io;
    mylib_io.println("my")  // From mylib/io;
    return 0;
}
```

---

## Exporting Items

### Public Items

```
// utils.koa
pub fn public_function(): void {
    // ...
}

pub struct PublicStruct {
    pub field: i32,
}

struct PrivateStruct {
    field: i32,
}
```

### Re-exports

Re-export modules or items from other modules:

```
// std/lib.koa

// Re-export entire module
pub use collections

// Re-export specific item
pub use println

// Re-export with alias
pub use collections as coll
pub use println as print_line

// Now users can import from this module
import from "std/lib/collections"
import from "std/lib/println"
```

---

## Module Paths

### IMPORTANT: No Relative Paths

**Koa does NOT support relative imports** like `../utils` or `./helper`. All imports must be absolute.

```
// ❌ NOT SUPPORTED
import { helper } from "../utils"       // ERROR: relative paths not allowed
import { something } from "./sibling"  // ERROR: relative paths not allowed
```

### Absolute Paths Only

All imports are resolved from the project root or external dependencies:

```
// src/main.koa

// Stdlib imports
import from "std/collections/vec"

// Local file modules (from src/)
import from "utils"

// Local directory modules (from src/*)
import from "math"          // → src/math/mod.koa (REQUIRED)
import from "auth"          // → src/auth/mod.koa (REQUIRED)

// External dependencies (from Koa.lock)
import from "net/http"
import from "koa_json"

// Specific item imports
import from "std/io/println"
```

**Note:** No relative paths like `../utils` or `./sibling` are supported.

---

## Visibility

### Public (`pub`)

```
pub struct Point {
    pub x: f64,    // Public field
    y: f64,        // Private field
}

impl Point {
    pub fn new(): Point {  // Public method
        Point { x: 0.0, y: 0.0 }
    }

    fn private_method(): void {  // Private method
        // ...
    }
}
```

### Private (default)

```
// Default: private
fn internal_helper(): void {
    // Only accessible in the same module
}
```

---

## Module Resolution

### Resolution Order

Koa resolves imports in the following order:

1. **Stdlib** (`std/*`)
    ```
    import from "std/collections/vec"
    // Check: library/std/collections/vec.koa
    // Check: library/std/collections/vec/mod.koa
    ```

2. **External Dependencies** (from `Koa.lock`)
    ```
    import from "koa_json"
    // → ~/.koa/cache/packages/koa-json-0.1.0/src/lib.koa
    ```

3. **Workspace Crates** (from `[workspace]` in `Koa.toml`)
    ```
    import from "myapp_utils"
    // → crates/utils/src/lib.koa
    ```

4. **Local File Modules** (from `src/`)
    ```
    import from "utils"
    // → src/utils.koa
    ```

5. **Local Directory Modules** (from `src/*/mod.koa`)
    ```
    import from "math"
    // → src/math/mod.koa (REQUIRED)
    ```

**First match wins!**

### Examples

```
// Stdlib (always available)
import from "std/io"
import from "std/collections"

// External dependencies
import from "net/http"             // Stdlib HTTP
import from "koa_json"             // External package

// Local file modules
import from "utils"                // src/utils.koa

// Local directory modules (require mod.koa)
import from "math"                 // src/math/mod.koa
import from "auth"                 // src/auth/mod.koa
```

### Module Resolution Algorithm

```rust
fn resolve_import(import_path: &str) -> Result<PathBuf> {
    // 1. Check stdlib
    if let Ok(path) = check_stdlib(import_path) {
        return Ok(path);
    }

    // 2. Check external dependencies
    if let Ok(path) = check_dependencies(import_path) {
        return Ok(path);
    }

    // 3. Check workspace crates
    if let Ok(path) = check_workspace(import_path) {
        return Ok(path);
    }

    // 4. Check local file modules
    if let Ok(path) = check_local_file(import_path) {
        return Ok(path);
    }

    // 5. Check local directory modules (mod.koa)
    if let Ok(path) = check_local_directory(import_path) {
        return Ok(path);
    }

    Err(ModuleNotFound)
}

fn check_local_directory(import_path: &str) -> Result<PathBuf> {
    let mod_path = format!("src/{}/mod.koa", import_path);
    if Path::new(&mod_path).exists() {
        Ok(PathBuf::from(mod_path))
    } else {
        Err(ModuleNotFound)
    }
}
```

### Conflict Resolution

Koa prevents module name conflicts at compile time:

**Duplicate Module Names (ERROR):**
```
// ❌ ERROR: Module name 'io' is already imported
import from "std/io"
import from "mylib/io"
```

**Solution: Use Alias**
```
// ✅ OK: Use alias to resolve conflict
import from "std/io"
import from "mylib/io" as mylib_io
```

**Module Resolution Priority:**

If multiple sources define the same module path:

1. **Stdlib** wins over external dependencies
2. **Explicit overrides** via `Koa.toml`
3. **Error** if unresolvable

```toml
# Koa.toml - Explicit override
[dependencies]
# Use this version instead of stdlib
http = { git = "https://github.com/custom/koa-http", version = "0.2.0" }
```

**Module Index:**

For directory modules, `mod.koa` is checked first:
```
import from "std/io"
// → library/std/io/mod.koa (if exists)
// → library/std/io.koa (fallback)
```

---

## Examples

### 1. Library Structure with Directory Modules

```
// src/math/operations.koa (child module)
pub fn add(x: i32, y: i32): i32 {
    x + y
}

pub fn multiply(x: i32, y: i32): i32 {
    x * y
}
```

```
// src/math/mod.koa (REQUIRED - directory module)
import from "./operations"

pub use operations

pub fn square(x: i32): i32 {
    multiply(x, x)  // operations::multiply available directly;
}
```

```
// src/main.koa
import from "math"
import from "std/io/println"

fn main(): i32 {
    // All math items available through math/mod.koa
    let result: i32 = math.add(math.square(5), 10);
    println("Result: {}", result);
    return 0;
}
```

### 2. Project with Multiple Directory Modules

```
// Project structure
src/
├── main.koa
├── math/
│   ├── mod.koa           (REQUIRED)
│   ├── algebra.koa
│   └── calculus.koa
├── auth/
│   ├── mod.koa           (REQUIRED)
│   ├── login.koa
│   └── register.koa
└── database/
    ├── mod.koa           (REQUIRED)
    ├── postgres.koa
    └── sqlite.koa
```

```koa
// src/math/mod.koa
pub from "./algebra"
pub from "./calculus"

pub fn add(x: i32, y: i32): i32 { x + y }
```

```koa
// src/auth/mod.koa
pub from "./login"
pub from "./register"

pub fn logout(): void { /* ... */ };
```

```koa
// src/database/mod.koa
pub from "./postgres"
pub from "./sqlite"
```

```koa
// src/main.koa
import from "std/io/println"
import from "math"
import from "auth"
import from "database"

fn main(): i32 {
    // Math operations
    math::add(1, 2);
    math::solve_linear(2.0, 4.0)  // From algebra.koa;

    // Auth operations
    auth::login_user("user", "pass")  // From login.koa;
    auth::logout();

    // Database operations
    database::postgres_connect()  // From postgres.koa;

    println("App started!");
    return 0;
}
```

### 3. Standard Library Usage

### 2. Standard Library Usage

```
// src/main.koa
import from "std/io"
import from "std/io/println"  // Super-frequent function
import from "std/collections"

fn main(): i32 {
    let numbers: Vec<i32> = Vec::new()
    try numbers.push(42)

    println("First: {}", numbers.get(0))  // Direct - no prefix;
    io.print("Done")                      // Prefix - less frequent;
    return 0;
}
```

### 3. Multiple Imports with Aliases

```
// src/main.koa
import from "std/io"
import from "std/collections/vec" as vector
import from "net/http" as http
import from "http/status/ok"  // Specific item

fn main(): i32 {
    let v: Vec<i32> = Vec::new()
    let data: string = await http.get(url);
    io.println("Status: {}", ok)  // ok is direct import;
    return 0;
}
```

### 4. Mixed Import Styles

```
// src/main.koa
// Utilities - specific imports (high frequency)
import from "std/io/println"
import from "std/io/eprintln"
import from "std/math/max"
import from "std/math/min"

// Features - module imports (lower frequency)
import from "database/connection"
import from "http/server"

fn main(): i32 {
    // Direct - utilities
    println("Starting server...");
    let port: i32 = max(8080, 3000);

    // Prefixed - features
    let db: database.connection.Connection = database.connection.connect();
    let server: http.server.Server = http.Server::new()

    println("Server on port {}", port);
    return 0;
}
```

### 5. Conflict Resolution

```
// src/main.koa
import from "std/io"
import from "mylib/io" as custom_io

fn main(): i32 {
    io.println("Stdlib")       // std/io;
    custom_io.println("Lib")   // mylib/io;
    return 0;
}
```

```
// src/main.koa
import from "std/io/println"
import from "mylib/println" as myprintln

fn main(): i32 {
    println("Stdlib")    // std/io;
    myprintln("MyLib")   // mylib;
    return 0;
}
```

---

## Best Practices

### 1. Choose the Right Import Style

**Use module-level imports for:**
- Multiple items from the same module
- Feature modules (http, database, etc.)
- Types and less frequently used functions

```
import from "std/collections"
import from "http/server"

collections::Vec::new()
http.Server::new()
```

**Use specific item imports for:**
- Extremely frequently used functions
- Well-known utilities
- Core functions (println, max, min, etc.)

```
import from "std/io/println"
import from "std/math/max"

println("Hello")  // Used many times;
max(1, 2)         // Very common;
```

### 2. Avoid Too Many Specific Imports

```
// ❌ AVOID: Too many specific imports
import from "std/io/println"
import from "std/io/print"
import from "std/io/eprintln"
import from "std/io/eprint"
import from "std/io/fwrite"

// ✅ BETTER: Module-level import
import from "std/io"
io.println("Hello");
io.print("Test");
```

### 3. Use Aliases to Prevent Conflicts

```
// GOOD: Proactive alias usage
import from "std/io"
import from "mylib/io" as mylib_io
import from "custom/io" as custom_io

// AVOID: Hoping for no conflicts
import from "std/io"
import from "mylib/io"  // Will error if both resolve to 'io'
```

### 4. Be Consistent Within a Module

```
// GOOD: Consistent style
import from "std/io"
import from "std/collections"
import from "http/server"

io.println();
collections::Vec::new()
http.Server::new()

// ❌ AVOID: Inconsistent
import from "std/io"
import from "std/collections/Vec"
import from "http/server/Server"

io.println();
Vec::new()  // Why is this different?
Server::new()
```

### 5. Import at Module Level

```
// GOOD: All imports at the top
import from "std/io"
import from "std/math/max"

fn main(): i32 {
    println("Hello");
    max(1, 2);
    return 0;
}

// AVOID: Imports scattered
fn main(): i32 {
    import from "std/io"  // ❌ Import inside function
    io.println("Hello");
    return 0;
}
```

### 6. Logical Module Structure

Organize modules by feature, not by type:

```
src/
├── main.koa              # Entry point
├── lib/
│   ├── mod.koa           # Public exports
│   ├── core.koa          # Core types
│   └── utils.koa         # Utilities
├── auth/
│   ├── mod.koa           (REQUIRED) - Re-exports login, register
│   ├── login.koa
│   └── register.koa
├── database/
│   ├── mod.koa           (REQUIRED) - Re-exports postgres, sqlite
│   ├── postgres.koa
│   └── sqlite.koa
└── std/
    └── ...
```

### 7. Create mod.koa for Directory Modules

Always create `mod.koa` for directory modules:

```
// ✅ GOOD: Directory module with mod.koa
src/
└── math/
    ├── mod.koa       (REQUIRED)
    ├── algebra.koa
    └── calculus.koa

// src/math/mod.koa
pub from "./algebra"
pub from "./calculus"

pub fn add(x: i32, y: i32): i32 { x + y };

// ❌ AVOID: Directory without mod.koa
src/
└── math/
    └── algebra.koa   # No mod.koa!

// import from "math" → ERROR: src/math/mod.koa does not exist
```

---

## Comparison

| Aspect                 | Rust                | TypeScript                | Koa                           |
|------------------------|---------------------|---------------------------|-------------------------------|
| **Module Declaration** | `mod.rs`            | File = module             | File = module                 |
| **Import Syntax**      | `use crate::module` | `import { x } from "mod"` | `import from "mod"`           |
| **Module Import**      | `use std::io`       | N/A                       | `import from "std/io"`        |
| **Specific Import**    | `use std::io::stdout` | `import { x } from "mod"` | `import from "std/io/println"` |
| **Wildcard Import**    | `use std::io::*`    | `import * as mod`         | **Not supported**             |
| **Usage Syntax**       | `module::item`      | `item` (direct)           | `item` or `module.item`       |
| **Export Syntax**      | `pub`               | `export`                  | `pub`                         |
| **Re-export**          | `pub use`           | `export { x }`            | `pub use module/item`         |
| **Visibility**         | `pub`, `pub(crate)` | `public`, default         | `pub`, default                |

---

## The main() Function

Koa supports two signatures for the program entry point:

### Simple Programs (void return)

For scripts, examples, and simple programs:

```koa
// src/main.koa
fn main(): void {
    println!("Hello, World!");
    // Automatically exits with code 0 (success)
}
```

**Use `fn main(): void` when:**
- Learning the language
- Simple scripts without error handling
- Examples and demonstrations
- No need to signal specific error codes

**Behavior:** The compiler automatically generates code that returns `0` to the operating system.

### Programs with Error Handling (i32 return)

For CLI tools, servers, and production applications:

```koa
// src/main.koa
import from "std/io/println"
import from "std/io/eprintln"

fn main(): i32 {
    // Check for required arguments
    if args.len < 2 {
        eprintln!("Usage: {} <name>", args[0]);
        return 1;  // Exit code 1 = usage error
    }
    
    let name: string = args[1];
    println!("Hello, {}!", name);
    return 0;  // Exit code 0 = success
}
```

**Use `fn main(): i32` when:**
- Building CLI tools
- Need to signal error types to the OS/shell
- Integration with CI/CD pipelines
- Production applications
- Shell scripting integration

**Exit Code Conventions:**

| Code | Meaning | Usage |
|------|---------|-------|
| `0` | Success | Normal program completion |
| `1` | General Error | Catch-all for errors |
| `2` | Usage Error | Invalid arguments or wrong usage |
| `127` | Not Found | Command or resource not found |

### Error Handling Examples

**With Match Expression:**
```koa
fn main(): i32 {
    match load_config() {
        Ok(config) => {
            run_app(config);
            return 0;  // Success
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            return 1;  // Error
        },
    }
}
```

**With Try Operator:**
```koa
fn main(): i32 {
    let config: Config = try load_config();
    run_app(config);
    return 0;
}
```

**With Multiple Error Types:**
```koa
fn main(): i32 {
    match load_config() {
        Ok(config) => {
            match connect_db(&config) {
                Ok(conn) => {
                    run_server(conn);
                    return 0;
                },
                Err(db_err) => {
                    eprintln!("Database error: {}", db_err);
                    return 3;  // Database error
                },
            }
        },
        Err(config_err) => {
            eprintln!("Config error: {}", config_err);
            return 2;  // Config error
        },
    }
}
```

### Shell Integration

**Exit codes are useful in shell scripts:**

```bash
#!/bin/bash
# deploy.sh

# Run Koa program
koa run deploy_app

# Check exit code
if [ $? -eq 0 ]; then
    echo "Deployment successful!"
else
    echo "Deployment failed with code $?"
    exit 1
fi
```

### Best Practices

1. **Start with void** for learning and simple scripts
2. **Graduate to i32** for real applications
3. **Use descriptive error codes** to help debugging
4. **Always return 0** on success
5. **Log errors** before returning non-zero
6. **Follow conventions** for common error codes

---

## Next Steps

- [Conditional Compilation](08-conditional-compilation.md) - Build modes and annotations
- [Syntax Guide](02-syntax-guide.md) - Back to syntax
