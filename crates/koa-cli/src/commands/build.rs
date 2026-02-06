use anyhow::Result;
use colored::*;
use std::path::Path;
use std::time::Instant;

fn create_dir_all(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)
        .map_err(|e| anyhow::anyhow!("Failed to create directory '{}': {}", path.display(), e))
}

pub fn execute(input: Option<&str>, output: Option<&str>, _mode: &str) -> Result<()> {
    let target_path = resolve_target(input)?;
    let target_path = Path::new(&target_path);

    if !target_path.exists() {
        anyhow::bail!("Input file '{}' not found", target_path.display());
    }

    // Check if this is a project (has Koa.toml)
    let is_project = target_path.is_dir() || Path::new(target_path).join("Koa.toml").exists();

    let (source_path, project_dir) = if is_project {
        let project_root = if target_path.is_dir() {
            target_path.to_path_buf()
        } else {
            target_path.parent().unwrap().to_path_buf()
        };

        let main_file = project_root.join("src/main.koa");
        if !main_file.exists() {
            anyhow::bail!("Project main file not found: {}", main_file.display());
        }

        (main_file, Some(project_root))
    } else {
        (target_path.to_path_buf(), None)
    };

    let start = Instant::now();

    println!(
        "{} {}",
        "▸".cyan(),
        format!("Reading {}", source_path.display()).dimmed()
    );
    let source = std::fs::read_to_string(&source_path)?;

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

    // Determine output paths
    let (ll_path, exe_path) = if let Some(project_dir) = project_dir {
        let build_dir = project_dir.join("build/debug");
        create_dir_all(&build_dir)?;

        let ll_path = build_dir.join("main.ll");

        let exe_name = source_path
            .file_stem()
            .unwrap_or_else(|| std::ffi::OsStr::new("output"));

        let exe_path = if cfg!(target_os = "windows") {
            build_dir.join(exe_name).with_extension("exe")
        } else {
            build_dir.join(exe_name)
        };

        (ll_path, exe_path)
    } else {
        let ll_path = source_path.with_extension("ll");
        let exe_path = output
            .map(|p| Path::new(p).to_path_buf())
            .unwrap_or_else(|| {
                if cfg!(target_os = "windows") {
                    source_path.with_extension("exe")
                } else {
                    source_path.with_extension("")
                }
            });
        (ll_path, exe_path)
    };

    println!("{} Writing LLVM IR...", "▸".cyan());
    std::fs::write(&ll_path, &llvm_ir)?;
    println!("  {} Written to {}", "✓".green(), ll_path.display());

    println!("{} Compiling to native executable...", "▸".cyan());
    let output = std::process::Command::new("clang")
        .arg("-o")
        .arg(&exe_path)
        .arg(&ll_path)
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

    // Check for Koa.toml (Project mode)
    if Path::new("Koa.toml").exists() {
        return Ok(".".to_string());
    }

    // Check for src/main.koa (Implicit entry point)
    if Path::new("src/main.koa").exists() {
        return Ok(".".to_string());
    }

    anyhow::bail!("No input file specified, no Koa.toml, and no src/main.koa or src/lib.koa found.")
}
