//! Koa CLI - Compiler driver

use clap::{Parser as ClapParser, Subcommand};
use koa::{Lexer, Parser as KoaParser, TypeChecker};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

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

    /// Fetch dependencies
    Fetch,

    /// Update dependencies
    Update {
        /// Specific package to update
        package: Option<String>,
    },

    /// Run tests
    Test {
        /// Test name filter
        #[arg(long)]
        filter: Option<String>,
    },
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

        Commands::Fetch => fetch(),

        Commands::Update { package } => update(package),

        Commands::Test { filter } => test(filter),
    }
}

/// Build command
fn build(input: PathBuf, output: Option<PathBuf>, mode: String) -> Result<()> {
    println!("Building {:?} in {} mode", input, mode);

    // Read source
    let source = std::fs::read_to_string(&input).into_diagnostic()?;

    // Lex
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    println!("Tokens: {}", tokens.len());

    // Parse
    let mut parser = KoaParser::new(tokens);
    let ast = parser.parse()?;

    println!("Declarations: {}", ast.declarations.len());

    // Type check
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;

    // TODO: Compile to LLVM IR
    // TODO: Generate object file
    // TODO: Link

    let out_path = output.unwrap_or_else(|| {
        let mut out = input.clone();
        out.set_extension("");
        out
    });

    println!("Output: {:?}", out_path);

    Ok(())
}

/// Run command
fn run(input: PathBuf) -> Result<()> {
    println!("Running {:?}", input);

    // Build then execute
    build(input.clone(), None, "debug".to_string())?;

    let mut exe = input.clone();
    exe.set_extension("");

    std::process::Command::new(&exe)
        .status()
        .into_diagnostic()?;

    Ok(())
}

/// Fetch command
fn fetch() -> Result<()> {
    println!("Fetching dependencies...");

    // TODO: Read Koa.toml
    // TODO: Download dependencies
    // TODO: Generate Koa.lock

    Ok(())
}

/// Update command
fn update(package: Option<String>) -> Result<()> {
    match package {
        Some(pkg) => println!("Updating {}...", pkg),
        None => println!("Updating all dependencies..."),
    }

    // TODO: Update dependencies
    // TODO: Update Koa.lock

    Ok(())
}

/// Test command
fn test(filter: Option<String>) -> Result<()> {
    match filter {
        Some(f) => println!("Running tests matching: {}", f),
        None => println!("Running all tests..."),
    }

    // TODO: Find and run tests
    // TODO: Support test filters

    Ok(())
}
