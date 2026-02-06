# Cross Compilation

Koa supports cross-compilation to different target platforms via LLVM.

---

## Supported Targets

### Tier 1 Platforms (Fully Supported)

| Host | Target | Status |
|------|--------|--------|
| macOS (x86_64) | macOS (x86_64, arm64) | ✅ |
| macOS (x86_64) | Linux (x86_64) | ✅ |
| macOS (x86_64) | Windows (x86_64) | ✅ |
| macOS (arm64) | macOS (arm64, x86_64) | ✅ |
| Linux (x86_64) | Linux (x86_64, arm64) | ✅ |
| Linux (x86_64) | macOS (x86_64, arm64) | ✅ |
| Linux (x86_64) | Windows (x86_64) | ✅ |
| Windows (x86_64) | Windows (x86_64) | ✅ |
| Windows (x86_64) | Linux (x86_64) | ✅ |

### Tier 2 Platforms (Best Effort)

| Host | Target | Status |
|------|--------|--------|
| Linux (x86_64) | Android (arm64) | ⚠️ |
| Linux (x86_64) | FreeBSD (x86_64) | ⚠️ |
| Any | WebAssembly (wasm32-wasi) | ⚠️ |

### Tier 3 Platforms (Not Supported)

- Embedded systems with < 1MB RAM (use C)
- Bare metal (use Rust or C)
- iOS/App Store (Apple restrictions)
- Nintendo Switch, PlayStation, Xbox (proprietary SDKs)

---

## Target Triples

Targets use LLVM-style triples:

```
<arch>-<vendor>-<os>-<env>
```

**Examples:**

```
x86_64-unknown-linux-gnu      # Linux on x86_64
aarch64-apple-darwin          # macOS on ARM (Apple Silicon)
x86_64-pc-windows-msvc        # Windows with MSVC
wasm32-wasi                   # WebAssembly
armv7-unknown-linux-gnueabihf # Linux on ARMv7 (Raspberry Pi)
```

---

## Basic Usage

### Compile for Different Target

```bash
# Compile for Linux from macOS
koa build --target x86_64-unknown-linux-gnu

# Compile for Windows from Linux
koa build --target x86_64-pc-windows-msvc

# Compile for macOS ARM from x86_64
koa build --target aarch64-apple-darwin
```

### Set Default Target in Koa.toml

```toml
[build]
target = "x86_64-unknown-linux-gnu"
mode = "release"
```

---

## Prerequisites

### 1. LLVM Toolchain

LLVM must support the target platform:

```bash
# List available targets
clang --print-targets

# Check if specific target is available
clang --print-targets | grep x86_64-unknown-linux-gnu
```

### 2. System Root (for some targets)

For cross-compilation to different OS, you may need a sysroot:

```bash
# Example: Cross-compile from macOS to Linux
brew install x86_64-unknown-linux-gnu

# Set environment variable
export KOA_SYSROOT=/usr/local/x86_64-unknown-linux-gnu
```

### 3. Standard Library

The standard library must be compiled for the target:

```bash
# Build stdlib for target
koa build --target x86_64-unknown-linux-gnu --std
```

---

## Advanced Usage

### Custom Linker

```bash
# Use custom linker
koa build --target x86_64-unknown-linux-gnu \
  --linker=x86_64-linux-gnu-gcc
```

### Cross-Compilation Environment Variables

| Variable | Description |
|----------|-------------|
| `KOA_TARGET` | Override target triple |
| `KOA_SYSROOT` | System root for target |
| `KOA_LINKER` | Custom linker |
| `KOA_AR` | Custom ar (archiver) |
| `KOA_CFLAGS` | Additional C compiler flags |

**Example:**

```bash
export KOA_TARGET=x86_64-unknown-linux-gnu
export KOA_SYSROOT=/usr/local/x86_64-unknown-linux-gnu
export KOA_LINKER=x86_64-linux-gnu-gcc
koa build
```

---

## Platform-Specific Notes

### macOS → Linux

**Requirements:**
- Xcode command-line tools
- Linux sysroot (optional)

```bash
# Install sysroot (optional)
brew install x86_64-unknown-linux-gnu

# Cross-compile
koa build --target x86_64-unknown-linux-gnu
```

### macOS → Windows

**Requirements:**
- MinGW-w64

```bash
# Install MinGW
brew install mingw-w64

# Cross-compile
koa build --target x86_64-pc-windows-msvc
```

### Linux → macOS

**Requirements:**
- macOS SDK (from Xcode)
- `osxcross` toolchain

```bash
# Install osxcross
git clone https://github.com/tpoechtrager/osxcross
cd osxcross
./build.sh

# Cross-compile
export KOA_SYSROOT=/path/to/osxcross/target/sdk
koa build --target x86_64-apple-darwin
```

### Linux → Windows

**Requirements:**
- MinGW-w64

```bash
# Install MinGW
sudo apt-get install mingw-w64

# Cross-compile
koa build --target x86_64-pc-windows-msvc
```

### Any → WebAssembly

**Requirements:**
- WASI SDK

```bash
# Install WASI SDK
git clone https://github.com/WebAssembly/wasi-sdk
export WASI_SDK_PATH=/path/to/wasi-sdk

# Cross-compile
export KOA_TARGET=wasm32-wasi
export KOA_SYSROOT=$WASI_SDK_PATH/share/sysroot
koa build
```

---

## Conditional Compilation

Code can be conditionally compiled based on target:

```
// Linux-specific code
[@target(os = "linux")]
pub fn get_config_path(): string {
    return "/etc/myapp/config.koa";
}

// macOS-specific code
[@target(os = "macos")]
pub fn get_config_path(): string {
    return "/Library/Application Support/myapp/config.koa";
}

// Windows-specific code
[@target(os = "windows")]
pub fn get_config_path(): string {
    return "C:\\ProgramData\\myapp\\config.koa";
}

// Fallback for other platforms
pub fn get_config_path(): string {
    return "./config.koa";
}
```

**Available annotations:**

```
[@target(os = "linux")]
[@target(os = "macos")]
[@target(os = "windows")]
[@target(arch = "x86_64")]
[@target(arch = "aarch64")]
[@target(arch = "arm")]
[@target(endian = "little")]
[@target(endian = "big")]
```

---

## Foreign Function Interface (FFI)

Cross-compilation affects FFI:

```
// C library header
// extern int add(int a, int b);

// Koa FFI declaration
[@link("mymath")]
extern {
    fn add(a: i32, b: i32): i32;
}

// Usage
fn main(): i32 {
    return add(1, 2);
}
```

**Cross-compilation:**

```bash
# Link against platform-specific library
koa build --target x86_64-unknown-linux-gnu \
  --link-args="-L/usr/local/lib -lmymath"
```

---

## Static Linking

For portable binaries, use static linking:

```toml
[build]
target = "x86_64-unknown-linux-gnu"
mode = "release"
static-linking = true
```

**Or via command line:**

```bash
koa build --target x86_64-unknown-linux-gnu --static
```

**Result:** Single binary with no runtime dependencies.

---

## Dynamic Linking

For smaller binaries, use dynamic linking:

```bash
# Default (dynamic linking)
koa build --target x86_64-unknown-linux-gnu
```

**Result:** Binary that links against shared libraries (.so, .dll, .dylib).

---

## Cross-Compilation Cache

Cross-compiled artifacts are cached separately:

```
.koa/cache/
├── x86_64-unknown-linux-gnu/
│   └── build/
├── aarch64-apple-darwin/
│   └── build/
└── x86_64-pc-windows-msvc/
    └── build/
```

**Clear cache for specific target:**

```bash
koa cache clean --target x86_64-unknown-linux-gnu
```

---

## Testing Cross-Compiled Binaries

### Using Docker

```bash
# Test Linux binary on macOS
docker run --rm -v ./build:/mnt ubuntu:latest /mnt/myapp
```

### Using QEMU

```bash
# Test ARM binary on x86_64
qemu-aarch64 ./build/myapp
```

### Using Wine (for Windows binaries)

```bash
# Test Windows binary on Linux
wine ./build/myapp.exe
```

---

## Common Issues

### Issue: "target not found"

**Solution:** Install LLVM toolchain for target

```bash
# macOS
brew install llvm

# Linux
sudo apt-get install llvm-dev
```

### Issue: "cannot find -lc"

**Solution:** Install sysroot or use static linking

```bash
# Use static linking
koa build --target x86_64-unknown-linux-gnu --static
```

### Issue: "standard library not compiled for target"

**Solution:** Build stdlib for target

```bash
koa build --target x86_64-unknown-linux-gnu --std
```

---

## Best Practices

1. **Test on actual hardware** - Emulation (QEMU) isn't perfect
2. **Use CI/CD** - Test on all target platforms
3. **Static linking for portability** - Fewer runtime dependencies
4. **Conditional compilation sparingly** - Most code should be platform-agnostic
5. **Document platform requirements** - Clearly state supported platforms in README

---

## Examples

### Cross-Compile CLI Tool

```bash
# Build for all platforms
for target in x86_64-unknown-linux-gnu x86_64-pc-windows-msvc aarch64-apple-darwin; do
  koa build --target $target --mode release --static
  mv build/myapp build/myapp-$target
done

# Result:
# build/myapp-x86_64-unknown-linux-gnu
# build/myapp-x86_64-pc-windows-msvc.exe
# build/myapp-aarch64-apple-darwin
```

### Cross-Compile WebAssembly Module

```bash
# Build WASM module
koa build --target wasm32-wasi --mode release

# Run with wasmtime
wasmtime build/myapp.wasm
```

---

## See Also

- [Build System](16-build-system.md) - Koa.toml configuration
- [FFI](13-ffi.md) - C interop
- [Conditional Compilation](08-conditional-compilation.md) - Platform-specific code
