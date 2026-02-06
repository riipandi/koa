use super::build;
use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn execute(input: Option<&str>) -> Result<()> {
    // First, resolve the target path
    let target_path = build::resolve_target(input)?;
    let target_path = Path::new(&target_path);

    if !target_path.exists() {
        anyhow::bail!("Input file '{}' not found", target_path.display());
    }

    // Build the project
    build::execute(input, "debug")?;

    // Determine the executable path
    let exe_path = if cfg!(target_os = "windows") {
        target_path.with_extension("exe")
    } else {
        target_path.with_extension("")
    };

    // Run the executable
    println!(
        "{} Running {}...",
        "Running".green().bold(),
        exe_path.display()
    );

    let output = std::process::Command::new(&exe_path)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", exe_path.display(), e))?;

    // Print stdout
    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // Print stderr
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // Return the exit code
    if output.status.success() {
        Ok(())
    } else {
        anyhow::bail!("Program exited with status: {}", output.status);
    }
}
