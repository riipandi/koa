.PHONY: all clean test watch run install help fmt check build debug release example

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

# Build the compiler (Rust)
build:
	cargo build

# Build in debug mode (Koa programs)
debug: build
	@mkdir -p $(DEBUG_DIR)
	cargo run --bin koa -- build --debug examples/hello.koa --output $(DEBUG_DIR)/hello

# Build in release mode (Koa programs)
release: build
	@mkdir -p $(RELEASE_DIR)
	cargo run --bin koa -- build --release examples/hello.koa --output $(RELEASE_DIR)/hello

# Run tests
test:
	cargo test --all

# Watch for file changes (HMR)
watch:
	cargo watch -x 'clear && cargo build'

# Run the CLI directly
run:
	cargo run --bin koa -- --help

# Install to ~/.local/bin (for development)
install:
	cargo install --path .

# Clean build artifacts
clean:
	cargo clean
	rm -rf $(BUILD_DIR)

# Format code
fmt:
	cargo fmt

# Check code without building
check:
	cargo check --all

# Run linter
clippy:
	cargo clippy -- -D warnings

# Run all checks
check-all: fmt check clippy

# Generate documentation
docs:
	cargo doc --no-deps --open

# Run example in debug mode
example-hello: debug
	$(DEBUG_DIR)/hello

# Run example in release mode
example-hello-release: release
	$(RELEASE_DIR)/hello

# Show build directories
info:
	@echo "Build Configuration:"
	@echo "  Architecture: $(ARCH)"
	@echo "  Debug output: $(DEBUG_DIR)"
	@echo "  Release output: $(RELEASE_DIR)"
	@echo ""
	@echo "Platform: $(UNAME_S) $(UNAME_M)"

# Show help
help:
	@echo "Available targets:"
	@echo "  make              - Build the Koa compiler"
	@echo "  make debug        - Build example in debug mode"
	@echo "  make release      - Build example in release mode"
	@echo "  make test         - Run all tests"
	@echo "  make watch        - Watch for changes"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make fmt          - Format code"
	@echo "  make check        - Check code (no build)"
	@echo "  make clippy       - Run linter"
	@echo "  make check-all    - Run all checks (fmt + clippy)"
	@echo "  make docs         - Generate documentation"
	@echo "  make install      - Install to ~/.local/bin"
	@echo "  make info         - Show build configuration"
	@echo "  make help         - Show this help message"

# Development workflow
dev: fmt check
