//! ## Parser - User input cleaning and validation
//! Main input reader `mod` for the project. Reads a TOML file and parses it into a valid format for later handoff to
//! the `assembler` module.

use serde_derive::Deserialize;
// use hyper::StatusCode;
// use toml_config;
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
    toml_data: WebAPI,
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

/// Provides all of the top-level unpacking (deserialization) from the toml file.
///
/// The struct also serves as the 1:1 schema of the input toml file, as well as all of
/// it's sub-structs and optionals.
///
/// Most of the important data is in the `groups` field, as those house
/// the low-level implementation details for the actual `Endpoints`.
#[derive(Deserialize, Debug)]
struct WebAPI {
    title: String,
    version: String,
    groups: Vec<EndpointGroup>,
}

/// Holds the data for grouped endpoints working with the same logic.
/// Mainly exists to conform easily to TOML structure.
#[derive(Deserialize, Debug)]
struct EndpointGroup {
    name: String,
    endpoints: Vec<Endpoint>,
}

/// Single-endpoint struct. Holds info such as type, route, input data, etc.
/// A single WebAPI is composed of many `Endpoint` instances.
#[derive(Deserialize, Debug)]
struct Endpoint {
    route: String,
    http_verb: HTTPVerbs,
    query_param: Option<QueryParam>,
    success_code: u16,
}

#[derive(Deserialize, Debug)]
struct QueryParam {
    name: String,
    field_type: UnitTypes,
}

#[derive(Deserialize, Debug)]
enum UnitTypes {
    String,
    U32,
    I32,
    U16,
    I16,
}

/// Very basic (and probably bad) implementation of the few accepted HTTP Verbs.
/// TODO: replace this with a `std` lib enum of http verbs.
#[derive(Deserialize, Debug)]
enum HTTPVerbs {
    Get,
    Post,
    Delete,
    Update,
}

impl WebAPI {
    /// Top-level function to return the data within the config struct.
    fn parse_toml(toml_str: &String) -> WebAPI {
        let parsed_toml: WebAPI = toml::from_str(toml_str).unwrap();
        parsed_toml
    }
}
