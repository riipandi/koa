.PHONY: all build clean test watch run install fmt check clippy docs help
.PHONY: build-koa build-cli test-koa test-cli

# Default target
all: build

# Build the compiler (all workspace members)
build:
	cargo build --workspace

# Build only the compiler library
build-koa:
	cargo build -p koa

# Build only the CLI
build-cli:
	cargo build -p koa-cli

# Build in release mode
build-release:
	cargo build --workspace --release

# Run tests
test:
	cargo test --workspace

# Run tests for specific crates
test-koa:
	cargo test -p koa

test-cli:
	cargo test -p koa-cli

# Watch for file changes
watch:
	cargo watch -x 'clear && cargo build --workspace'

# Run the CLI directly
run:
	cargo run -p koa-cli -- --help

# Show CLI version
version:
	cargo run -p koa-cli -- --version

# Install the CLI using cargo
install:
	cargo install --path crates/koa-cli

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt --all

# Check code without building
check:
	cargo check --workspace --all-targets

# Run linter
clippy:
	cargo clippy --workspace -- -D warnings

# Run all checks
check-all: fmt check clippy

# Generate documentation
docs:
	cargo doc --no-deps --workspace --open

# Development/Example shortcuts
example: build-cli
	cargo run -p koa-cli -- build examples/hello_world.koa

# Show help
help:
	@echo "Koa Compiler - Makefile Targets"
	@echo ""
	@echo "Building:"
	@echo "  make build         - Build all workspace members"
	@echo "  make build-release - Build in release mode"
	@echo "  make build-koa     - Build compiler library only"
	@echo "  make build-cli     - Build CLI tool only"
	@echo ""
	@echo "Testing:"
	@echo "  make test          - Run all tests"
	@echo "  make test-koa      - Test compiler library"
	@echo "  make test-cli      - Test CLI tool"
	@echo ""
	@echo "Development:"
	@echo "  make run           - Run CLI help"
	@echo "  make watch         - Watch for changes"
	@echo "  make example       - Build example project"
	@echo "  make install       - Install CLI using cargo"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt           - Format code"
	@echo "  make check         - Check code (no build)"
	@echo "  make clippy        - Run linter"
	@echo "  make check-all     - Run all checks"
	@echo "  make docs          - Generate documentation"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make help          - Show this help message"
