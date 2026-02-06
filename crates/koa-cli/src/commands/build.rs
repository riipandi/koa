use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn execute(input: Option<&str>, _mode: &str) -> Result<()> {
    let target_path = resolve_target(input)?;
    let target_path = Path::new(&target_path);

    if !target_path.exists() {
        anyhow::bail!("Input file '{}' not found", target_path.display());
    }

    println!(
        "{} Building {}...",
        "Compiling".green().bold(),
        target_path.display()
    );

    let source = std::fs::read_to_string(target_path)?;

    // Lexing
    let mut lexer = koa::Lexer::new(&source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| anyhow::anyhow!("Lexer error:\n{}", e))?;

    // Parsing
    let mut parser = koa::Parser::new(tokens);
    let ast = parser
        .parse()
        .map_err(|e| anyhow::anyhow!("Parser error:\n{}", e))?;

    // IR Lowering
    let mut lowerer = koa::ir::IrLowerer::new();
    let ir_program = lowerer
        .lower(&ast)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    // LLVM Gen
    let llvm_ir =
        koa::llvm_gen::compile_to_llvm(&ir_program).map_err(|e| anyhow::anyhow!("{:?}", e))?;

    // Write out LLVM IR
    let output_path = target_path.with_extension("ll");
    std::fs::write(&output_path, llvm_ir)?;

    println!(
        "{} LLVM IR generated at {}",
        "Finished".green().bold(),
        output_path.display()
    );

    // Compile LLVM IR to native executable
    let exe_path = if cfg!(target_os = "windows") {
        target_path.with_extension("exe")
    } else {
        target_path.with_extension("")
    };

    println!("{} Compiling to executable...", "Compiling".green().bold());

    let output = std::process::Command::new("clang")
        .arg("-o")
        .arg(&exe_path)
        .arg(&output_path)
        .arg("-lSystem")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute clang: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("clang compilation failed:\n{}", stderr);
    }

    println!(
        "{} Executable generated at {}",
        "Success".green().bold(),
        exe_path.display()
    );
    Ok(())
}

pub fn resolve_target(input: Option<&str>) -> Result<String> {
    if let Some(path) = input {
        return Ok(path.to_string());
    }

    // Check for Koa.toml (Project mode)
    if Path::new("Koa.toml").exists() {
        return Ok("Project (Koa.toml)".to_string());
    }

    // Check for src/main.koa (Implicit entry point)
    if Path::new("src/main.koa").exists() {
        return Ok("src/main.koa".to_string());
    }

    // Check for src/lib.koa (Library implicit entry point)
    if Path::new("src/lib.koa").exists() {
        return Ok("src/lib.koa".to_string());
    }

    anyhow::bail!("No input file specified, no Koa.toml, and no src/main.koa or src/lib.koa found.")
}
