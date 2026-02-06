//! Koa CLI - Compiler driver

use clap::{Parser as ClapParser, Subcommand};
use koa::{ir::IrLowerer, Lexer, Parser as KoaParser, TypeChecker};
use miette::{IntoDiagnostic, Result};
use std::fs;
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
    println!("Reading source...");
    let source = fs::read_to_string(&input).into_diagnostic()?;
    println!("Source read: {} bytes", source.len());

    // Lex
    println!("Lexing...");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    println!("Tokens: {}", tokens.len());

    // Parse
    println!("Parsing...");
    let mut parser = KoaParser::new(tokens);
    let ast = parser.parse()?;
    println!("Declarations: {}", ast.declarations.len());

    // Type check
    println!("Type checking...");
    let mut typeck = TypeChecker::new();
    typeck.check(&ast)?;
    println!("Type check passed");

    // Lower to IR
    println!("Lowering to IR...");
    let mut lowerer = IrLowerer::new();
    let ir_program = lowerer.lower(&ast)?;
    println!("IR functions: {}", ir_program.functions.len());

    // Debug: Print IR
    if ir_program.functions.len() > 0 {
        let func = &ir_program.functions[0];
        println!("  Function: {}", func.name);
        println!("  Instructions: {}", func.body.instructions.len());
        for (i, instr) in func.body.instructions.iter().enumerate() {
            println!("    {}: {:?}", i, instr);
        }
    }

    // Generate LLVM IR
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

    println!("LLVM IR written to: {:?}", ll_path);

    // TODO: Compile .ll to object file
    // TODO: Link to executable

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
