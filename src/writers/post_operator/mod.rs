// use glob::glob;
use std::path::Path;
use std::process::Command;

pub fn format_all_rs(path: &Path) -> std::io::Result<()> {
    // Calls rustfmt on all files ending in .rs within path directory and all subdirectories
    Command::new("cargo")
        .args(&["fmt", "--manifest-path", path.join("Cargo.toml").to_str().unwrap()])
        .output()?;
    Ok(())
}
