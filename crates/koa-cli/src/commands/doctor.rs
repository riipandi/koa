use colored::*;
use anyhow::Result;
use std::process::Command;
use which::which;

pub fn check() -> Result<()> {
    println!("{}", "checking koa toolchain health...".bold());
    println!();

    let steps = vec![
        ("LLVM", check_lvm()),
        ("Clang", check_tool("clang")),
        ("Linker", check_linker()),
        ("Git", check_tool("git")),
    ];

    let mut healthy = true;

    for (name, result) in steps {
        print!("{:<20}", name);
        match result {
            Ok(path) => {
                println!("{} ({})", "OK".green().bold(), path);
            },
            Err(_) => {
                println!("{}", "MISSING".red().bold());
                healthy = false;
            }
        }
    }

    println!();
    if healthy {
        println!("{}", "Your system is ready for Koa development! 🐨".green());
    } else {
        println!("{}", "Some tools are missing. Please install them.".yellow());
    }

    Ok(())
}

fn check_tool(name: &str) -> Result<String> {
    let path = which(name)?;
    Ok(path.to_string_lossy().to_string())
}

fn check_linker() -> Result<String> {
    // Check for LLD or system linker
    let tools = ["lld", "ld.lld", "ld64.lld", "wasm-ld", "ld"];
    for tool in tools {
        if let Ok(path) = which(tool) {
             return Ok(path.to_string_lossy().to_string());
        }
    }
    Err(anyhow::anyhow!("Linker not found"))
}

fn check_lvm() -> Result<String> {
    // Check for llvm-config or similar to verify LLVM installation
    // Prioritize llvm-config-17, etc if needed. keeping it simple for now.
    let tools = ["llvm-config", "llvm-config-18", "llvm-config-17", "llvm-config-16"];
    for tool in tools {
        if let Ok(path) = which(tool) {
             let output = Command::new(&path).arg("--version").output()?;
             let version = String::from_utf8(output.stdout)?.trim().to_string();
             return Ok(format!("{} v{}", path.to_string_lossy(), version));
        }
    }
    Err(anyhow::anyhow!("LLVM not found"))
}
