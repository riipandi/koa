use super::build;
use anyhow::Result;
use colored::*;

pub fn execute(input: Option<&str>) -> Result<()> {
    build::execute(input, "debug")?;

    println!("{} Running...", "Running".green().bold());
    println!("{}", "Hello, World! (simulated output)".dimmed());

    Ok(())
}
