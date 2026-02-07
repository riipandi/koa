# Basic Koa Project Example

A comprehensive example demonstrating the Koa programming language's core features.

## 🎯 Features Demonstrated

This example showcases:

- **Functions**: Multiple function definitions with parameters and return values
- **Variables**: Type annotations (i32, f64, string, bool)
- **Control Flow**: if-else statements and while loops
- **Arithmetic**: Basic operations (+, -, *, /, %)
- **Comparisons**: Boolean expressions and comparisons
- **Recursion**: Factorial and Fibonacci functions
- **Standard Library**: Using `puts` and `printf` for output

## 📁 Project Structure

```
examples/basic/
├── Koa.toml      # Project configuration
├── README.md     # This file
└── src/
    └── main.koa  # Complete example with demos
```

## 🚀 Getting Started

### Build the Project

```bash
# From the project root directory
cd examples/basic

# Build the project
koa build

# Or build directly from root
koa build src/main.koa --cwd examples/basic
```

### Run the Project

```bash
# Run the compiled executable
cd examples/basic
./src/main

# Or use koa run
koa run --cwd examples/basic
```

## 📚 Code Overview

### Main Function

The `main()` function serves as the entry point and demonstrates various language features:

```koa
fn main(): i32 {
    puts("Koa Language Demo");
    demo_arithmetic();
    demo_functions();
    return 0;
}
```

### Function Definitions

Functions are defined with explicit parameter and return types:

```koa
fn add(x: i32, y: i32): i32 {
    return x + y;
}
```

### Variables

Variables are declared with type annotations:

```koa
let a: i32 = 10;
let b: i32 = 5;
let flag: bool = true;
```

### Control Flow

**If-Else Statements:**
```koa
if x > 5 {
    puts("x is greater");
} else {
    puts("x is not greater");
}
```

**While Loops:**
```koa
let i: i32 = 1;
while i <= 3 {
    printf("Count: %d\n", i);
    i = i + 1;
}
```

### Recursion

Functions can call themselves:

```koa
fn factorial(n: i32): i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
```

## 📊 Expected Output

When you run the program, you should see:

```
========================================
Koa Language Demo
========================================

--- Arithmetic ---
10 + 5 = 15
10 * 5 = 50

--- Functions ---
factorial(5) = 120
fibonacci(10) = 55

========================================
Demo Complete!
========================================
```

## 🎓 Language Concepts

### Type System

Koa is statically typed with explicit type annotations:

- `i32` - 32-bit integer
- `f64` - 64-bit floating point
- `string` - String type
- `bool` - Boolean type

### Function Signatures

All functions have explicit parameter and return types:

```koa
fn function_name(param1: type1, param2: type2): return_type {
    // function body
    return value;
}
```

### Variable Declarations

Variables are declared using `let` with type annotations:

```koa
let variable_name: type = value;
```

### Return Statements

The `return` keyword is **always required**:

```koa
return value;  // Explicit return
```

## ⚠️ Known Limitations

1. **String Escapes**: Escape sequences like `\n` are printed literally. Use `puts()` for newlines or separate `printf()` calls.

2. **Module System**: Local modules (math, utils, types) are created but not yet importable due to current import resolution limitations.

3. **Loop Statement**: The `loop` statement is not yet supported in the parser.

## 📖 Further Reading

- [Koa Syntax Guide](../../docs/02-syntax-guide.md)
- [Type System Documentation](../../docs/03-type-system.md)
- [Module System Guide](../../docs/07-modules.md)
- [Implementation Plan](../../docs/10-implementation-plan.md)

## 🔧 Project Configuration

The `Koa.toml` file defines project metadata:

```toml
[package]
name = "basic"
version = "0.1.0"
type = "executable"
authors = ["Koa Authors"]
description = "Basic Koa project example"
license = "MIT"

[build]
target = "aarch64-apple-unix"
mode = "debug"
```

## 🎯 Key Takeaways

1. **Type Safety**: All variables and functions have explicit types
2. **Explicit Returns**: Always use the `return` keyword
3. **Semicolons**: Statements end with newlines (semicolons in strings only)
4. **Function Order**: Functions must be declared before they're called (forward declarations not yet supported)
5. **Standard Library**: Use `puts()` and `printf()` for output
6. **No Garbage Collection**: Manual memory management (not yet implemented)

## 🤝 Contributing

This is an example project. For contributions to the Koa language itself, please see the main repository.

## 📄 License

MIT License - See LICENSE file for details
