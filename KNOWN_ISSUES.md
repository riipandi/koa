# Known Issues

**Comment Bug:** The `hello_world.koa` example contains doc comments that trigger a known lexer bug (infinite loop). This is documented in the CONVERSATION SUMMARY and needs to be fixed before the `debug` Makefile target can work properly.

**Workaround:** Use `simple.koa` which doesn't have comments:
```bash
# Use simple example instead
make debug-simple
```
