use anyhow::Result;
use std::path::Path;
use colored::*;

pub fn execute(input: Option<&str>, mode: &str) -> Result<()> {
    let target = resolve_target(input)?;
    println!("{} Building {} in {} mode...", "Compiling".green().bold(), target, mode);
    
    // Scaffolding: In the future, this calls the compiler library
    // minimal simulation
    if !Path::new(&target).exists() && !target.starts_with("Project") {
         anyhow::bail!("Input file '{}' not found", target);
    }
    
    println!("{} Finished dev [unoptimized + debuginfo] target(s) in 0.5s", "Finished".green().bold());
    Ok(())
}

fn resolve_target(input: Option<&str>) -> Result<String> {
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
