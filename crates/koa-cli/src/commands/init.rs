use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;

const README_TEMPLATE: &str = "# {name}\n\nA Koa programming language project.\n\n## Getting Started\n\n```bash\n# Run the project\nkoa run\n\n# Build the project\nkoa build\n\n# Build in release mode\nkoa build --mode release\n```\n\n## Project Structure\n\n```\n.\n├── Koa.toml      # Project configuration\n├── README.md     # This file\n└── src/\n    └── main.koa  # Entry point\n```\n";

const KOA_TOML_TEMPLATE: &str = "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\ntype = \"executable\"\nauthors = [\"{author}\"]\ndescription = \"{description}\"\nlicense = \"MIT\"\n\n[build]\ntarget = \"{target}\"\nmode = \"debug\"\n";

const MAIN_KOA_TEMPLATE: &str =
    "fn main(): i32 {\n    println(\"Hello, World!\");\n    return 0;\n}\n";

pub fn execute(dir: Option<&str>) -> Result<()> {
    let project_name = get_project_name(dir)?;
    let project_dir = Path::new(&project_name);

    if project_dir.exists() {
        anyhow::bail!(
            "{} Directory '{}' already exists",
            "Error:".red().bold(),
            project_name
        );
    }

    println!("{} {}", "Creating".bold().cyan(), project_name.cyan());

    let target = get_target_triple();
    let author = get_author();
    let description = "A Koa programming language project";

    fs::create_dir_all(project_dir.join("src"))
        .with_context(|| format!("Failed to create {}/src directory", project_name))?;

    let readme_content = README_TEMPLATE.replace("{name}", &project_name);

    fs::write(project_dir.join("README.md"), readme_content)
        .with_context(|| "Failed to create README.md")?;

    let toml_content = KOA_TOML_TEMPLATE
        .replace("{name}", &project_name)
        .replace("{author}", &author)
        .replace("{description}", description)
        .replace("{target}", &target);

    fs::write(project_dir.join("Koa.toml"), toml_content)
        .with_context(|| "Failed to create Koa.toml")?;

    fs::write(project_dir.join("src/main.koa"), MAIN_KOA_TEMPLATE)
        .with_context(|| "Failed to create src/main.koa")?;

    println!("{} {}", "✓ Created".green().bold(), "README.md".green());
    println!("{} {}", "✓ Created".green().bold(), "Koa.toml".green());
    println!("{} {}", "✓ Created".green().bold(), "src/main.koa".green());

    println!();
    println!(
        "{} {} `cd {}` and run `{} {}`!",
        "Next:".bold().cyan(),
        "Navigate to".cyan(),
        project_name.cyan(),
        "koa run".green(),
        "to get started".green()
    );

    Ok(())
}

fn get_project_name(dir: Option<&str>) -> Result<String> {
    match dir {
        Some(name) => {
            if name.is_empty() {
                anyhow::bail!("Project name cannot be empty");
            }
            Ok(name.to_string())
        }
        None => {
            let current_dir = std::env::current_dir()?;
            Ok(current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("koa_project")
                .to_string())
        }
    }
}

fn get_target_triple() -> String {
    format!(
        "{}-{}-{}",
        std::env::consts::ARCH,
        if std::env::consts::OS == "macos" {
            "apple"
        } else {
            std::env::consts::OS
        },
        std::env::consts::FAMILY
    )
}

fn get_author() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "Your Name".to_string())
}
