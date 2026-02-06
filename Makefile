.PHONY: all clean test watch run install help fmt check build debug release example
.PHONY: cli pkg-init pkg-add pkg-list pkg-tree test-examples

# Detect architecture
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ARCH := $(UNAME_M)
ifeq ($(ARCH),x86_64)
	ARCH := x86_64
else ifeq ($(ARCH),aarch64)
	ARCH := aarch64
else ifeq ($(ARCH),arm64)
	ARCH := aarch64
endif

BUILD_DIR := build
DEBUG_DIR := $(BUILD_DIR)/debug/$(ARCH)
RELEASE_DIR := $(BUILD_DIR)/release/$(ARCH)

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

# Run tests for specific crate
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

# Install CLI to ~/.local/bin (for development)
install:
	cargo install --path crates/koa-cli

# Install in release mode
install-release:
	cargo install --path crates/koa-cli --release

# Clean build artifacts
clean:
	cargo clean
	rm -rf $(BUILD_DIR)

# Format code
fmt:
	cargo fmt --all

# Check code without building
check:
	cargo check --workspace --all-targets

check-koa:
	cargo check -p koa --all-targets

check-cli:
	cargo check -p koa-cli --all-targets

# Run linter
clippy:
	cargo clippy --workspace -- -D warnings

# Run all checks
check-all: fmt check clippy

# Generate documentation
docs:
	cargo doc --no-deps --workspace --open

# Build example in debug mode
debug: build-cli
	@mkdir -p $(DEBUG_DIR)
	cargo run -p koa-cli -- build examples/hello_world.koa --output $(DEBUG_DIR)/hello_world
	@echo "Built $(DEBUG_DIR)/hello_world"

# Build example in release mode
release: build-cli
	@mkdir -p $(RELEASE_DIR)
	cargo run -p koa-cli -- build --mode release examples/hello_world.koa --output $(RELEASE_DIR)/hello_world
	@echo "Built $(RELEASE_DIR)/hello_world"

# Run example in debug mode
example-hello: debug
	$(DEBUG_DIR)/hello_world

# Run example in release mode
example-hello-release: release
	$(RELEASE_DIR)/hello_world

# Build simple example
debug-simple: build-cli
	@mkdir -p $(DEBUG_DIR)
	cargo run -p koa-cli -- build examples/simple.koa --output $(DEBUG_DIR)/simple
	@echo "Built $(DEBUG_DIR)/simple"

# Initialize a new project (interactive)
pkg-init:
	cargo run -p koa-cli -- init --interactive

# Initialize a project with name
pkg-init-name:
	@read -p "Project name: " name; \
	cargo run -p koa-cli -- init $$name

# Add dependency
pkg-add:
	@read -p "Package name: " pkg; \
	read -p "Git URL: " url; \
	cargo run -p koa-cli -- pkg add $$pkg --git $$url

# List dependencies
pkg-list:
	cargo run -p koa-cli -- pkg list

# Show dependency tree
pkg-tree:
	cargo run -p koa-cli -- pkg tree

# Fetch dependencies
pkg-fetch:
	cargo run -p koa-cli -- pkg fetch

# Update dependencies
pkg-update:
	cargo run -p koa-cli -- pkg update

# Show build directories
info:
	@echo "Build Configuration:"
	@echo "  Architecture: $(ARCH)"
	@echo "  Debug output: $(DEBUG_DIR)"
	@echo "  Release output: $(RELEASE_DIR)"
	@echo ""
	@echo "Workspace Members:"
	@echo "  - crates/koa (compiler library)"
	@echo "  - crates/koa-cli (CLI tool)"
	@echo "  - crates/koa-runtime (runtime)"
	@echo ""
	@echo "Platform: $(UNAME_S) $(UNAME_M)"

# Development workflow
dev: fmt check

# Run development server (HMR for Koa code)
dev-hmr:
	cargo watch -x 'clear && cargo run -p koa-cli -- build examples/hello.koa'

# Show help
help:
	@echo "Koa Compiler - Makefile Targets"
	@echo ""
	@echo "Building:"
	@echo "  make build         - Build all workspace members"
	@echo "  make build-koa     - Build compiler library only"
	@echo "  make build-cli     - Build CLI tool only"
	@echo "  make build-release - Build in release mode"
	@echo ""
	@echo "Examples:"
	@echo "  make debug         - Build example in debug mode"
	@echo "  make release       - Build example in release mode"
	@echo "  make example-hello - Run example (debug)"
	@echo ""
	@echo "Package Management:"
	@echo "  make pkg-init      - Interactive project init"
	@echo "  make pkg-init-name - Init with project name"
	@echo "  make pkg-add       - Add dependency"
	@echo "  make pkg-list      - List dependencies"
	@echo "  make pkg-tree      - Show dependency tree"
	@echo ""
	@echo "Testing:"
	@echo "  make test          - Run all tests"
	@echo "  make test-koa      - Test compiler library"
	@echo "  make test-cli      - Test CLI tool"
	@echo ""
	@echo "Development:"
	@echo "  make watch         - Watch for changes"
	@echo "  make dev-hmr       - Dev server with HMR"
	@echo "  make run           - Run CLI with --help"
	@echo "  make version       - Show CLI version"
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
	@echo "  make install       - Install CLI to ~/.local/bin"
	@echo "  make install-release - Install in release mode"
	@echo ""
	@echo "Info:"
	@echo "  make info          - Show build configuration"
	@echo "  make help          - Show this help message"
