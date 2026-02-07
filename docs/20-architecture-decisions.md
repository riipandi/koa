# Architecture Decisions (ADRs)

This document contains Architecture Decision Records (ADRs) for major design choices in Koa.

---

## ADR-001: snake_case Naming Convention

**Status:** Accepted

**Context:**
Koa targets TypeScript developers who are familiar with camelCase. However, Koa is also a general-purpose compiled language.

**Decision:**
Use `snake_case` for variables and functions, `PascalCase` for types.

**Rationale:**
- Consistent with Rust, showing Koa is a compiled language
- More readable for multi-word identifiers
- TypeScript developers are already familiar with snake_case from other languages
- Clear distinction between types and values

**Consequences:**
```
// Variables and functions
const user_name: string = "Alice";
fn calculate_sum(): i32 { }

// Types
struct Point { }
enum Color { }
type Result<T> = Ok(T) | Err(Error)
```

**Alternatives Considered:**
- camelCase: Rejected because it's less common in systems languages
- kebab-case: Rejected because not valid in most languages

---

## ADR-002: Required Return Keyword

**Status:** Accepted

**Context:**
Some languages (Rust, Ruby) allow implicit returns. TypeScript requires explicit returns.

**Decision:**
The `return` keyword is **required** for all return values. No implicit returns.

**Rationale:**
- More explicit and readable
- Consistent with TypeScript
- No confusion about control flow
- Easier for beginners to understand
- Compiler can still optimize tail calls

**Consequences:**
```
// ✅ CORRECT
fn add(x: i32, y: i32): i32 {
    return x + y;
}

// ❌ ERROR
fn add(x: i32, y: i32): i32 {
    x + y  // ERROR: use 'return' keyword
}
```

**Alternatives Considered:**
- Implicit return (last expression): Rejected for clarity
- Optional return (like Rust): Rejected for consistency

---

## ADR-003: No Relative Paths in Module Imports

**Status:** Accepted

**Context:**
Many languages allow relative imports (`../utils`, `./helpers`). This can lead to dependency hell.

**Decision:**
Module imports must be absolute. No relative paths allowed.

**Resolution Order:**
1. Stdlib (`std/*`)
2. External dependencies (from `Koa.lock`)
3. Workspace crates (from `[workspace]`)
4. Local `src/` modules

**Rationale:**
- Easier refactoring (move files without breaking imports)
- Clear dependency structure
- Consistent with Rust's module system
- Simpler tooling

**Consequences:**
```
// ✅ CORRECT
import from "net/http";
import from "myapp_utils";

// ✅ CORRECT (specific items)
import from "net/http/serve";
import from "myapp_utils/helper";

// ❌ ERROR
import from "../utils";
import from "./helpers";
```

**Alternatives Considered:**
- Relative paths: Rejected for complexity
- Node.js resolution: Rejected for ambiguity

---

## ADR-004: JSON Lockfile Format

**Status:** Accepted

**Context:**
Many package managers use TOML, YAML, or custom formats for lockfiles.

**Decision:**
Use JSON format for `Koa.lock`.

**Rationale:**
- Fast native parsing in most languages
- Better tooling support (jq, JSON Schema)
- Git-friendly (line-based diffs)
- No TOML ambiguity
- Industry standard for lockfiles (npm, Cargo uses TOML but JSON is common)

**Consequences:**
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
  ]
}
```

**Alternatives Considered:**
- TOML: Rejected (slower parsing, ambiguous)
- YAML: Rejected (slower parsing, complexity)
- Custom binary format: Rejected (not human-readable)

---

## ADR-005: GC Configuration Redesign

**Status:** Accepted

**Context:**
Go uses `GOGC` environment variable. This naming is confusing for Koa.

**Decision:**
Use Koa-specific naming:
- CLI flag: `--gc-percent`
- Environment variable: `KOA_GC_PERCENT`
- Programmatic API: `std/runtime/gc` module

**Rationale:**
- Koa-specific naming is clearer
- Programmatic control important for production
- Separation from Go's conventions
- More flexibility for future configuration

**Consequences:**
```
// CLI
koa run --gc-percent=200
koa run --gc-limit=2GB

// Environment
export KOA_GC_PERCENT=off
export KOA_GC_LIMIT=1GB

// Programmatic
import { gc } from "std/runtime";

gc::collect();
gc::set_percent(150);
```

**Alternatives Considered:**
- Copy Go's GOGC: Rejected (confusing)
- Only environment variables: Rejected (not flexible enough)
- Only programmatic: Rejected (not convenient for ops)

---

## ADR-006: No Arrow Functions

**Status:** Accepted

**Context:**
TypeScript has arrow functions `() => {}`. Should Koa support them?

**Decision:**
Do not support arrow function syntax. Use regular `fn` keyword.

**Rationale:**
- Koa doesn't have JavaScript's "closure hell"
- `fn` is already short and clear
- Arrow functions add parser complexity
- Function pointers sufficient for higher-order functions
- Consistent with other systems languages (Rust, Go, Zig)

**Consequences:**
```
// ✅ CORRECT
map(numbers, fn(x: i32): i32 {
    return x * 2;
});

// ✅ ALSO CORRECT (assign to variable)
const doubler: fn(i32): i32 = fn(x: i32): i32 {
    return x * 2;
};

// ❌ NOT SUPPORTED
map(numbers, (x) => x * 2);
```

**Alternatives Considered:**
- Arrow functions: Rejected (adds complexity)
- Closures with `fn`: Accepted (already supported)

---

## ADR-007: Enum with Integer Values Only

**Status:** Accepted

**Context:**
Some languages support enum values (integers, strings). TypeScript allows both.

**Decision:**
Support enum with integer values only. No string enums.

**Rationale:**
- Integer enums useful for FFI (C enums)
- String enums not essential
- Can use `const` strings as alternative
- Reduces compiler complexity

**Consequences:**
```
// ✅ CORRECT (integer enum)
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
}

// ✅ ALSO CORRECT (no values)
enum Color {
    Red,
    Green,
    Blue,
}

// ❌ NOT SUPPORTED (string enum)
enum Color {
    Red = "red",
    Green = "green",
}

// Alternative for string constants
const COLOR_RED: string = "red";
const COLOR_GREEN: string = "green";
```

**Alternatives Considered:**
- String enums: Rejected (not essential)
- Arbitrary expression values: Rejected (too complex)

---

## ADR-008: Rust-style Documentation Comments

**Status:** Accepted

**Context:**
Many documentation comment styles exist (Javadoc, Doxygen, docblocks).

**Decision:**
Use Rust-style documentation comments (`///` and `//!`).

**Rationale:**
- Clean and familiar to Rust developers
- Support for markdown formatting
- Testable examples in documentation
- IDE-friendly
- Proven effective in large projects

**Consequences:**
```
///
/// Calculate the distance between two points
///
/// # Examples
/// ```
/// const p1: Point = Point::new(0.0, 0.0);
/// const p2: Point = Point::new(3.0, 4.0);
/// const dist: f64 = p1.distance(p2);
/// assert_eq!(dist, 5.0);
/// ```
///
/// # Parameters
/// - `other`: The other point
///
/// # Returns
/// The distance in units
///
pub fn distance(self, other: Point): f64 {
    return (self.x - other.x).sqrt();
}
```

**Alternatives Considered:**
- Javadoc (`/** */`): Rejected (verbose)
- JSDoc (`/** */`): Rejected (TypeScript association)
- Doxygen (`/// @param`): Rejected (requires annotations)

---

## ADR-009: No OOP Inheritance

**Status:** Accepted

**Context:**
Should Koa support class inheritance like TypeScript?

**Decision:**
Do not support OOP inheritance. Use composition and traits (future).

**Rationale:**
- Composition over inheritance
- Reduces complexity
- Avoids diamond problem
- More flexible
- Consistent with Go's philosophy

**Consequences:**
```
// ✅ CORRECT (composition)
struct Animal {
    name: string,
}

struct Dog {
    animal: Animal,
    breed: string,
}

// ❌ NOT SUPPORTED (inheritance)
class Dog extends Animal { }
```

**Alternatives Considered:**
- Single inheritance: Rejected (complexity)
- Interface inheritance: Rejected (use traits in future)
- Multiple inheritance: Rejected (too complex)

---

## ADR-010: Explicit Type Annotations Required

**Status:** Accepted

**Context:**
TypeScript allows type inference from initializers. Should Koa?

**Decision:**
Type annotations are **required** for all variable declarations. No inference from initializers.

**Rationale:**
- More explicit and readable
- Easier to understand code
- Catches type errors early
- Consistent with explicit philosophy
- Better IDE support

**Consequences:**
```
// ✅ CORRECT
let x: i32 = 42;
let y: string = "hello";

// ❌ ERROR (type annotation required)
let x = 42;
let y = "hello";
```

**Note:** Type inference is still used in other contexts (generic functions, return types).

**Alternatives Considered:**
- Inference from initializers: Rejected (not explicit enough)
- Optional annotations: Rejected (inconsistent)

---

## ADR-014: Hybrid Import System (Module Prefix + Specific Items)

**Status:** Accepted

**Context:**
Koa needs a module import system. Options include:
- TypeScript-style: `import { x } from "mod"` → direct usage
- Go-style: `import "mod"` → prefixed usage
- Rust-style: Both `use std::io` and `use std::io::stdout`

**Decision:**
Support **both** module-level and specific item imports, but **no wildcard imports**.

**Syntax:**
```koa
// Module-level import (default)
import from "std/io";
io.println("Hello");

// Module with alias
import from "std/io" as stdio;
stdio.println("Hello");

// Specific item import
import from "std/io/println";
println("Hello");  // Direct - no prefix

// Specific item with alias
import from "std/io/println" as p;
p("Hello");
```

**Key Rules:**
1. Path separator: `/` (consistent with module paths)
2. Only 1 level: `module/item` (not `mod/submod/item`)
3. Module name = last segment of import path (e.g., `"std/io"` → `io`)
4. Conflict detection for both module names and specific items
5. **No wildcard imports** (`import from "std/io/*"` is ERROR)

**Rationale:**
- **Flexibility:** Choose between explicit prefixes (module) or direct access (specific)
- **Simplicity:** No wildcards means explicit dependencies
- **Clarity:** Module prefixes prevent namespace pollution
- **Familiarity:** Rust developers will recognize this pattern
- **Balance:** Specific imports for high-frequency items (println, max, min)
- **Explicitness:** Always clear where items come from

**Usage Guidelines:**
```
// ✅ RECOMMENDED: Module prefix for features
import from "http/server";
import from "database/connection";

// ✅ RECOMMENDED: Specific for utilities
import from "std/io/println";
import from "std/math/max";

// ❌ AVOID: Too many specific imports
import from "std/io/println";
import from "std/io/print";
import from "std/io/eprintln";
// Use: import from "std/io" instead
```

**Conflict Resolution:**
```koa
// Module name conflicts
import from "std/io";
import from "mylib/io" as mylib_io;  // Alias required

// Specific item conflicts
import from "std/io/println";
import from "mylib/println" as myprintln;  // Alias required
```

**Re-exports:**
Support all re-export styles:
```koa
pub use io;                    // Re-export module
pub use println;               // Re-export specific item
pub use io as stdio;           // Re-export with alias
pub use println as print_line; // Re-export item with alias
```

**Consequences:**
- Two import styles to learn (but consistent: `/` separator)
- More flexible than pure module-prefix
- More explicit than wildcard imports
- No namespace pollution from `*` imports
- Better code organization: features as modules, utilities as items

**Alternatives Considered:**
- **Module prefix only:** Rejected (too verbose for frequently used items)
- **Named imports only:** Rejected (less explicit, harder conflict resolution)
- **Wildcard imports:** Rejected (namespace pollution, unclear dependencies)
- **ESM-style `{ }`:** Rejected (different from Koa's path-based philosophy)

**Comparison with Other Languages:**
| Language | Module Import | Specific Import | Wildcard |
|----------|--------------|-----------------|----------|
| **Koa**  | `import from "std/io"` | `import from "std/io/println"` | ❌ No |
| **Rust** | `use std::io` | `use std::io::stdout` | `use std::io::*` |
| **Go**   | `import "fmt"` | N/A | N/A |
| **TS**   | N/A | `import { x } from "mod"` | `import * as mod` |

---

## ADR-015: Rust-Style Local Module Resolution (mod.koa Required)

**Status:** Accepted

**Context:**
Koa needs a local module system for organizing code within a project. Options include:
- File-based (TypeScript/Node.js style): `utils.koa`, `math/algebra.koa`
- Rust-style: Directory modules require `mod.koa`
- Mixed: Optional `mod.koa`

**Decision:**
Adopt **Rust-style module resolution** where directory modules **MUST** have a `mod.koa` file.

**Structure:**
```
src/
├── main.koa
├── utils.koa              # File module
└── math/
    ├── mod.koa            (REQUIRED)
    ├── algebra.koa        # Child
    └── calculus.koa       # Child
```

**Rules:**
1. **File modules:** Single `.koa` file = module
   - `src/utils.koa` → `import from "utils"`

2. **Directory modules:** MUST have `mod.koa`
   - `src/math/mod.koa` → `import from "math"`
   - Cannot import children directly

3. **Child modules:** Accessible through parent's `mod.koa`
   - `src/math/algebra.koa` → accessed via `math/mod.koa`

4. **Re-exports:** Parent `mod.koa` re-exports children
   ```koa
   // src/math/mod.koa
   pub from "./algebra"
   pub from "./calculus"

   pub fn add(x: i32, y: i32): i32 { x + y }
   ```

**Resolution Order:**
```
import from "something":

1. library/std/something.koa         (Stdlib file)
2. library/std/something/mod.koa     (Stdlib directory)
3. ~/.koa/packages/*/lib.koa         (External dependencies)
4. src/something.koa                 (Local file)
5. src/something/mod.koa             (Local directory - REQUIRES mod.koa)

First match wins!
```

**Rationale:**
- **Explicit structure:** Clear module boundaries
- **Scalable:** Proven for large projects (Rust ecosystem)
- **Refactoring:** Move files, update `mod.koa`, everything works
- **Tooling:** Compiler can easily track module graph
- **Familiar:** Rust developers feel at home
- **Prevents ambiguity:** No confusion between file and directory modules

**Consequences:**
```koa
// ✅ CORRECT: File module
// src/utils.koa
import from "utils"

// ✅ CORRECT: Directory module with mod.koa
// src/math/mod.koa (REQUIRED)
import from "math"

// ❌ ERROR: Directory without mod.koa
// src/math/algebra.koa exists but NO mod.koa
import from "math"  // ERROR: src/math/mod.koa does not exist

// ❌ ERROR: Cannot import child directly
import from "math/algebra"  // ERROR: must go through math/mod.koa

// ✅ CORRECT: Import through parent
import from "math"
math::solve_linear()  // Re-exported from algebra.koa
```

**Error Handling:**
```
# Missing mod.koa
Error: cannot find module 'math' in src/math/
  → src/math/mod.koa does not exist
  → Hint: create src/math/mod.koa or use file module (math.koa)

# Attempting to import child directly
Error: cannot import child module directly
  → import from "math/algebra"
  → Hint: import from "math" and access via math::<item>
  → Parent module must re-export child in mod.koa
```

**Configuration (Koa.toml):**
```toml
[module]
# Source directory (default: "src")
src = "src"

# Allow root files (default: false)
allow_root = false  # If false, all .koa files must be in src/
```

**Best Practices:**
```
// ✅ GOOD: Logical module hierarchy
src/
├── auth/
│   ├── mod.koa       # Re-exports login, register
│   ├── login.koa
│   └── register.koa
└── database/
    ├── mod.koa       # Re-exports postgres, sqlite
    ├── postgres.koa
    └── sqlite.koa

// ❌ AVOID: Flat structure with many files
src/
├── auth_login.koa
├── auth_register.koa
├── db_postgres.koa
├── db_sqlite.koa
# Harder to navigate and organize
```

**Alternatives Considered:**
- **File-based only:** Rejected (no directory modules, harder to organize)
- **Optional mod.koa:** Rejected (ambiguous, two conventions)
- **Relative paths:** Already rejected in ADR-003
- **Node.js resolution:** Rejected (ambiguous `index.koa`)

**Comparison with Other Languages:**
| Language | File Module | Directory Module | Child Import |
|----------|-------------|------------------|--------------|
| **Koa**  | `utils.koa` | `math/mod.koa` (REQUIRED) | Via `mod.koa` only |
| **Rust** | `utils.rs` | `math/mod.rs` (REQUIRED) | Via `mod.rs` only |
| **Go**   | `utils.go` | `math/` (any `.go` file) | Via package |
| **TS**   | `utils.ts` | `math/index.ts` (optional) | Direct or via index |

---

## Future ADRs

The following decisions may need ADRs in the future:

- ADR-016: Async runtime model (Tokio vs async-std)
- ADR-015: Error handling refinements
- ADR-016: Macro system (if any)
- ADR-017: Const generics
- ADR-018: Module versioning strategy
- ADR-019: FFI ABI design (C ABI vs others)

---

## ADR-011: FFI via On-the-fly Bindgen

**Status:** Accepted

**Context:**
Seamless interoperability with C is a requirement for a modern compiled language. Writing bindings manually is error-prone.

**Decision:**
Implement direct support for `import "header.h"`. The compiler will use `bindgen` (or equivalent) to generate bindings at compile-time.

**Rationale:**
- Frictionless experience for developers (Zig-style).
- leveraging existing robust tooling (`bindgen`) instead of reinventing C parsing.
- Ensures bindings are always up-to-date with headers.

**Consequences:**
```
// main.koa
import "stdio.h"; // Automatic binding

fn main(): i32 {
    unsafe {
        printf("Hello from C!\n");
    }
    return 0;
}
```

---

## ADR-012: Structural Interfaces

**Status:** Accepted

**Context:**
The "No Trait" decision (ADR-009) limits the power of Generics. We need a way to constrain generics.

**Decision:**
Introduce **Interfaces** with Structural Typing (Go-style).

**Rationale:**
- Simpler than Rust Traits (no associated types, lifetimes).
- More flexible than Java Interfaces (no explicit `implements` needed, though allowed).
- Solves the Generic Constraint problem.

**Consequences:**
```
interface Stringer {
    fn to_string(self): string;
}

fn print<T: Stringer>(item: T) {
    println(item.to_string());
}
```

---

## ADR-013: Runtime Memory Safety Model

**Status:** Accepted

**Context:**
Koa aims for simplicity and does not use a Borrow Checker. How do we ensure memory safety in concurrency?

**Decision:**
Adopt a **Runtime Safety** model with optionally active Race Detectors.
- Garbage Collection handles heap safety.
- Race Detector (ThreadSanitizer integration) handles data race detection during testing/debug.
- Encourage `std/channel` for communication.

**Rationale:**
- "Too lazy to be complex" philosophy prefers runtime checks over complex compile-time rules.
- Proven effective by Go.
- Reduces learning curve significantly compared to Rust.

**Consequences:**
- Developers must write tests with `--race` flag.
- Stdlib must provide robust `Mutex` and `Channel` primitives.

---

## How to Add an ADR

1. Create a new section with next number (e.g., ADR-011)
2. Include: Status, Context, Decision, Rationale, Consequences
3. Document alternatives considered
4. Update this summary

---

## References

- [Michael Nygard's Architecture Decision Records](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [Rust ADRs](https://github.com/rust-lang/rfcs/blob/master/0000-architectural-decisions.md)
- [Go Proposals](https://go.googlesource.com/proposal/+/refs/heads/master/design/)
