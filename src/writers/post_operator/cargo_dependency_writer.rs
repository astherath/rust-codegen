use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

fn get_cargo_dependency_string() -> String {
    let dependencies = [
        "tokio = { version = \"0.2\", features = [\"macros\", \"rt-threaded\"] }",
        "warp = \"0.2\"",
        "serde = {version = \"1.0\", features = [\"derive\"] }",
        "serde_derive = \"1.0\"",
        "thiserror = \"1.0\"",
        "chrono = { version = \"0.4\", features = [\"serde\"] }",
        "futures = { version = \"0.3.4\", default-features = false, features = [\"async-await\"]}",
        "mongodb = \"1.0.0\"",
        "actix-web = \"3\"",
    ];

    let mut output_string = String::new();

    for dependency in &dependencies {
        output_string.push_str(&format!("{}\n", dependency));
    }

    output_string
}

pub fn write_cargo_toml_file(base_path_str: &String) -> std::io::Result<()> {
    let mut file_path = PathBuf::from(base_path_str);
    file_path.push(String::from("Cargo.toml"));

    if let Ok(val) = check_written_already(&file_path) {
        if val {
            return Ok(());
        }
    }

    // create file with options instead of fs::File for appending
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let cargo_deps = get_cargo_dependency_string();
    file.write_all(cargo_deps.as_bytes())?;

    // write the toml flag to file
    file.write_all(get_toml_written_string().as_bytes())?;

    Ok(())
}

fn check_written_already(path_str: &PathBuf) -> std::io::Result<bool> {
    let contents = std::fs::read_to_string(path_str).expect("Unable to read file");
    let parsed_toml: toml::Value = toml::from_str(&contents)?;

    // get the last bool value
    if let Some(other) = parsed_toml.get("other") {
        if let Some(written) = other.get("written") {
            if let Some(val) = written.as_bool() {
                return Ok(val);
            }
        }
    }
    return Ok(false);
}

fn get_toml_written_string() -> String {
    String::from("\n[other]\nwritten = true")
}
