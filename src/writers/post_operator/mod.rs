mod cargo_dependency_writer;
use std::path::Path;
use std::process::Command;

fn format_all_rs(path_string: &String) -> std::io::Result<()> {
    // Calls rustfmt on all files ending in .rs within path directory and all subdirectories
    let path = Path::new(path_string);
    Command::new("cargo")
        .args(&[
            "fmt",
            "--manifest-path",
            path.join("Cargo.toml").to_str().unwrap(),
        ])
        .output()?;
    Ok(())
}

// TODO: find a better name for this function
pub fn do_post_write_ops(base_path_str: &String) -> std::io::Result<()> {
    format_all_rs(base_path_str)?;
    cargo_dependency_writer::write_cargo_toml_file(base_path_str)?;

    Ok(())
}
