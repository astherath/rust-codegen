// use crate::readers::assembler::WebAPI;
use std::fs::{remove_dir_all, DirBuilder, File};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

/// Highest level function abstracts away the struct and all of its calls.
///
/// Should succeed at creating the entirety of the directory tree needed.
pub fn build(base_path_str: String) -> Result<()> {
    let directory_builder = DirectoryBuilder::new(base_path_str);
    directory_builder.create_base_dir()?;
    Ok(())
}

/// Singleton whose sole job is to succesfully build the directory tree for the
/// generated code to live in.
///
/// Guarantees success in creating the dir. tree so use
/// with caution (will destroy files residing in it's working folders).
///
/// Serves as the foundation for the generated code writer, and is meant to be
/// extendable by design.
pub struct DirectoryBuilder {
    dir_builder: DirBuilder,
    base_dir: PathBuf,
}

impl DirectoryBuilder {
    /// Constructor that takes in the root output directory where all of the
    /// generated code will reside.
    pub fn new(output_dir_str: String) -> DirectoryBuilder {
        let dir_builder = DirBuilder::new();
        let base_dir = PathBuf::from(output_dir_str);
        DirectoryBuilder {
            dir_builder,
            base_dir,
        }
    }

    /// Creates the base directory and nothing more.
    ///
    /// If the directory is already populated then
    /// cleans it and creates the directory again.
    pub fn create_base_dir(&self) -> Result<()> {
        // creating the base directory;
        // if an error occurs, hard wipes the directory and retries
        if let Err(_) = self.dir_builder.create(&self.base_dir) {
            remove_dir_all(&self.base_dir)?;
            self.dir_builder.create(&self.base_dir)?;
        }
        // -
        // -    let config_paths = ["database", "config"];
        // -    for path in &config_paths {
        // -        let mut new_path = base_path.clone();
        // -        new_path.push(path);
        // -        builder.create(&new_path)?;
        // -    }
        // -
        // -    let mut path_list = Vec::new();
        // -
        // -    {
        // -        let path_str = "database/mod.rs";
        // -        let mut new_path = base_path.clone();
        // -        new_path.push(path_str);
        // -        path_list.push(new_path.to_str().unwrap().to_string());
        // -        let db_uri = &String::from(
        // -            "mongodb://127.0.0.1:27017/?compressors=disabled&gssapiServiceName=mongodb",
        // -        );
        // -        create_database_config_file(&new_path, db_uri)?;
        // -    };
        // -
        // -    // path_list.push(new_path.to_str().unwrap().to_string());
        // -
        // -    Ok(path_list)

        Ok(())
    }
}
