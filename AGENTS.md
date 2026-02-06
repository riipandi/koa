# AGENTS.md

This file contains guidelines and commands for agentic coding assistants working on the Koa compiler.

## Build Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build
cargo build -p koa             # Build specific crate

# Test
cargo test                     # All tests
cargo test -p koa              # Test specific crate
cargo test test_name           # Run SINGLE test (most common)
cargo test lexer::tests        # Test specific module
cargo test -- --nocapture      # Show test output
cargo test -- --backtrace      # With backtrace

# Lint/Format
cargo fmt                      # Format code
cargo fmt --check              # Check formatting
cargo clippy -- -D warnings    # Run linter

# Other
cargo check                    # Check without building
cargo clean                    # Clean artifacts
cargo doc --no-deps --open     # Generate docs
```

## Code Style Guidelines

**Rust Setup**: Rust 2024 edition. Always run `cargo fmt` and fix `cargo clippy` before committing.

**Imports**: Sort alphabetically, separate std/third-party/internal with blank lines, avoid `use super::*;`

**Naming**: `PascalCase` for types, `snake_case` for functions, `SCREAMING_SNAKE_CASE` for constants, keep acronyms capitalized (`AST`, `LLVM`)

**Error Handling**: Use `miette::Result<T>`, `.into_diagnostic()` wrapper, `bail!()` for early errors, implement `Diagnostic` trait with source spans

**Documentation**: All public items MUST have docs with `///`, include Examples, document Errors/Panics sections

**Testing**: Name tests `test_<what>_<condition>_<expected>`, use `unwrap()` for success, `assert_eq!()` for checks

**Performance**: Use `&str` over `String`, `Cow<'_, str>` for conditional allocation, `Box<T>` for large types, `#[inline]` for small functions

**Dependencies**: Use workspace dependencies (e.g., `miette.workspace = true`)

## Compiler-Specific Guidelines

### Lexer (crates/koa/src/lexer/)
- Return `Result<Vec<Token>>`
- Include source location (line, column) in each token
- Handle UTF-8 correctly

### Parser (crates/koa/src/parser/)
- Return `Result<Ast>` with miette diagnostics
- Provide error recovery (don't stop at first error)
- Use lalrpop for grammar definition
- Report errors with source spans

### Type Checker (crates/koa/src/typeck/)
- Track type environment per scope
- Provide detailed type mismatch errors
- Support generic monomorphization

### LLVM Gen (crates/koa/src/llvm_gen/)
- Use inkwell for LLVM bindings
- Generate IR that preserves source locations for debugging
- Support multiple optimization levels

### CLI (crates/koa-cli/)
- Use clap derive macros for subcommands
- Provide helpful error messages with hints
- Support `--verbose` and `--quiet` flags

## Common Patterns

```rust
// Builder pattern
impl LexerBuilder {
    pub fn new(input: impl Into<String>) -> Self {
        Self { input: input.into(), keep_comments: false }
    }
    pub fn keep_comments(mut self, yes: bool) -> Self {
        self.keep_comments = yes;
        self
    }
}

// From/Into conversions
impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Self {
        Self::new(kind, Span::default())
    }
}

// Default implementations
impl Default for Lexer {
    fn default() -> Self {
        Self::new("")
    }
}
```

## Development Workflow

1. Run `cargo test` to ensure baseline passes
2. Edit files, run `cargo build` frequently
3. Use `cargo test test_name` for rapid iteration
4. Run `cargo fmt` before committing
5. Run `cargo clippy` and fix warnings
6. Final check: `cargo test && cargo clippy -- -D warnings`

## File Organization

```
crates/koa/src/
├── lib.rs              # Public API re-exports
├── lexer/              # Tokenization
├── parser/             # AST generation
├── ast/                # AST definitions
├── typeck/             # Type checker
├── ir/                 # Intermediate representation
└── llvm_gen/           # LLVM IR generation

crates/koa-cli/src/
└── main.rs             # CLI entry point

crates/koa-runtime/src/
├── lib.rs              # Runtime API
├── gc/                 # Garbage collector
├── async/              # Async runtime
└── alloc/              # Memory allocator
```

## Questions?

1. Check existing code for examples
2. Consult project README and documentation
3. Ask: What would similar Rust compilers (rustc, rust-analyzer) do?
