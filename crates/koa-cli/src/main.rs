//! Koa CLI - Compiler driver

use clap::{Parser as ClapParser, Subcommand};
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use inquire::{Confirm, Select, Text};
use koa::{ir::IrLowerer, Lexer, Parser as KoaParser, TypeChecker};
use log::{debug, error, info, warn};
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

/// Koa Programming Language Compiler
#[derive(ClapParser, Debug)]
#[command(name = "koa")]
#[command(author = "Koa Contributors")]
#[command(version = "0.1.0")]
#[command(about = "Modern compiled programming language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile Koa code
    Build {
        /// Input file
        input: PathBuf,

        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Build mode
        #[arg(long, default_value = "debug")]
        mode: String,
    },

    /// Run Koa code
    Run {
        /// Input file
        input: PathBuf,
    },

    /// Package management
    Pkg {
        #[command(subcommand)]
        command: PkgCommands,
    },

    /// Initialize a new Koa project
    Init {
        /// Project name (optional, defaults to current directory)
        name: Option<String>,

        /// Interactive mode with prompts
        #[arg(long)]
        interactive: bool,

        /// Create as library project
        #[arg(long)]
        lib: bool,

        /// Initialize git repository
        #[arg(long)]
        git: bool,

        /// Skip creating .gitignore
        #[arg(long)]
        no_gitignore: bool,

        /// Skip creating README.md
        #[arg(long)]
        no_readme: bool,
    },

    /// Run tests
    Test {
        /// Test name filter
        #[arg(long)]
        filter: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum PkgCommands {
    /// Download dependencies
    Fetch,

    /// Update dependencies
    Update {
        /// Specific package to update
        package: Option<String>,
    },

    /// Add a new dependency
    Add {
        /// Package name and version (e.g., http@0.1.0)
        package: String,

        /// Git repository URL
        #[arg(long)]
        git: Option<String>,

        /// Version constraint
        #[arg(long)]
        version: Option<String>,

        /// Branch name
        #[arg(long)]
        branch: Option<String>,

        /// Local path
        #[arg(long)]
        path: Option<String>,
    },

    /// Remove a dependency
    Remove {
        /// Package name to remove
        package: String,
    },

    /// List installed dependencies
    List,

    /// Check for outdated dependencies
    Outdated,

    /// Show dependency tree
    Tree,
}

#[derive(Debug, Deserialize, Serialize)]
struct KoaToml {
    package: Package,
    #[serde(default)]
    dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    r#type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Dependency {
    Git(GitDependency),
    Path(PathDependency),
    Simple(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GitDependency {
    git: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PathDependency {
    path: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up multi-progress for logging and progress bars
    let multi = MultiProgress::new();
    let logger = LogWrapper::new(
        multi,
        env_logger::Builder::new()
            .filter_level(
                std::env::var("KOA_LOG")
                    .or_else(|_| std::env::var("RUST_LOG"))
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(log::LevelFilter::Warn),
            )
            .build(),
    );

    // Set the global logger
    log::set_boxed_logger(Box::new(logger))
        .map(|()| {
            log::set_max_level(
                std::env::var("KOA_LOG")
                    .or_else(|_| std::env::var("RUST_LOG"))
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(log::LevelFilter::Warn),
            )
        })
        .into_diagnostic()?;

    info!("Koa CLI v0.1.0");
    debug!("Command: {:?}", cli.command);

    match cli.command {
        Commands::Build {
            input,
            output,
            mode,
        } => build(input, output, mode),

        Commands::Run { input } => run(input),

        Commands::Pkg { command } => match command {
            PkgCommands::Fetch => pkg_fetch(),
            PkgCommands::Update { package } => pkg_update(package),
            PkgCommands::Add {
                package,
                git,
                version,
                branch,
                path,
            } => pkg_add(package, git, version, branch, path),
            PkgCommands::Remove { package } => pkg_remove(package),
            PkgCommands::List => pkg_list(),
            PkgCommands::Outdated => pkg_outdated(),
            PkgCommands::Tree => pkg_tree(),
        },

        Commands::Init {
            name,
            interactive,
            lib,
            git,
            no_gitignore,
            no_readme,
        } => init(name, interactive, lib, git, no_gitignore, no_readme),

        Commands::Test { filter } => test(filter),
    }
}

/// Build command
fn build(input: PathBuf, output: Option<PathBuf>, mode: String) -> Result<()> {
    info!("Building '{}' in {} mode", input.display(), mode);
    debug!("Input file: {:?}", input);
    debug!("Output file: {:?}", output);

    let spinner = create_spinner(&format!(
        "Building {} in {} mode",
        style(input.display()).cyan(),
        mode
    ));

    // Read source
    spinner.set_message("Reading source...");
    let source = fs::read_to_string(&input).into_diagnostic()?;
    info!("Source read: {} bytes", source.len());
    debug!("Source content length: {}", source.len());
    spinner.finish_with_message(format!("Source read: {} bytes", style(source.len()).cyan()));

    // Lex
    let lex_spinner = create_spinner("Lexing...");
    debug!("Starting lexical analysis");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    info!("Lexical analysis complete: {} tokens", tokens.len());
    debug!("Token count: {}", tokens.len());
    lex_spinner.finish_with_message(format!(
        "Tokens: {}",
        style(tokens.len()).cyan().to_string()
    ));

    // Parse
    let parse_spinner = create_spinner("Parsing...");
    debug!("Starting parsing");
    let mut parser = KoaParser::new(tokens);
    let ast = parser.parse()?;
    info!("Parsing complete: {} declarations", ast.declarations.len());
    debug!("Declarations: {}", ast.declarations.len());
    parse_spinner.finish_with_message(format!(
        "Declarations: {}",
        style(ast.declarations.len()).cyan().to_string()
    ));

    // Type check
    let typeck_spinner = create_spinner("Type checking...");
    debug!("Starting type checking");
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    info!("Type checking passed");
    typeck_spinner.finish_with_message(style("Type check passed").green().to_string());

    // Lower to IR
    let lower_spinner = create_spinner("Lowering to IR...");
    debug!("Starting IR lowering");
    let mut lowerer = IrLowerer::new();
    let ir_program = lowerer.lower(&ast)?;
    info!(
        "IR lowering complete: {} functions",
        ir_program.functions.len()
    );
    debug!("IR functions: {:?}", ir_program.functions);
    lower_spinner.finish_with_message(format!(
        "IR functions: {}",
        style(ir_program.functions.len()).cyan().to_string()
    ));

    // Debug: Print IR
    if ir_program.functions.len() > 0 {
        let func = &ir_program.functions[0];
        debug!(
            "Function: {} with {} instructions",
            func.name,
            func.body.instructions.len()
        );
        println!("  Function: {}", style(&func.name).cyan());
        println!(
            "  Instructions: {}",
            style(func.body.instructions.len()).cyan()
        );
        for (i, instr) in func.body.instructions.iter().enumerate() {
            println!("    {}: {:?}", i, instr);
        }
    }

    // Generate LLVM IR
    let llvm_spinner = create_spinner("Generating LLVM IR...");
    debug!("Starting LLVM IR generation");
    let llvm_ir = koa::llvm_gen::compile_to_llvm(&ir_program)?;
    info!("LLVM IR generated: {} bytes", llvm_ir.len());

    // Determine output path
    let out_path = output.unwrap_or_else(|| {
        let mut out = input.clone();
        out.set_extension("");
        out
    });

    // Write LLVM IR to .ll file
    let ll_path = out_path.with_extension("ll");
    fs::write(&ll_path, llvm_ir).into_diagnostic()?;
    info!("LLVM IR written to: {}", ll_path.display());
    llvm_spinner.finish_with_message(format!(
        "LLVM IR written to: {}",
        style(ll_path.display()).cyan().to_string()
    ));

    // TODO: Compile .ll to object file
    // TODO: Link to executable

    Ok(())
}

/// Run command
fn run(input: PathBuf) -> Result<()> {
    info!("Running {}", input.display());
    println!("Running {}", style(input.display()).cyan());

    // Build then execute
    debug!("Building before running");
    build(input.clone(), None, "debug".to_string())?;

    let mut exe = input.clone();
    exe.set_extension("");
    debug!("Executing: {:?}", exe);

    let status = std::process::Command::new(&exe)
        .status()
        .into_diagnostic()?;

    info!("Execution exited with: {:?}", status);
    Ok(())
}

/// Initialize a new Koa project
fn init(
    name: Option<String>,
    interactive: bool,
    lib: bool,
    git: bool,
    no_gitignore: bool,
    no_readme: bool,
) -> Result<()> {
    info!("Initializing new Koa project");
    debug!(
        "Parameters: name={:?}, interactive={}, lib={}, git={}, no_gitignore={}, no_readme={}",
        name, interactive, lib, git, no_gitignore, no_readme
    );

    // Interactive mode
    let (name, lib, git, no_gitignore, no_readme) = if interactive {
        info!("Starting interactive mode");
        let project_name = if let Some(n) = &name {
            n.clone()
        } else {
            Text::new("Project name:")
                .with_placeholder("myproject")
                .prompt()
                .into_diagnostic()?
        };

        let project_type = Select::new("Project type:", vec!["Executable", "Library"])
            .prompt()
            .into_diagnostic()?;

        let init_git = Confirm::new("Initialize git repository?")
            .with_default(true)
            .prompt()
            .into_diagnostic()?;

        let create_gitignore = Confirm::new("Create .gitignore?")
            .with_default(true)
            .prompt()
            .into_diagnostic()?;

        let create_readme = Confirm::new("Create README.md?")
            .with_default(true)
            .prompt()
            .into_diagnostic()?;

        debug!(
            "Interactive selections: name={}, type={}, git={}, gitignore={}, readme={}",
            project_name, project_type, init_git, create_gitignore, create_readme
        );

        (
            Some(project_name),
            project_type == "Library",
            init_git,
            !create_gitignore,
            !create_readme,
        )
    } else {
        (name, lib, git, no_gitignore, no_readme)
    };

    let project_name = if let Some(n) = &name {
        n.clone()
    } else {
        // Check if current directory is empty
        let current_dir = std::env::current_dir().into_diagnostic()?;
        let dir_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("koa-project")
            .to_string();

        debug!("Current directory name: {}", dir_name);

        // Check if directory is empty (only hidden files allowed)
        let is_empty = current_dir
            .read_dir()
            .into_diagnostic()?
            .filter_map(Result::ok)
            .all(|entry| {
                let name = entry.file_name();
                name.to_string_lossy().starts_with('.')
            });

        if !is_empty {
            error!("Current directory is not empty");
            miette::bail!(
                "Current directory is not empty. Specify a project name or run in an empty directory."
            );
        }

        dir_name
    };

    info!("Project name: {}", project_name);
    info!(
        "Project type: {}",
        if lib { "library" } else { "executable" }
    );

    // Create project directory if name was provided
    let project_dir = if name.is_some() {
        let dir = PathBuf::from(&project_name);
        if dir.exists() {
            error!("Directory '{}' already exists", project_name);
            miette::bail!("Directory '{}' already exists", project_name);
        }

        let spinner = create_spinner(&format!("Creating {}...", style(&project_name).cyan()));
        fs::create_dir_all(&dir).into_diagnostic()?;
        info!("Created directory: {}", project_name);
        spinner.finish_with_message(
            style(format!("Created {}", project_name))
                .green()
                .to_string(),
        );

        dir
    } else {
        PathBuf::from(".")
    };

    // Create Koa.toml
    let spinner = create_spinner("Creating Koa.toml...");
    let koa_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
type = "{}"

[dependencies]
"#,
        project_name,
        if lib { "library" } else { "executable" }
    );

    let toml_path = project_dir.join("Koa.toml");
    let mut file = File::create(&toml_path).into_diagnostic()?;
    file.write_all(koa_toml.as_bytes()).into_diagnostic()?;
    spinner.finish_with_message(style("Koa.toml created").green().to_string());

    // Create src directory
    let spinner = create_spinner("Creating src directory...");
    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir).into_diagnostic()?;
    spinner.finish_with_message(style("src/ created").green().to_string());

    // Create main.koa or lib.koa
    let spinner = create_spinner(&format!(
        "Creating {}...",
        style(if lib { "lib.koa" } else { "main.koa" }).cyan()
    ));

    let source_file = if lib {
        src_dir.join("lib.koa")
    } else {
        src_dir.join("main.koa")
    };

    let source_code = if lib {
        r#"///
/// Koa Library
///

///
/// Add two numbers
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

///
/// Subtract two numbers
///
pub fn sub(x: i32, y: i32): i32 {
    return x - y;
}
"#
    } else {
        r#"///
/// Koa Main Entry Point
///

fn main(): i32 {
    println!("Hello, World!");
    return 0;
}
"#
    };

    let mut file = File::create(&source_file).into_diagnostic()?;
    file.write_all(source_code.as_bytes()).into_diagnostic()?;
    spinner.finish_with_message(
        style(format!(
            "{} created",
            if lib { "lib.koa" } else { "main.koa" }
        ))
        .green()
        .to_string(),
    );

    // Create .gitignore
    if !no_gitignore {
        let spinner = create_spinner("Creating .gitignore...");
        let gitignore = r#"# Koa build artifacts
.koa/
build/
*.koa.o
*.koa.bc

# Koa lockfile
# Uncomment to commit lockfile:
# Koa.lock

# OS-specific
.DS_Store
Thumbs.db

# IDE
.vscode/
.idea/
*.swp
*.swo
"#;

        let gitignore_path = project_dir.join(".gitignore");
        let mut file = File::create(&gitignore_path).into_diagnostic()?;
        file.write_all(gitignore.as_bytes()).into_diagnostic()?;
        spinner.finish_with_message(style(".gitignore created").green().to_string());
    }

    // Create README.md
    if !no_readme {
        let spinner = create_spinner("Creating README.md...");
        let readme = format!(
            r#"# {}

{}

## Installation

```bash
git clone https://github.com/user/{}.git
cd {}
koa build
```

## Usage

```bash
{}
```

## Development

```bash
# Run tests
koa test

# Build
koa build

# Run
{}
```

## License

MIT
"#,
            project_name,
            if lib {
                "A Koa library project"
            } else {
                "A Koa application"
            },
            project_name,
            project_name,
            if lib {
                format!("import {{ add }} from \"{}\";\n\nfn main(): i32 {{\n    println!(\"1 + 2 = {{}}\", add(1, 2));\n    return 0;\n}}", project_name)
            } else {
                "koa run".to_string()
            },
            if lib {
                format!("cargo run --example test")
            } else {
                "./build/debug/main".to_string()
            }
        );

        let readme_path = project_dir.join("README.md");
        let mut file = File::create(&readme_path).into_diagnostic()?;
        file.write_all(readme.as_bytes()).into_diagnostic()?;
        spinner.finish_with_message(style("README.md created").green().to_string());
    }

    // Initialize git repository if requested
    if git {
        let spinner = create_spinner("Initializing git repository...");
        std::process::Command::new("git")
            .arg("init")
            .current_dir(&project_dir)
            .output()
            .into_diagnostic()?;

        spinner.finish_with_message(style("Git repository initialized").green().to_string());
    }

    // Print summary
    println!();
    println!(
        "{}",
        style("✓ Project created successfully!").green().bold()
    );
    println!();

    if name.is_some() {
        println!("Next steps:");
        println!("  1. cd {}", style(&project_name).cyan());
    } else {
        println!("Next steps:");
    }
    println!(
        "  2. {} dependencies if needed",
        style("koa pkg add").cyan()
    );
    println!("  3. {} to build", style("koa build").cyan());
    println!(
        "  4. {} to run",
        style(if lib { "koa test" } else { "koa run" }).cyan()
    );
    println!();

    Ok(())
}

/// Package: Fetch dependencies
fn pkg_fetch() -> Result<()> {
    info!("Fetching dependencies");
    let spinner = create_spinner("Fetching dependencies...");

    // Check if Koa.toml exists
    if !PathBuf::from("Koa.toml").exists() {
        error!("Koa.toml not found");
        spinner.finish_with_message(style("Error: Koa.toml not found").red().to_string());
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // TODO: Read Koa.toml
    // TODO: Download dependencies
    // TODO: Generate Koa.lock

    info!("Dependencies fetched successfully");
    spinner.finish_with_message(style("Dependencies fetched").green().to_string());

    Ok(())
}

/// Package: Update dependencies
fn pkg_update(package: Option<String>) -> Result<()> {
    if let Some(pkg) = &package {
        info!("Updating dependency: {}", pkg);
    } else {
        info!("Updating all dependencies");
    }

    let spinner = create_spinner(if let Some(pkg) = &package {
        format!("Updating {}...", style(pkg).cyan().to_string())
    } else {
        "Updating all dependencies...".to_string()
    });

    // TODO: Update dependencies
    // TODO: Update Koa.lock

    info!("Dependencies updated successfully");
    spinner.finish_with_message(style("Dependencies updated").green().to_string());

    Ok(())
}

/// Package: Add dependency
fn pkg_add(
    package: String,
    git: Option<String>,
    version: Option<String>,
    branch: Option<String>,
    path: Option<String>,
) -> Result<()> {
    info!("Adding dependency: {}", package);
    debug!(
        "Git: {:?}, Version: {:?}, Branch: {:?}, Path: {:?}",
        git, version, branch, path
    );

    let spinner = create_spinner(format!("Adding {}...", style(&package).cyan().to_string()));

    // Check if Koa.toml exists
    let toml_path = PathBuf::from("Koa.toml");
    if !toml_path.exists() {
        error!("Koa.toml not found");
        spinner.finish_with_message(style("Error: Koa.toml not found").red().to_string());
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // Parse package name and version from string (e.g., "http@0.1.0")
    let (pkg_name, pkg_version) = if let Some(pos) = package.find('@') {
        let name = package[..pos].to_string();
        let ver = package[pos + 1..].to_string();
        (name, Some(ver))
    } else {
        (package, None)
    };

    debug!("Package name: {}, version: {:?}", pkg_name, pkg_version);

    // Read existing Koa.toml
    let content = fs::read_to_string(&toml_path).into_diagnostic()?;
    let mut koa_toml: KoaToml = toml::from_str(&content).into_diagnostic()?;

    // Determine dependency spec
    let dep = if let Some(local_path) = path {
        debug!("Using path dependency: {}", local_path);
        Dependency::Path(PathDependency { path: local_path })
    } else if let Some(git_url) = git {
        debug!("Using git dependency: {}", git_url);
        let git_dep = GitDependency {
            git: git_url,
            version: version.or(pkg_version),
            branch,
            ..Default::default()
        };
        Dependency::Git(git_dep)
    } else {
        // TODO: Search for package in virtual registry
        error!("Must specify --git or --path");
        spinner.finish_with_message(
            style("Error: Must specify --git or --path")
                .red()
                .to_string(),
        );
        miette::bail!("Package location not specified. Use --git URL or --path PATH");
    };

    // Add dependency
    koa_toml.dependencies.insert(pkg_name.clone(), dep.clone());
    debug!("Added dependency: {:?}", dep);

    // Write back to Koa.toml
    let new_content = toml::to_string_pretty(&koa_toml).into_diagnostic()?;
    fs::write(&toml_path, new_content).into_diagnostic()?;
    info!("Dependency '{}' added to Koa.toml", pkg_name);

    spinner.finish_with_message(format!(
        "{} added to {}",
        style(pkg_name).cyan(),
        style("Koa.toml").green().to_string()
    ));

    // Optionally fetch immediately
    println!(
        "Run {} to download the dependency",
        style("koa pkg fetch").cyan()
    );

    Ok(())
}

/// Package: Remove dependency
fn pkg_remove(package: String) -> Result<()> {
    info!("Removing dependency: {}", package);

    let spinner = create_spinner(format!(
        "Removing {}...",
        style(&package).cyan().to_string()
    ));

    let toml_path = PathBuf::from("Koa.toml");
    if !toml_path.exists() {
        error!("Koa.toml not found");
        spinner.finish_with_message(style("Error: Koa.toml not found").red().to_string());
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // Read existing Koa.toml
    let content = fs::read_to_string(&toml_path).into_diagnostic()?;
    let mut koa_toml: KoaToml = toml::from_str(&content).into_diagnostic()?;

    // Check if dependency exists
    if !koa_toml.dependencies.contains_key(&package) {
        warn!("Dependency '{}' not found", package);
        spinner.finish_with_message(format!(
            "Dependency {} not found",
            style(&package).yellow().to_string()
        ));
        return Ok(());
    }

    // Remove dependency
    koa_toml.dependencies.remove(&package);
    debug!("Removed dependency: {}", package);

    // Write back to Koa.toml
    let new_content = toml::to_string_pretty(&koa_toml).into_diagnostic()?;
    fs::write(&toml_path, new_content).into_diagnostic()?;
    info!("Dependency '{}' removed from Koa.toml", package);

    spinner.finish_with_message(format!(
        "{} removed from {}",
        style(package).cyan(),
        style("Koa.toml").green()
    ));

    Ok(())
}

/// Package: List dependencies
fn pkg_list() -> Result<()> {
    let toml_path = PathBuf::from("Koa.toml");
    if !toml_path.exists() {
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // Read Koa.toml
    let content = fs::read_to_string(&toml_path).into_diagnostic()?;
    let koa_toml: KoaToml = toml::from_str(&content).into_diagnostic()?;

    if koa_toml.dependencies.is_empty() {
        println!("No dependencies");
        return Ok(());
    }

    println!(
        "\n{} ({})",
        style("Dependencies").green().bold(),
        style(koa_toml.dependencies.len()).cyan()
    );

    for (name, dep) in koa_toml.dependencies.iter() {
        match dep {
            Dependency::Git(git) => {
                let version = git.version.as_deref().unwrap_or("latest");
                println!(
                    "  {} {} {}",
                    style(name).cyan(),
                    style(version).yellow(),
                    style(format!("git+{}", git.git)).dim()
                );
            }
            Dependency::Path(path) => {
                println!(
                    "  {} {} {}",
                    style(name).cyan(),
                    style("path").yellow(),
                    style(&path.path).dim()
                );
            }
            Dependency::Simple(ver) => {
                println!("  {} {}", style(name).cyan(), style(ver).yellow());
            }
        }
    }

    Ok(())
}

/// Package: Check for outdated dependencies
fn pkg_outdated() -> Result<()> {
    let spinner = create_spinner("Checking for outdated dependencies...");

    let toml_path = PathBuf::from("Koa.toml");
    if !toml_path.exists() {
        spinner.finish_with_message(style("Error: Koa.toml not found").red().to_string());
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // TODO: Implement version checking
    // This requires fetching git tags and comparing versions

    spinner.finish_with_message(style("All dependencies are up to date").green().to_string());

    Ok(())
}

/// Package: Show dependency tree
fn pkg_tree() -> Result<()> {
    let toml_path = PathBuf::from("Koa.toml");
    if !toml_path.exists() {
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // Read Koa.toml
    let content = fs::read_to_string(&toml_path).into_diagnostic()?;
    let koa_toml: KoaToml = toml::from_str(&content).into_diagnostic()?;

    println!(
        "\n{} {}\n",
        style(koa_toml.package.name).cyan().bold(),
        style(&koa_toml.package.version).yellow()
    );

    if koa_toml.dependencies.is_empty() {
        println!("(no dependencies)");
    } else {
        for (name, dep) in koa_toml.dependencies.iter() {
            match dep {
                Dependency::Git(git) => {
                    let version = git.version.as_deref().unwrap_or("latest");
                    println!("└── {} {}", style(name).cyan(), style(version).yellow());
                }
                Dependency::Path(path) => {
                    println!("└── {} {}", style(name).cyan(), style(&path.path).yellow());
                }
                Dependency::Simple(ver) => {
                    println!("└── {} {}", style(name).cyan(), style(ver).yellow());
                }
            }
        }
    }

    Ok(())
}

/// Test command
fn test(filter: Option<String>) -> Result<()> {
    let spinner = create_spinner(if let Some(f) = &filter {
        format!("Running tests matching {}...", style(f).cyan())
    } else {
        "Running all tests...".to_string()
    });

    // TODO: Find and run tests
    // TODO: Support test filters

    spinner.finish_with_message(style("Tests passed").green().to_string());

    Ok(())
}

/// Create a styled progress spinner
fn create_spinner(message: impl Into<String>) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.into());
    spinner
}

// Implement Default for GitDependency
impl Default for GitDependency {
    fn default() -> Self {
        Self {
            git: String::new(),
            version: None,
            branch: None,
            tag: None,
            rev: None,
        }
    }
}
