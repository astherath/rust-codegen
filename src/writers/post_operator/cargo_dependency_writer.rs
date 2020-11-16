use std::fs::File;
use std::io::prelude::*;

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

pub fn write_cargo_toml_file(path_str: &String) -> std::io::Result<()> {
    let mut file = File::create(path_str)?;
    let cargo_deps = get_cargo_dependency_string();
    file.write_all(cargo_deps.as_bytes())?;

    Ok()
}
