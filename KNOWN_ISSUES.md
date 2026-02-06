# Known Issues

## Fixed Issues

### Comment Bug (FIXED ✓)
The `hello_world.koa` example contains doc comments that previously triggered a lexer bug (infinite loop). This has been fixed by implementing proper comment handling in the lexer.

**Fix:** Added comment detection and skipping in `crates/koa/src/lexer/mod.rs`:
- Line comments (`//`)
- Doc comments (`///` and `//!`)
- Block comments (`/* ... */`)

**Current Status:** The lexer now correctly handles all types of comments. The `hello_world.koa` file can be tokenized successfully.

**Remaining Issue:** The `println!` macro is not yet implemented, so the parser will fail with "Expected Semicolon" error. This is expected and not a bug - it's a feature that needs to be implemented.

---

## Outstanding Issues

### 1. String Escape Sequences (Partially Fixed ✓)
**FIXED:** String literals no longer include surrounding quotes in output (2026-02-07).

**Remaining Issue:** The lexer does not process escape sequences in string literals. Sequences like `\n`, `\t`, `\"`, etc. are treated as literal characters (backslash + letter).

**Example:**
```koa
printf("Hello\nWorld");  // Prints "Hello" followed by literal "\n", then "World"
```

**Workaround:** Use `puts()` which automatically adds a newline:
```koa
puts("Hello, World!");  // Prints "Hello, World!" with newline
```

**Future Implementation:** Add escape sequence processing in `crates/koa/src/lexer/mod.rs` in the `lex_string()` function (line 247-262).

---

### 2. Struct Field Access
Struct field access (`object.field`) may fail with "Local variable not found: t0" error in IR lowering. This is a bug in the member expression handling in the IR lowerer.

**Example:**
```koa
struct Point { x: i32; y: i32; }
fn main(): i32 {
    let p = Point { x: 1, y: 2 };
    return p.x;  // May fail with "Local variable not found: t0"
}
```

**Workaround:** None currently. Need to fix the IR lowering for member expressions.

**Future Implementation:** Fix `lower_expression()` in `crates/koa/src/ir/mod.rs` to properly handle member access expressions.

---

### 3. Macro Support
The `println!` macro is not yet implemented. The parser expects semicolons after function calls but doesn't recognize macro syntax.

**Workaround:** Use C functions directly:
```koa
fn main(): i32 {
    puts("Hello, World!");
    return 0;
}
```

**Future Implementation:** Need to add macro expansion to the compiler pipeline.

---

## Development Notes

### Recent Achievements (2026-02-07)
- ✅ **End-to-end compilation** - Koa can now compile and execute programs!
- ✅ **Hello World works** - Successfully compiles and runs basic programs
- ✅ **Build & Run commands** - Both `koa build` and `koa run` commands are functional
- ✅ **LLVM integration** - Generates LLVM IR, compiles to native executables via clang
- ✅ **External functions** - Can call C library functions (printf, puts, etc.)

### Test Results
All tests pass (81 tests total):
- `test_line_comment` - Tests `//` comments
- `test_doc_comment` - Tests `///` and `//!` doc comments  
- `test_block_comment` - Tests `/* */` multi-line comments
- Plus 77 other tests for lexer, parser, type checker, IR, and LLVM codegen

The compiler now successfully:
1. Tokenizes source code
2. Parses to AST
3. Type checks
4. Lowers to IR
5. Generates LLVM IR
6. Compiles to native executable
7. Runs the program
