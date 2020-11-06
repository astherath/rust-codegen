//! ## Parser - User input cleaning and validation
//! Main input reader `mod` for the project. Reads a TOML file and parses it into a valid format for later handoff to
//! the `assembler` module.

use serde_derive::Deserialize;
use std::fs;
use toml;

/// Arbitrary file-reading util function.
/// Returns the file data as a single string (for use with the `toml::de::Deserializer`).
fn read_file_data(filename: &String) -> String {
    let contents = fs::read_to_string(filename).expect("Unable to read file");
    contents
}

/// Main top-level struct that handles all of the reading of the input file.
///
/// Also performs minimal sanitization, but mainly acts as the first
/// barrier of safety for bad data (TOML validation performed at this step).
pub struct InputFileReader {
    toml_data: TomlConfig,
}

impl InputFileReader {
    /// Substitute constructor (instanciation without a file wouldn't really make sense anyway)
    /// Validates that the file exists and has a valid extension (`.toml`).
    ///
    /// Reads the file data into a single `str` and uses it to
    /// populate the struct with a `toml::de::Deserializer`.
    pub fn from_file(filename: &String) -> InputFileReader {
        let file_data = read_file_data(filename);
        let toml_data = TomlConfig::parse_toml(&file_data);
        InputFileReader { toml_data }
    }
}

/// Serves as a facade for all of the `toml` reading/parsing.
#[derive(Deserialize)]
struct TomlConfig {
    title: String,
    version: String,
}
impl TomlConfig {
    /// Top-level function to return the data within the config struct.
    fn parse_toml(toml_str: &String) -> TomlConfig {
        let parsed_toml: TomlConfig = toml::from_str(toml_str).unwrap();
        parsed_toml
    }
}
