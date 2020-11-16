// use glob::glob;
use std::path::Path;
use std::process::Command;

pub fn format_all_rs(path_string: &String) -> std::io::Result<()> {
    // Calls rustfmt on all files ending in .rs within path directory and all subdirectories
    let path = Path::new(path_string);
    Command::new("cargo")
        .args(&["fmt", "--manifest-path", path.join("Cargo.toml").to_str().unwrap()])
        .output()?;
    Ok(())
}
