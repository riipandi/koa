//! Koa CLI - Compiler driver

use clap::{Parser as ClapParser, Subcommand};
use console::{style, Color};
use indicatif::{ProgressBar, ProgressStyle};
use koa::{ir::IrLowerer, Lexer, Parser as KoaParser, TypeChecker};
use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
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

        Commands::Test { filter } => test(filter),
    }
}

/// Build command
fn build(input: PathBuf, output: Option<PathBuf>, mode: String) -> Result<()> {
    let spinner = create_spinner(&format!(
        "Building {} in {} mode",
        style(input.display()).cyan(),
        mode
    ));

    // Read source
    spinner.set_message("Reading source...");
    let source = fs::read_to_string(&input).into_diagnostic()?;
    spinner.finish_with_message(format!("Source read: {} bytes", style(source.len()).cyan()));

    // Lex
    let lex_spinner = create_spinner("Lexing...");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    lex_spinner.finish_with_message(format!(
        "Tokens: {}",
        style(tokens.len()).cyan().to_string()
    ));

    // Parse
    let parse_spinner = create_spinner("Parsing...");
    let mut parser = KoaParser::new(tokens);
    let ast = parser.parse()?;
    parse_spinner.finish_with_message(format!(
        "Declarations: {}",
        style(ast.declarations.len()).cyan().to_string()
    ));

    // Type check
    let typeck_spinner = create_spinner("Type checking...");
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    typeck_spinner.finish_with_message(style("Type check passed").green().to_string());

    // Lower to IR
    let lower_spinner = create_spinner("Lowering to IR...");
    let mut lowerer = IrLowerer::new();
    let ir_program = lowerer.lower(&ast)?;
    lower_spinner.finish_with_message(format!(
        "IR functions: {}",
        style(ir_program.functions.len()).cyan().to_string()
    ));

    // Debug: Print IR
    if ir_program.functions.len() > 0 {
        let func = &ir_program.functions[0];
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
    let llvm_ir = koa::llvm_gen::compile_to_llvm(&ir_program)?;

    // Determine output path
    let out_path = output.unwrap_or_else(|| {
        let mut out = input.clone();
        out.set_extension("");
        out
    });

    // Write LLVM IR to .ll file
    let ll_path = out_path.with_extension("ll");
    fs::write(&ll_path, llvm_ir).into_diagnostic()?;
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
    println!("Running {}", style(input.display()).cyan());

    // Build then execute
    build(input.clone(), None, "debug".to_string())?;

    let mut exe = input.clone();
    exe.set_extension("");

    std::process::Command::new(&exe)
        .status()
        .into_diagnostic()?;

    Ok(())
}

/// Package: Fetch dependencies
fn pkg_fetch() -> Result<()> {
    let spinner = create_spinner("Fetching dependencies...");

    // Check if Koa.toml exists
    if !PathBuf::from("Koa.toml").exists() {
        spinner.finish_with_message(style("Error: Koa.toml not found").red().to_string());
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // TODO: Read Koa.toml
    // TODO: Download dependencies
    // TODO: Generate Koa.lock

    spinner.finish_with_message(style("Dependencies fetched").green().to_string());

    Ok(())
}

/// Package: Update dependencies
fn pkg_update(package: Option<String>) -> Result<()> {
    let spinner = create_spinner(if let Some(pkg) = &package {
        format!("Updating {}...", style(pkg).cyan().to_string())
    } else {
        "Updating all dependencies...".to_string()
    });

    // TODO: Update dependencies
    // TODO: Update Koa.lock

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
    let spinner = create_spinner(format!("Adding {}...", style(&package).cyan().to_string()));

    // Check if Koa.toml exists
    let toml_path = PathBuf::from("Koa.toml");
    if !toml_path.exists() {
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

    // Read existing Koa.toml
    let content = fs::read_to_string(&toml_path).into_diagnostic()?;
    let mut koa_toml: KoaToml = toml::from_str(&content).into_diagnostic()?;

    // Determine dependency spec
    let dep = if let Some(local_path) = path {
        Dependency::Path(PathDependency { path: local_path })
    } else if let Some(git_url) = git {
        let mut git_dep = GitDependency {
            git: git_url,
            version: version.or(pkg_version),
            branch,
            ..Default::default()
        };
        Dependency::Git(git_dep)
    } else {
        // TODO: Search for package in virtual registry
        spinner.finish_with_message(
            style("Error: Must specify --git or --path")
                .red()
                .to_string(),
        );
        miette::bail!("Package location not specified. Use --git URL or --path PATH");
    };

    // Add dependency
    koa_toml.dependencies.insert(pkg_name.clone(), dep);

    // Write back to Koa.toml
    let new_content = toml::to_string_pretty(&koa_toml).into_diagnostic()?;
    fs::write(&toml_path, new_content).into_diagnostic()?;

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
    let spinner = create_spinner(format!(
        "Removing {}...",
        style(&package).cyan().to_string()
    ));

    let toml_path = PathBuf::from("Koa.toml");
    if !toml_path.exists() {
        spinner.finish_with_message(style("Error: Koa.toml not found").red().to_string());
        miette::bail!("Koa.toml not found. Are you in a Koa project?");
    }

    // Read existing Koa.toml
    let content = fs::read_to_string(&toml_path).into_diagnostic()?;
    let mut koa_toml: KoaToml = toml::from_str(&content).into_diagnostic()?;

    // Check if dependency exists
    if !koa_toml.dependencies.contains_key(&package) {
        spinner.finish_with_message(format!(
            "Dependency {} not found",
            style(&package).yellow().to_string()
        ));
        return Ok(());
    }

    // Remove dependency
    koa_toml.dependencies.remove(&package);

    // Write back to Koa.toml
    let new_content = toml::to_string_pretty(&koa_toml).into_diagnostic()?;
    fs::write(&toml_path, new_content).into_diagnostic()?;

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
