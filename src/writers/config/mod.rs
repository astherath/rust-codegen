use crate::readers::assembler::WebAPI;
use std::fs::{remove_dir_all, DirBuilder, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Creates the boilerplate config files as well as the directories (but does not populate them).
// pub fn create_base_files(api_config: WebAPI) -> std::io::Result<()> {
pub fn create_base_files() -> std::io::Result<Vec<String>> {
    let output_dir_name = "generated_output";
    let builder = DirBuilder::new();
    let base_path = PathBuf::from(output_dir_name);
    if let Err(_) = builder.create(&base_path) {
        remove_dir_all(&base_path)?;
        builder.create(&base_path)?;
    }

    let config_paths = ["database", "config"];
    for path in &config_paths {
        let mut new_path = base_path.clone();
        new_path.push(path);
        builder.create(&new_path)?;
    }

    let mut path_list = Vec::new();

    {
        let path_str = "database/mod.rs";
        let mut new_path = base_path.clone();
        new_path.push(path_str);
        path_list.push(new_path.to_str().unwrap().to_string());
        let db_uri = &String::from(
            "mongodb://127.0.0.1:27017/?compressors=disabled&gssapiServiceName=mongodb",
        );
        create_database_config_file(&new_path, db_uri)?;
    };

    // path_list.push(new_path.to_str().unwrap().to_string());

    Ok(path_list)
}

fn write_database_config_from_base_path(new_path: &mut Path) {
    // path_str is the final destination for the file being written to
    let path_str = "config/mod.rs";
    new_path.push(path_str);
    create_general_config_file(&new_path)?;
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

fn create_general_config_file(path: &Path) -> std::io::Result<()> {
    let mut output_str = String::new();
    output_str.push_str(&format!(""));
    let mut file = File::create(path)?;
    file.write_all(output_str.as_bytes())?;
    Ok(())
}
