//! ## Parser - User input cleaning and validation
//! Main input reader `mod` for the project. Reads a TOML file and parses it into a valid format for later handoff to
//! the `writer` module.
//!
//! This module only actually reads the file data, all other operations are handled by the
//! `assembler` sibling module.

use super::assembler::WebAPI;
use std::fs;

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
    pub toml_data: WebAPI,
}

impl InputFileReader {
    /// Substitute constructor (instanciation without a file wouldn't really make sense anyway)
    /// Validates that the file exists and has a valid extension (`.toml`).
    ///
    /// Reads the file data into a single `str` and uses it to
    /// populate the struct with a `toml::de::Deserializer`.
    pub fn from_file(filename: &String) -> InputFileReader {
        let file_data = read_file_data(filename);
        let toml_data = WebAPI::parse_toml(&file_data);
        InputFileReader { toml_data }
    }

    /// Very temporary function to smoke check the incoming data by printing it out
    pub fn pretty_print_data(&self) {
        println!("{:#?}", self.toml_data);
    }
}
