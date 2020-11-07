use crate::readers::assembler::WebAPI;
use std::fs::{remove_dir_all, DirBuilder, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Creates the boilerplate config files as well as the directories (but does not populate them).
// pub fn create_base_files(api_config: WebAPI) -> std::io::Result<()> {
pub fn create_base_files() -> std::io::Result<Vec<String>> {
    let output_dir_name = "generated_output";
    let builder = DirBuilder::new();
    let mut base_path = PathBuf::from(output_dir_name);
    if let Err(_) = builder.create(&base_path) {
        remove_dir_all(&base_path)?;
        builder.create(&base_path)?;
    }

    let config_paths = ["database", "config"];
    for path in &config_paths {
        base_path.push(path);
        builder.create(&base_path)?;
        base_path.pop();
    }

    let mut path_list = Vec::new();

    {
        let path = "database/mod.rs";
        base_path.push(path);
        path_list.push(base_path.to_str().unwrap().to_string());
        let db_uri = &String::from(
            "mongodb://127.0.0.1:27017/?compressors=disabled&gssapiServiceName=mongodb",
        );
        create_database_config_file(&base_path, db_uri).unwrap();
    };
    Ok(path_list)
}

fn create_database_config_file(path: &Path, db_uri: &String) -> std::io::Result<()> {
    let mut output_str = String::new();
    output_str.push_str(&format!(
        "\
            use mongodb::Client;

            pub struct DatabaseConfig {{
                pub db_client: Client,
            }}

            impl DatabaseConfig {{
                pub fn new(db_uri: &String) -> DatabaseConfig {{
                    let client = Client::with_uri_str(\"{}\");
                    DatabaseConfig {{db_client: client}}
                }}
            }}

            ",
        db_uri
    ));
    let mut file = File::create(path)?;
    file.write_all(output_str.as_bytes())?;
    Ok(())
}
