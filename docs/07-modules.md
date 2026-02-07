# Module System

Koa uses hybrid module system: Rust-style file structure with TypeScript-style imports.

## Philosophy

- **File = Module** - Every file is a module
- **Explicit imports** - `import { ... } from "path"` syntax
- **Explicit exports** - `pub` keyword for public items
- **No hierarchies** - Flat module structure

---

## File Structure

```
src/
├── main.koa                    # main module
├── utils.koa                   # utils module
├── math/
│   ├── operations.koa          # math::operations module
│   └── mod.koa                 # math::mod module (optional)
└── std/
    ├── io/
    │   └── mod.koa             # std::io module
    └── collections/
        └── vec.koa             # std::collections::vec module
```

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

### Named Imports

```
// main.koa
import { greet } from "utils"

fn main(): i32 {
    let message: string = greet("World")
    println!("{}", message)
    0
}
```

### Multiple Imports

```
import { greet, farewell } from "utils"
import { Vec, HashMap } from "std/collections"
```

### Aliased Imports

```
import { greet as say_hello } from "utils"
import { Vec as Vector } from "std/collections"
```

### Module Import (Import All)

```
// Import all public items
import * as utils from "utils"

utils.greet("World")
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

```
// std/lib.koa
import { Vec } from "collections/vec"
import { HashMap } from "collections/hashmap"

// Re-export
pub { Vec, HashMap }
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
import { Vec } from "std/collections/vec"

// Local project imports (from src/)
import { operations } from "math/operations"

// External dependencies (from Koa.lock)
import { serve } from "net/http"
import { parse_json } from "koa_json"
```

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
    import { Vec } from "std/collections/vec"
    // → library/std/collections/vec.koa
    ```

2. **External Dependencies** (from `Koa.lock`)
    ```
    import { parse_json } from "koa_json"
    // → ~/.koa/cache/packages/koa-json-0.1.0/src/lib.koa
    ```

3. **Workspace Crates** (from `[workspace]` in `Koa.toml`)
    ```
    import { utils } from "myapp_utils"
    // → crates/utils/src/lib.koa
    ```

4. **Local Modules** (from `src/`)
    ```
    import { operations } from "math/operations"
    // → src/math/operations.koa
    ```

### Examples

```
// Stdlib (always available)
import { println } from "std/io"
import { Vec, HashMap } from "std/collections"

// External dependencies
import { http_get } from "net/http"             // Stdlib HTTP
import { parse_json } from "koa_json"           // External package

// Local modules
import { helper } from "utils"                  // src/utils.koa
import { Calculator } from "math/calculator"    // src/math/calculator.koa
```

### Conflict Resolution

If multiple sources define the same module:

1. **Stdlib wins** over external dependencies
2. **Explicit overrides** via `Koa.toml`
3. **Error** if unresolvable

```toml
# Koa.toml - Explicit override
[dependencies]
# Use this version instead of stdlib
http = { git = "https://github.com/custom/koa-http", version = "0.2.0" }
```
   ```

3. **Module index**: `mod.koa` for directory
   ```
   import { print } from "std/io"
   // → src/std/io/mod.koa (if exists)
   // → src/std/io.koa (fallback)
   ```

---

## Examples

### 1. Library Structure

```
// src/math/operations.koa
pub fn add(x: i32, y: i32): i32 {
    x + y
}

pub fn multiply(x: i32, y: i32): i32 {
    x * y
}
```

```
// src/math/mod.koa
import { add, multiply } from "./operations"

pub { add, multiply }

pub fn square(x: i32): i32 {
    multiply(x, x)
}
```

```
// src/main.koa
import { add, square } from "math/mod"

fn main(): i32 {
    let result: i32 = add(square(5), 10)
    println!("{}", result)
    0
}
```

### 2. Standard Library Usage

```
// src/main.koa
import { println, print } from "std/io/mod"
import { Vec, HashMap } from "std/collections"

fn main(): i32 {
    let numbers: Vec<i32> = Vec::new()
    try numbers.push(42)
    println!("{}", numbers.get(0))
    0
}
```

### 3. Aliasing

```
// src/main.koa
import { Vec as Vector } from "std/collections/vec"
import { fetch as http_get } from "net/http"

fn main(): i32 {
    let v: Vector<i32> = Vector::new()
    let data: string = await http_get(url)
    0
}
```

---

## Best Practices

### 1. Descriptive Module Names

```
// GOOD
import { http_get } from "net/http"

// AVOID
import { get } from "std/http"
```

### 2. Explicit Imports

```
// GOOD: Explicit
import { Vec, HashMap } from "std/collections"

// AVOID: Import semua
import * as collections from "std/collections"
collections::Vec::new()
```

### 3. Logical Module Structure

```
src/
├── main.koa              # Entry point
├── lib/
│   ├── mod.koa           # Public exports
│   ├── core.koa          # Core types
│   └── utils.koa         # Utilities
└── std/
    └── ...
```

---

## Comparison

| Aspect                 | Rust                | TypeScript                | Koa                       |
|------------------------|---------------------|---------------------------|---------------------------|
| **Module Declaration** | `mod.rs`            | File = module             | File = module             |
| **Import Syntax**      | `use crate::module` | `import { x } from "mod"` | `import { x } from "mod"` |
| **Export Syntax**      | `pub`               | `export`                  | `pub`                     |
| **Re-export**          | `pub use`           | `export { x }`            | `pub { x }`               |
| **Visibility**         | `pub`, `pub(crate)` | `public`, default         | `pub`, default            |

---

## Next Steps

- [Conditional Compilation](08-conditional-compilation.md) - Build modes and annotations
- [Syntax Guide](02-syntax-guide.md) - Back to syntax
