# AGENTS.md

This file contains guidelines and commands for agentic coding assistants working on the Koa compiler.

## Build Commands

```bash
# Build
cargo build                    # Debug build (all workspace)
cargo build --release          # Release build
cargo build -p koa             # Build specific crate
make build                     # Build using Makefile
make build-koa                 # Build compiler library only
make build-cli                 # Build CLI tool only

# Test
cargo test                     # All tests
cargo test -p koa              # Test specific crate
cargo test test_name           # Run SINGLE test by exact name
cargo test lexer::tests        # Test specific module
cargo test -- --nocapture      # Show test output
cargo test -- --backtrace      # With backtrace
make test                      # Run all tests via Makefile
make test-koa                  # Test compiler library

# Lint/Format
cargo fmt                      # Format code
cargo fmt --check              # Check formatting
cargo clippy -- -D warnings    # Run linter (warnings as errors)
make fmt                       # Format via Makefile
make clippy                    # Lint via Makefile
make check-all                 # Run fmt + check + clippy

# Other
cargo check                    # Check without building
cargo clean                    # Clean artifacts
cargo doc --no-deps --open     # Generate and open docs
make install                   # Install CLI to ~/.local/bin
```

## Code Style Guidelines

**Rust Edition**: 2024. Always run `cargo fmt` and fix `cargo clippy` before committing.

**Import Organization**:
- Group 1: std imports (`use std::...`)
- Group 2: Third-party crates (`use miette::...`)
- Group 3: Internal imports (`use crate::...`)
- Sort alphabetically within each group
- Separate groups with blank lines

**Example**:
```rust
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use miette::{IntoDiagnostic, Result};
use inkwell::context::Context;

use crate::ast::*;
use crate::lexer::{Lexer, TokenKind};
```

**Naming Conventions**:
- Types: `PascalCase` (e.g., `IrProgram`, `LLVMCodegen`)
- Functions: `snake_case` (e.g., `parse_fn_decl`, `codegen_global`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `VERSION`)
- Acronyms: Keep capitalized (e.g., `AST`, `LLVM`, `IR`)
- Private fields: `snake_case` (e.g., `current_function`)

**Error Handling**:
- Use `miette::Result<T>` as return type
- Use `.into_diagnostic()?` to wrap external errors
- Use `miette::miette!("message")` for custom errors
- Avoid `bail!` macro (not used in this codebase)
- Implement `Diagnostic` trait for custom error types with source spans

**Example**:
```rust
use miette::{IntoDiagnostic, Result};

fn example() -> Result<()> {
    let file = File::open("path").into_diagnostic()?;
    let value = parse(&file)?;
    Ok(())
}
```

**Documentation**:
- All public items MUST have module-level docs with `//!`
- Use `///` for item documentation
- Include Examples sections where applicable
- Document Errors and Panics sections for functions
- Keep docs concise but informative
- **All documentation and code comments MUST be in English**
- Progress notes should be as concise as possible

**Testing**:
- Name tests descriptively: `test_<feature>_<scenario>` or `test_<what>`
- Use `unwrap()` for expected success cases
- Use `assert_eq!()` for value comparisons
- Use `assert!()` for boolean conditions
- Keep tests focused and independent
- **AVOID `unwrap()` in production code** - use proper error handling with `?` or `miette::Result`

**Performance**:
- Use `&str` over `String` for function parameters where possible
- Use `Cow<'_, str>` for conditional string allocation
- Use `Box<T>` for large types to reduce stack usage
- Use `#[inline]` for small, frequently-called functions
- Prefer iterators over `collect()` where possible

**Dependencies**:
- Use workspace dependencies: `miette.workspace = true`
- For CLI-specific deps, specify versions in `koa-cli/Cargo.toml`
- Keep dependencies minimal and well-justified

## Compiler-Specific Guidelines

### Lexer (`crates/koa/src/lexer/`)
- Return `Result<Vec<Token>>` from `tokenize()`
- Include source location (line, column, start, end) in each token
- Handle UTF-8 correctly
- Skip whitespace, line comments (`//`), doc comments (`///`), and block comments (`/* */`)
- Track position, line, and column in lexer state

### Parser (`crates/koa/src/parser/`)
- Return `Result<Ast>` with miette diagnostics
- Token consumption: `self.consume_token(TokenKind::Fn)?`
- Token peeking: `self.peek_kind()` returns `Option<TokenKind>`
- Error messages: `miette::miette!("Expected {}", "token")`
- Build AST with proper spans for error reporting

### IR (`crates/koa/src/ir/`)
- Simplify AST for code generation
- Use explicit types (no type inference in IR)
- Lower control flow to basic blocks
- Separate declarations from instructions

### LLVM Gen (`crates/koa/src/llvm_gen/`)
- Use inkwell for LLVM bindings (version 0.4.0, LLVM 15)
- Preserve source locations for debugging
- Use `.into_diagnostic()?` on LLVM builder calls
- Declare external functions (printf, etc.) before use
- Support multiple optimization levels via LLVM passes

### CLI (`crates/koa-cli/`)
- Use clap derive macros for subcommands
- Use `indicatif` for progress bars and spinners
- Use `log` crate with `env_logger` for logging
- Provide helpful error messages with hints
- Support `--verbose` for debug output

## Common Patterns

**Builder Pattern**:
```rust
impl LLVMCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self { context, module, builder, functions: HashMap::new(), locals: HashMap::new(), current_function: None }
    }
}
```

**Token Pattern**:
```rust
fn consume_token(&mut self, kind: TokenKind) -> Result<Token> {
    let token = self.tokens.get(self.position)
        .ok_or_else(|| miette::miette!("Unexpected EOF"))?;
    if token.kind != kind {
        miette::bail!("Expected {:?}, found {:?}", kind, token.kind);
    }
    self.position += 1;
    Ok(token.clone())
}
```

**Default Implementation**:
```rust
impl Default for Lexer<'_> {
    fn default() -> Self {
        Self::new("")
    }
}
```

## Development Workflow

1. Run `cargo test` to ensure baseline passes
2. Edit files, run `cargo build` frequently to catch errors early
3. Use `cargo test test_name` for rapid test iteration
4. Run `cargo fmt` before committing
5. Run `cargo clippy -- -D warnings` and fix all warnings
6. Final check: `cargo test && cargo clippy -- -D warnings`

## File Organization

```
crates/koa/src/
├── lib.rs              # Public API re-exports, module declarations
├── lexer/              # Tokenization (mod.rs with tests)
├── parser/             # AST generation from tokens
├── ast/                # AST node definitions (FnDecl, Statement, etc.)
├── typeck/             # Type checking (stubs)
├── ir/                 # Intermediate representation
└── llvm_gen/           # LLVM IR generation (inkwell bindings)

crates/koa-cli/src/
└── main.rs             # CLI entry point with clap commands

crates/koa-runtime/src/
├── lib.rs              # Runtime API (stubs)
├── gc/                 # Garbage collector (not implemented)
├── async/              # Async runtime (not implemented)
└── alloc/              # Memory allocator (not implemented)

library/std/            # Standard library source files
examples/               # Example Koa programs
docs/                   # Documentation
```

## Important Notes

- **Comment handling**: Lexer now properly skips `//`, `///`, `//!`, and `/* */` comments
- **LLVM version**: Using inkwell 0.4.0 with LLVM 15
- **Workspace dependencies**: Defined in root `Cargo.toml`
- **Testing**: All tests pass, use `cargo test -p koa` for compiler tests
- **Parser grammar**: Hand-written recursive descent (not using lalrpop yet)

## Questions?

1. Check existing code in `crates/koa/src/` for patterns
2. Consult documentation in `docs/` directory
3. Look at test cases for usage examples
4. Follow the established patterns in similar modules
5. When in doubt, ask: What would rustc/rust-analyzer do?

## Documentation Guidelines

**DO NOT** create separate markdown files for progress reports or summaries.

**Use these files for documentation**:

1. **`docs/SUMMARY.md`** - Overall project status and milestones
   - Update implementation status
   - Track milestone progress
   - Add recent updates section

2. **`CHANGELOG.md`** - Detailed change tracking
   - Follow [Keep a Changelog](https://keepachangelog.com/) format
   - Document all Added/Changed/Fixed/Removed items
   - Include version numbers and dates

3. **`KNOWN_ISSUES.md`** - Track bugs and issues
   - Document all discovered issues with descriptions
   - Mark fixed issues with (FIXED ✓) and move to "Fixed Issues" section
   - Keep outstanding issues in "Outstanding Issues" section
   - Include workarounds when available

4. **`docs/*.md`** - Feature-specific documentation
   - `docs/03-type-system.md` - Type system features
   - `docs/10-implementation-plan.md` - Implementation roadmap
   - Update existing docs, don't create new ones

**Examples of what NOT to do**:
- ❌ `docs/PHASE2_SUMMARY.md`
- ❌ `docs/NEXT_STEPS.md`
- ❌ `docs/PROGRESS_REPORT.md`
- ❌ Any standalone progress/summary documents

**Where to document**:
- ✅ Progress updates → `docs/SUMMARY.md` (Recent Updates section)
- ✅ Changes → `CHANGELOG.md` (Unreleased section)
- ✅ Issues found → `KNOWN_ISSUES.md` (Outstanding Issues section)
- ✅ Implementation details → Update existing `docs/*.md` files
- ✅ Next steps → `docs/10-implementation-plan.md` (Next Steps section)
