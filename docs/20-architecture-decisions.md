# Architecture Decisions (ADRs)

This document contains Architecture Decision Records (ADRs) for major design choices in Koa.

---

## ADR-001: snake_case Naming Convention

**Status:** Accepted

**Context:**
Koa targets TypeScript developers who are familiar with camelCase. However, Koa is also a systems programming language.

**Decision:**
Use `snake_case` for variables and functions, `PascalCase` for types.

**Rationale:**
- Consistent with Rust, showing Koa is a systems language
- More readable for multi-word identifiers
- TypeScript developers are already familiar with snake_case from other languages
- Clear distinction between types and values

**Consequences:**
```typescript
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
```typescript
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
```typescript
// ✅ CORRECT
import { serve } from "std/net/http";
import { utils } from "myapp_utils";

// ❌ ERROR
import { utils } from "../utils";
import { utils } from "./helpers";
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
```typescript
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
```typescript
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
```typescript
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
```typescript
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
```typescript
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
```typescript
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

## Future ADRs

The following decisions may need ADRs in the future:

- ADR-011: Trait system design
- ADR-012: Async runtime model (Tokio vs async-std)
- ADR-013: Error handling refinements
- ADR-014: Macro system (if any)
- ADR-015: Const generics
- ADR-016: Module versioning strategy
- ADR-017: Standard library structure
- ADR-018: FFI ABI design (C ABI vs others)

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
