use anyhow::{Context, Result};
use colored::*;
use inquire::Text;
use std::fs;
use std::path::Path;

const README_TEMPLATE: &str = "# {name}\n\nA Koa programming language project.\n\n## Getting Started\n\n```bash\n# Run the project\nkoa run\n\n# Build the project\nkoa build\n\n# Build in release mode\nkoa build --mode release\n```\n\n## Project Structure\n\n```\n.\n├── Koa.toml      # Project configuration\n├── README.md     # This file\n└── src/\n    └── main.koa  # Entry point\n```\n";

const KOA_TOML_TEMPLATE: &str = "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\ntype = \"executable\"\nauthors = [\"{author}\"]\ndescription = \"{description}\"\nlicense = \"MIT\"\n\n[build]\ntarget = \"{target}\"\nmode = \"debug\"\n";

const MAIN_KOA_TEMPLATE: &str =
    "fn main(): i32 {\n    println(\"Hello, World!\");\n    return 0;\n}\n";

const GITIGNORE_CONTENT: &str = ".DS_Store\n.DS_Store?\nThumbs.db\nehthumbs.db\nDesktop.ini\n$RECYCLE.BIN/\n*.sqlite*\n*.sqlite3*\n*.db\n.cache/\n.temp/\n/build/\n/temp/\n";

pub fn execute(dir: Option<&str>) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let is_empty = is_dir_empty(&current_dir)?;

    let (project_name, project_dir, use_current_dir) = if dir.is_none() && is_empty {
        let name = prompt_project_name()?;
        (name.clone(), current_dir.clone(), true)
    } else {
        let name = get_project_name(dir)?;
        let dir_path = Path::new(&name);
        if dir_path.exists() {
            anyhow::bail!(
                "{} Directory '{}' already exists",
                "Error:".red().bold(),
                name
            );
        }
        let dir_buf = dir_path.to_path_buf();
        (name, dir_buf, false)
    };

    if !project_dir.exists() {
        println!("{} {}", "Creating".bold().cyan(), project_name.cyan());
    }

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

    fs::write(project_dir.join(".gitignore"), GITIGNORE_CONTENT)
        .with_context(|| "Failed to create .gitignore")?;

    println!("{} {}", "✓ Created".green().bold(), "README.md".green());
    println!("{} {}", "✓ Created".green().bold(), "Koa.toml".green());
    println!("{} {}", "✓ Created".green().bold(), "src/main.koa".green());
    println!("{} {}", "✓ Created".green().bold(), ".gitignore".green());

    if !use_current_dir {
        println!();
        println!(
            "{} {} `cd {}` and run `{} {}`!",
            "Next:".bold().cyan(),
            "Navigate to".cyan(),
            project_name.cyan(),
            "koa run".green(),
            "to get started".green()
        );
    } else {
        println!();
        println!(
            "{} {} `{} {}`!",
            "Next:".bold().cyan(),
            "Run".cyan(),
            "koa run".green(),
            "to get started".green()
        );
    }

    Ok(())
}

fn prompt_project_name() -> Result<String> {
    let current_dir_name = std::env::current_dir()?
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("koa_project")
        .to_string();

    let ans = Text::new("What is your project's name?")
        .with_default(&current_dir_name)
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(inquire::validator::Validation::Invalid(
                    "Project name cannot be empty".into(),
                ))
            } else if !input
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                Ok(inquire::validator::Validation::Invalid(
                    "Use only letters, numbers, underscores, and hyphens".into(),
                ))
            } else {
                Ok(inquire::validator::Validation::Valid)
            }
        })
        .prompt()?;

    Ok(ans)
}

fn is_dir_empty(path: &Path) -> Result<bool> {
    let entries = fs::read_dir(path)?;
    Ok(entries.count() == 0)
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
