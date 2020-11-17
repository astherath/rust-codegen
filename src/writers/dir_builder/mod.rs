// use crate::readers::assembler::WebAPI;
use std::fs::{remove_dir_all, DirBuilder};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Serves as a flag indicator for the (very limited) types of
/// sub-directories possible.
///
/// This helps a lot when placing the files in their respective directories (cleanly).
#[derive(Eq, PartialEq, Hash, Clone)]
pub enum SubDir {
    Util,
    Routes,
    Models,
    Other,
}

impl SubDir {
    /// Matches the incoming path string to an unwrappable enum
    fn from_path_str(path_str: &str) -> Self {
        match path_str {
            "routes" => Self::Routes,
            "util" => Self::Util,
            "models" => Self::Models,
            _ => Self::Other,
        }
    }

    pub fn as_path_str(&self) -> String {
        match self {
            Self::Routes => "routes",
            Self::Util => "util",
            Self::Models => "models",
            Self::Other => "",
        }
        .to_string()
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
    pub base_dir: PathBuf,
    pub sub_dirs: Vec<SubDir>,
    group_names: Vec<String>,
}

impl DirectoryBuilder {
    /// Constructor that takes in the root output directory where all of the
    /// generated code will reside.
    pub fn new(output_dir_str: &str, group_names: Vec<String>) -> DirectoryBuilder {
        // make the directory builder and allow recursive path building
        let mut dir_builder = DirBuilder::new();
        dir_builder.recursive(true);

        // run cargo new first
        Self::run_cargo_new(output_dir_str);

        // convert the path string to a path buf
        let base_dir = PathBuf::from(&format!("{}/src", output_dir_str));

        // sub directory vector to hold the enums
        let sub_dirs = Vec::new();

        DirectoryBuilder {
            dir_builder,
            base_dir,
            sub_dirs,
            group_names,
        }
    }

    /// Highest level function abstracts away the struct and all of its calls.
    ///
    /// Should succeed at creating the entirety of the directory tree needed.
    pub fn build(&mut self) -> Result<()> {
        self.create_base_dir()?;

        // assemble vec of sub-dirs
        let sub_dir_strs = ["routes", "util", "models"];
        let mut sub_dirs = Vec::new();
        for dir_str in &sub_dir_strs {
            sub_dirs.push(dir_str.to_string());
        }

        // make the sub-dir
        self.create_sub_directories(&sub_dirs)?;

        Ok(())
    }

    /// Creates the base directory and nothing more.
    ///
    /// If the directory is already populated then
    /// cleans it and creates the directory again.
    pub fn create_base_dir(&self) -> Result<()> {
        // creating the base directory;
        // if an error occurs, hard wipes the directory and retries
        if self.dir_builder.create(&self.base_dir).is_err() {
            remove_dir_all(&self.base_dir)?;
            self.dir_builder.create(&self.base_dir)?;
        }
        Ok(())
    }

    /// Creates all of the sub-directories within the `base_dir`.
    ///
    /// Works off of a vector of `SubDir` structs, so adding/removing
    /// subdirs to the overall file hierarchy isn't a nightmare.
    fn create_sub_directories(&mut self, sub_dirs: &[String]) -> Result<()> {
        let mut full_dir = self.base_dir.clone();

        // add the group name to the dir to be created for each group
        for group_name in &self.group_names {
            full_dir.push(&group_name);

            for sub_dir in sub_dirs {
                // add sub dir to path and create the dir, then remove it
                full_dir.push(&sub_dir);
                self.dir_builder.create(&full_dir)?;
                full_dir.pop();

                // add the path to the list of sub dirs for the parent dir_builder
                self.sub_dirs.push(SubDir::from_path_str(sub_dir));
            }

            full_dir.pop();
        }
        Ok(())
    }

    /// Runs `cargo new` for the path to be generated
    fn run_cargo_new(project_name: &str) {
        // check if the dir exists, if so, just return early
        if Path::new(project_name).exists() {
            return;
        }
        // make the directory first
        DirBuilder::new().create(project_name).unwrap();

        Command::new("cargo")
            .args(&["init", project_name])
            .output()
            .unwrap();
    }
}
