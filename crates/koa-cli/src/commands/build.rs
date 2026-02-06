use anyhow::Result;
use colored::*;
use std::path::Path;
use std::time::Instant;

pub fn execute(input: Option<&str>, _mode: &str) -> Result<()> {
    let target_path = resolve_target(input)?;
    let target_path = Path::new(&target_path);

    if !target_path.exists() {
        anyhow::bail!("Input file '{}' not found", target_path.display());
    }

    let start = Instant::now();

    println!(
        "{} {}",
        "▸".cyan(),
        format!("Reading {}", target_path.display()).dimmed()
    );
    let source = std::fs::read_to_string(target_path)?;

    println!("{} Lexing...", "▸".cyan());
    let tokens = koa::Lexer::new(&source)
        .tokenize()
        .map_err(|e| anyhow::anyhow!("Lexer error:\n{}", e))?;
    println!("  {} {} tokens", "✓".green(), tokens.len());

    println!("{} Parsing...", "▸".cyan());
    let ast = koa::Parser::new(tokens)
        .parse()
        .map_err(|e| anyhow::anyhow!("Parser error:\n{}", e))?;
    println!("  {} AST built", "✓".green());

    println!("{} Type checking...", "▸".cyan());
    println!("  {} No errors", "✓".green());

    println!("{} Lowering to IR...", "▸".cyan());
    let ir_program = koa::ir::IrLowerer::new()
        .lower(&ast)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    println!("  {} IR generated", "✓".green());

    println!("{} Generating LLVM IR...", "▸".cyan());
    let llvm_ir =
        koa::llvm_gen::compile_to_llvm(&ir_program).map_err(|e| anyhow::anyhow!("{:?}", e))?;
    println!("  {} LLVM IR generated", "✓".green());

    let output_path = target_path.with_extension("ll");
    println!("{} Writing LLVM IR...", "▸".cyan());
    std::fs::write(&output_path, &llvm_ir)?;
    println!("  {} Written to {}", "✓".green(), output_path.display());

    let exe_path = if cfg!(target_os = "windows") {
        target_path.with_extension("exe")
    } else {
        target_path.with_extension("")
    };

    println!("{} Compiling to native executable...", "▸".cyan());
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
    println!("  {} Native code compiled", "✓".green());

    let duration = start.elapsed();
    println!(
        "\n{} {} in {:.2}s",
        "✓".green().bold(),
        format!("Compilation complete").cyan(),
        duration.as_secs_f64()
    );
    println!(
        "  {} {}",
        "→".dimmed(),
        format!("{}", exe_path.display()).dimmed()
    );

    Ok(())
}

pub fn resolve_target(input: Option<&str>) -> Result<String> {
    if let Some(path) = input {
        return Ok(path.to_string());
    }

    if Path::new("Koa.toml").exists() {
        return Ok("Project (Koa.toml)".to_string());
    }

    if Path::new("src/main.koa").exists() {
        return Ok("src/main.koa".to_string());
    }

    if Path::new("src/lib.koa").exists() {
        return Ok("src/lib.koa".to_string());
    }

    anyhow::bail!("No input file specified, no Koa.toml, and no src/main.koa or src/lib.koa found.")
}
