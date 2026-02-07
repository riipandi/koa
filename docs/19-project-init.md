# Project Initialization

The `koa init` command creates a new Koa project with boilerplate code and configuration.

---

## Basic Usage

### Create New Project

```bash
# Create executable project
koa init myapp

# Create library project
koa init --lib mylib

# Create project from template
koa init --template web-app mywebapp
```

**What gets created:**

```
myapp/
├── Koa.toml           # Package configuration
├── Koa.lock           # Lockfile (generated)
├── src/
│   └── main.koa       # Entry point
├── .gitignore         # Git ignore file
├── README.md          # Project README
└── .koa/
    └── cache/         # Build cache
```

---

## Templates

### Available Templates

| Template  | Description      | Use Case                  |
|-----------|------------------|---------------------------|
| `default` | Basic executable | CLI tools, utilities      |
| `lib`     | Library project  | Reusable libraries        |
| `web-app` | Web server       | HTTP servers, APIs        |
| `cli-app` | CLI tool         | Command-line applications |

### Default Template

**Description:** Basic executable with "Hello, World!"

**Files:**

```toml
# Koa.toml
[package]
name = "myapp"
version = "0.1.0"
type = "executable"
```

```
// src/main.koa
fn main(): i32 {
    println!("Hello, World!");
    return 0;
}
```

### Lib Template

**Description:** Library project with public API

**Files:**

```toml
# Koa.toml
[package]
name = "mylib"
version = "0.1.0"
type = "library"

[lib]
name = "mylib"
path = "src/lib.koa"
```

```
// src/lib.koa
///
/// My awesome library
///

///
/// Adds two numbers
///
/// # Examples
/// ```
/// const result: i32 = add(1, 2);
/// assert_eq!(result, 3);
/// ```
///
pub fn add(x: i32, y: i32): i32 {
    return x + y;
}
```

### Web-App Template

**Description:** Web server with HTTP routing

**Files:**

```toml
# Koa.toml
[package]
name = "webapp"
version = "0.1.0"
type = "executable"

[dependencies]
http = { git = "https://github.com/riipandi/koa-http", version = "0.1.0" }
```

```
// src/main.koa
import from "net/http";
import from "std/io/println";

fn main(): i32 {
    let router: http.Router = http.Router::new();

    router.get("/", fn(_req: Request): Response {
        return Response::ok("Hello, World!");
    });

    router.get("/health", fn(_req: Request): Response {
        return Response::ok("OK");
    });

    println("Server listening on :8080");
    http.serve(router, ":8080");

    return 0;
}
```

### CLI-App Template

**Description:** CLI tool with argument parsing

**Files:**

```toml
# Koa.toml
[package]
name = "mycli"
version = "0.1.0"
type = "executable"

[dependencies]
cli = { git = "https://github.com/riipandi/koa-cli", version = "0.1.0" }
```

```
// src/main.koa
import from "std/cli";
import from "std/io/println";

fn main(): i32 {
    let app: App = App::new("mycli")
        .version("0.1.0")
        .author("Your Name")
        .about("My awesome CLI tool")
        .arg(Arg::new("input")
            .help("Input file")
            .required(true))
        .arg(Arg::new("output")
            .short("o")
            .long("output")
            .help("Output file")
            .takes_value(true));

    let matches: Matches = app.parse();

    // Process input
    let input: string = matches.value_of("input").unwrap();
    println!("Processing: {}", input);

    return 0;
}
```

---

## Options

### --lib

Create a library project:

```bash
koa init --lib mylib
```

### --template

Use a specific template:

```bash
koa init --template web-app mywebapp
```

### --dir

Specify directory (default: project name):

```bash
koa init --dir ./src/myapp
```

### --git

Initialize Git repository:

```bash
koa init --git myapp
```

**Equivalent to:**

```bash
koa init myapp
cd myapp
git init
```

### --no-readme

Skip README.md creation:

```bash
koa init --no-readme myapp
```

### --license

Specify license:

```bash
koa init --license MIT myapp
koa init --license Apache-2.0 myapp
```

---

## Interactive Mode

For a guided experience, use interactive mode:

```bash
koa init --interactive
```

**Prompts:**

```
? Project name: myapp
? Project type: executable
? Template: default
? Author: Your Name <you@example.com>
? License: MIT
? Initialize Git: Yes
```

---

## Configuration

### Default Template Directory

Templates are stored in:

```
~/.koa/templates/
├── default/
├── lib/
├── web-app/
└── cli-app/
```

### Custom Templates

Create custom templates:

```bash
# Create template directory
mkdir -p ~/.koa/templates/my-template

# Add template files
cat > ~/.koa/templates/my-template/Koa.toml << EOF
[package]
name = "{{ name }}"
version = "0.1.0"
type = "executable"
EOF

# Use custom template
koa init --template my-template myapp
```

### Template Variables

Templates support variables:

| Variable        | Description     |
|-----------------|-----------------|
| `{{ name }}`    | Project name    |
| `{{ version }}` | Project version |
| `{{ author }}`  | Author name     |
| `{{ year }}`    | Current year    |

**Example:**

```toml
# Koa.toml template
[package]
name = "{{ name }}"
version = "{{ version }}"
authors = ["{{ author }}"]
```

---

## .gitignore

Generated `.gitignore` includes:

```
# Koa build artifacts
.koa/
build/
*.koa.o
*.koa.bc

# Koa lockfile (optional)
# Koa.lock

# OS-specific
.DS_Store
Thumbs.db

# IDE
.vscode/
.idea/
*.swp
*.swo
```

---

## README.md

Generated `README.md` includes:

```markdown
# {{ name }}

{{ description }}

## Installation

```bash
git clone https://github.com/user/{{ name }}.git
cd {{ name }}
koa build
```

## Usage

```bash
koa run
```

## Development

```bash
# Run tests
koa test

# Build
koa build

# Run
./build/debug/{{ name }}
```

## License

{{ license }}
```

---

## Next Steps

After project initialization:

```bash
# Change to project directory
cd myapp

# Build the project
koa build

# Run the project
koa run

# Run tests (if any)
koa test
```

---

## Examples

### Create CLI Tool

```bash
# Create CLI project
koa init --template cli-app mycli
cd mycli

# Build
koa build --mode release

# Install globally
sudo cp build/release/mycli /usr/local/bin/

# Use
mycli --help
```

### Create Web Server

```bash
# Create web app
koa init --template web-app mywebapp
cd mywebapp

# Fetch dependencies
koa fetch

# Run
koa run
```

### Create Library

```bash
# Create library
koa init --lib mylib
cd mylib

# Build
koa build

# Test
cat > examples/test.koa << EOF
import { add } from "mylib";

fn main(): i32 {
    println!("1 + 2 = {}", add(1, 2));
    return 0;
}
EOF

koa run examples/test.koa
```

---

## Troubleshooting

### Issue: "Template not found"

**Solution:** Update templates

```bash
koa update --templates
```

### Issue: "Directory already exists"

**Solution:** Use force or different directory

```bash
# Force overwrite
koa init --force myapp

# Use different directory
koa init --dir ./projects/myapp myapp
```

---

## Best Practices

1. **Use templates** - Don't start from scratch
2. **Initialize Git** - Use `--git` flag
3. **Choose appropriate template** - Match your use case
4. **Update README** - Document your project
5. **Set up CI/CD** - Add GitHub Actions/GitLab CI

---

## See Also

- [Build System](16-build-system.md) - Koa.toml configuration
- [Package Manager](11-package-manager.md) - Dependency management
- [Project Structure](../project-structure.md) - Directory layout
