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

    // assemble vec of sub-dirs
    let sub_dir_strs = ["database", "config", "routes", "util", "models"];
    let mut sub_dirs = Vec::new();
    for dir_str in &sub_dir_strs {
        let dir_string = dir_str.to_string();
        sub_dirs.push(SubDir::new(dir_string));
    }

    // make the sub-dirs
    directory_builder.create_sub_directories(sub_dirs)?;
    Ok(())
}

/// Very basic sub-directory struct to make it easy and simple to add
/// sub-directories in the future.
struct SubDir {
    path_name: String,
}

impl SubDir {
    fn new(path_name: String) -> SubDir {
        SubDir { path_name }
    }
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
        Ok(())
    }

    /// Creates all of the sub-directories within the `base_dir`.
    ///
    /// Works off of a vector of `SubDir` structs, so adding/removing
    /// subdirs to the overall file hierarchy isn't a nightmare.
    fn create_sub_directories(&self, sub_dirs: Vec<SubDir>) -> Result<()> {
        let mut full_dir = self.base_dir.clone();
        for sub_dir in &sub_dirs {
            full_dir.push(&sub_dir.path_name);
            self.dir_builder.create(&full_dir);
            full_dir.pop();
        }
        Ok(())
    }
}
