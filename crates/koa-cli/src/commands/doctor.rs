use anyhow::Result;
use colored::*;
use std::env;
use std::process::Command;
use which::which;

pub fn check() -> Result<()> {
    println!("{}", "Koa Toolchain Health Check".bold().cyan());
    println!("{}", "=".repeat(30).cyan());
    println!();

    // System Information
    println!("{}", "System Information:".bold());
    println!("{:<20} {}", "OS:", env::consts::OS);
    println!("{:<20} {}", "Architecture:", env::consts::ARCH);
    if let Ok(rust_v) = get_version("rustc") {
        println!("{:<20} {}", "Rust Version:", rust_v);
    }
    if let Ok(cargo_v) = get_version("cargo") {
        println!("{:<20} {}", "Cargo Version:", cargo_v);
    }
    println!();

    // Toolchain Check
    println!("{}", "Required Dependencies:".bold());
    let steps = vec![
        ("LLVM", check_llvm()),
        ("Clang", check_tool_with_version("clang")),
        ("Linker", check_linker()),
        ("Git", check_tool_with_version("git")),
    ];

    let mut healthy = true;

    for (name, result) in steps {
        print!("{:<20}", name);
        match result {
            Ok(info) => {
                println!("{} {}", "✓".green().bold(), info);
            }
            Err(_) => {
                println!("{}", "✗ MISSING".red().bold());
                healthy = false;
            }
        }
    }

    // Optional Tools
    println!();
    println!("{}", "Optional Tools:".bold());
    let optional_steps = vec![
        ("LLDB", check_tool_with_version("lldb")),
        ("Make", check_tool_with_version("make")),
    ];

    for (name, result) in optional_steps {
        print!("{:<20}", name);
        match result {
            Ok(info) => {
                println!("{} {}", "✓".green().bold(), info);
            }
            Err(_) => {
                println!("{}", "-".yellow());
            }
        }
    }

    println!();
    if healthy {
        println!(
            "{}",
            "✔ Your system is ready for Koa development! 🐨"
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            "✘ Some tools are missing or outdated. Please check the requirements."
                .yellow()
                .bold()
        );
    }

    Ok(())
}

fn get_version(name: &str) -> Result<String> {
    let output = Command::new(name).arg("--version").output()?;
    let out = String::from_utf8(output.stdout)?;
    Ok(out.trim().to_string())
}

fn check_tool_with_version(name: &str) -> Result<String> {
    let path = which(name)?;
    let version = get_version(name).unwrap_or_else(|_| "unknown version".to_string());
    Ok(format!("{} ({})", version, path.to_string_lossy()))
}

fn check_linker() -> Result<String> {
    let tools = ["lld", "ld.lld", "ld64.lld", "wasm-ld", "ld"];
    for tool in tools {
        if let Ok(path) = which(tool) {
            let version = get_version(tool).unwrap_or_else(|_| "unknown version".to_string());
            return Ok(format!("{} ({})", version, path.to_string_lossy()));
        }
    }
    Err(anyhow::anyhow!("Linker not found"))
}

fn check_llvm() -> Result<String> {
    let tools = [
        "llvm-config",
        "llvm-config-18",
        "llvm-config-17",
        "llvm-config-16",
        "llvm-config-15",
    ];
    for tool in tools {
        if let Ok(path) = which(tool) {
            let output = Command::new(&path).arg("--version").output()?;
            let version = String::from_utf8(output.stdout)?.trim().to_string();
            return Ok(format!("LLVM v{} ({})", version, path.to_string_lossy()));
        }
    }
    Err(anyhow::anyhow!("LLVM not found"))
}
