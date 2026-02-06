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

### 1. Macro Support
The `println!` macro is not yet implemented. The parser expects semicolons after function calls but doesn't recognize macro syntax.

**Workaround:** Use `simple.koa` which doesn't have macros:
```bash
cargo run -p koa-cli -- build examples/simple.koa --output /tmp/simple
```

**Future Implementation:** Need to add macro expansion to the compiler pipeline.

---

## Development Notes

All tests pass including new comment handling tests:
- `test_line_comment` - Tests `//` comments
- `test_doc_comment` - Tests `///` and `//!` doc comments  
- `test_block_comment` - Tests `/* */` multi-line comments

The compiler now successfully tokenizes files with comments without hanging.
