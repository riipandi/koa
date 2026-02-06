use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "dev".to_string());

    let timestamp = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%MZ"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let out_dir = env::var("OUT_DIR").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let version_string = format!("{} {}/{} ({} {})", version, os, arch, git_hash, timestamp);

    fs::write(
        format!("{}/version.rs", out_dir),
        format!(r#"pub const VERSION: &str = "{}";"#, version_string),
    )
    .unwrap();
}
