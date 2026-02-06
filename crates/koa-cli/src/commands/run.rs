use super::build;
use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn execute(input: Option<&str>) -> Result<()> {
    let target_path = build::resolve_target(input)?;
    let target_path = Path::new(&target_path);

    if !target_path.exists() {
        anyhow::bail!("Input file '{}' not found", target_path.display());
    }

    build::execute(input, "debug")?;

    let exe_path = if cfg!(target_os = "windows") {
        target_path.with_extension("exe")
    } else {
        target_path.with_extension("")
    };

    println!(
        "\n{} {}",
        "▸".cyan(),
        format!("Running {}", exe_path.display()).dimmed()
    );

    let output = std::process::Command::new(&exe_path)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", exe_path.display(), e))?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        anyhow::bail!("Program exited with status: {}", output.status);
    }

    println!(
        "{} {}",
        "✓".green(),
        format!("Program exited successfully").dimmed()
    );

    Ok(())
}
