# Changelog

All notable changes to the Koa programming language will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - LLVM Codegen Integration (2026-02-07)

#### LLVM Code Generator
- Type tracking for local variables and temporaries
- Proper type-aware load instructions (fixes float/int bug)
- Support for specialized generic functions in LLVM IR
- Support for generic struct instantiation

#### Testing
- Added test_generic_function_llvm - verifies generic function compilation
- Added test_generic_struct_llvm - verifies generic struct compilation
- Added test_multiple_generic_instantiations - verifies multiple specializations
- Added test_generic_with_constraints - verifies interface constraints
- Added test_debug_ir_output - for debugging IR output
- Total test suite: 48 tests passing (up from 40)

#### Bug Fixes
- Fixed hardcoded i32 type in load instructions
- Fixed missing type tracking for parameters
- Fixed generic struct instantiation in IR lowering

### Added - Phase 2: Generics & Interfaces (2026-02-07)

#### Parser
- Generic type parameter syntax (`<T>`, `<T: Constraint>`, `<T: A + B>`)
- Generic function call parsing with type arguments (`identity<i32>(42)`)
- Generic struct instantiation parsing (`Box<i32> { value: 42 }`)
- Backtracking logic to disambiguate `<` operator from generic type arguments
- `peek()` method for lookahead without consuming tokens

#### Type System
- Full generic type parameters for functions and structs
- Structural interface declarations with method signatures
- Generic constraints support (`T: Interface`)
- Multiple constraints with `+` operator (`T: A + B`)
- Type substitution algorithm for generic instantiation
- Interface satisfaction checking (automatic verification)
- Constraint validation during generic instantiation

#### IR & Monomorphization
- Monomorphization system for zero-cost generics
- Function specialization with name mangling (e.g., `identity<I32>`)
- Struct specialization for each type instantiation
- On-demand code generation during IR lowering
- Caching mechanism to avoid duplicate specializations
- Type substitution during IR lowering

#### Testing
- 7 generic-specific tests (parsing, type checking)
- 2 monomorphization tests (functions, structs)
- 2 interface satisfaction tests (success, failure cases)
- Total test suite: 40 tests passing

#### Documentation
- Updated `03-type-system.md` with generics and interfaces implementation
- Updated `10-implementation-plan.md` with Phase 2 completion status
- Updated `SUMMARY.md` with current progress
- New `CHANGELOG.md` for tracking changes

### Changed

#### Parser
- Enhanced `parse_postfix_expr()` to handle generic function calls
- Enhanced `parse_primary_expr()` to handle generic struct instantiation
- Improved error messages for generic syntax errors

#### Type Checker
- `check_expression()` now handles generic struct expressions
- `check_call_expr()` now validates type arguments and constraints
- `is_assignable()` now checks interface satisfaction
- Added `substitute_type()` for recursive type substitution
- Added `check_constraints()` for validating generic constraints
- Added `satisfies_interface()` for interface implementation verification

#### IR Lowering
- `IrLowerer` now includes monomorphization state
- `lower()` method now handles specialization
- `lower_type()` now triggers struct specialization
- `lower_expression()` now triggers function specialization

### Fixed
- Parser correctly disambiguates `<` in comparisons vs. generics
- Type checker properly substitutes generic parameters
- IR lowering correctly generates specialized function names

## [0.0.1] - 2026-01-XX (Phase 1)

### Added - Phase 1: Core Compiler

#### Lexer
- Token recognition for all Koa keywords
- Support for integers, floats, strings, booleans
- Comment handling (single-line `//` and multi-line `/* */`)
- Operator tokenization
- Span tracking for error reporting

#### Parser
- Full AST generation for Koa syntax
- Function declarations with parameters and return types
- Struct declarations with fields and methods
- Enum declarations with variants
- Interface declarations (basic)
- Expression parsing (binary, unary, literals, calls, etc.)
- Statement parsing (let, const, return, if, while, loop, etc.)
- Type annotations

#### Type Checker
- Static type checking for all expressions
- Function signature validation
- Struct field type checking
- Type compatibility checking
- Symbol table management with scoping
- Error reporting with source locations

#### IR Generation
- Intermediate representation (IR) for Koa programs
- Function lowering to IR
- Expression lowering to IR
- Basic instruction set (Add, Sub, Mul, Div, Call, etc.)
- Type lowering to IR types

#### LLVM Backend
- Basic LLVM IR generation
- Function compilation
- Simple expression compilation
- Module creation and verification

#### Testing
- 29 tests covering lexer, parser, type checker, and IR
- Integration tests for end-to-end compilation

#### Documentation
- Complete language specification
- Syntax guide
- Type system documentation
- Error handling guide
- Memory management guide
- Concurrency guide
- Module system guide
- Implementation plan
- Architecture decisions

### Infrastructure
- Cargo workspace setup
- `koa` compiler library
- `koa-cli` command-line interface
- `koa-runtime` runtime library (stub)
- Makefile for common tasks
- CI/CD setup (basic)

---

## Release Notes

### Phase 2 Highlights (2026-02-07)

**Generics & Interfaces** - The core implementation is now complete!

Koa now supports:
- **Generic Functions**: Write once, use with any type
- **Generic Structs**: Type-safe containers and data structures
- **Structural Interfaces**: Define behavior contracts
- **Monomorphization**: Zero-cost abstractions with compile-time specialization
- **Interface Satisfaction**: Automatic verification of interface implementation

Example:
```koa
interface Printable {
    fn print(self): void;
}

struct Book {
    title: string;
    fn print(self): void {
        println!("Book: {}", self.title);
        return;
    }
}

fn show<T: Printable>(item: T): void {
    item.print();
    return;
}

let book: Book = Book { title: "Koa Guide" };
show<Book>(book);  // ✅ Works!
```

**What's Next?**
- LLVM codegen integration for generics
- Type inference for local variables
- Generic enums
- Performance optimizations
