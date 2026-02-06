use clap::{Parser, Subcommand};
use colored::*;
use std::process;

mod commands;

#[derive(Parser)]
#[command(name = "koa")]
#[command(about = "The Koa Programming Language Compiler & Toolchain", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project
    Build {
        /// Input file (optional if Koa.toml exists)
        input: Option<String>,
        /// Output binary path
        #[arg(short = 'o', long)]
        output: Option<String>,
        /// Build mode (debug, release)
        #[arg(long, default_value = "debug")]
        mode: String,
    },
    /// Run the project
    Run {
        /// Input file (optional if Koa.toml exists)
        input: Option<String>,
    },
    /// Check toolchain health
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Doctor => commands::doctor::check(),
        Commands::Build {
            input,
            output,
            mode,
        } => commands::build::execute(input.as_deref(), output.as_deref(), mode),
        Commands::Run { input } => commands::run::execute(input.as_deref(), None),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}
