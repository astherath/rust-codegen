//! ## TOML specific parser, validator, and assembler
//! Uses the `toml` library to parse a string of file data into a
//! rust `struct`.
//!
//! Modifications to the main/sub structs result in direct changes to the parser,
//! and should only be done very deliberately.
//!
//! ### NOTE:
//! - All of the enums and structs used to parse *must* implement the `serde::Deserialize` trait.

use hyper::StatusCode;
use serde_derive::Deserialize;
use toml;

/// Provides all of the top-level unpacking (deserialization) from the toml file.
///
/// The struct also serves as the 1:1 schema of the input toml file, as well as all of
/// it's sub-structs and optionals.
///
/// Most of the important data is in the `groups` field, as those house
/// the low-level implementation details for the actual `Endpoints`.
#[derive(Deserialize, Debug)]
pub struct WebAPI {
    title: String,
    version: String,
    groups: Vec<EndpointGroup>,
}

/// Basic methods for the top-level struct. Basically all operations done by other classes
/// (minus `writer`) should be done though this interface instead of the member fields.
impl WebAPI {
    /// Top-level function to return the raw data as a parsed and valid
    /// `struct` as defined by the `WebAPI struct` itself.
    ///
    /// *NOTE*: the `toml::from_str` here is unwrapped, meaning that the
    /// error messages passed are not really that great or legible.
    pub fn parse_toml(toml_str: &String) -> WebAPI {
        let parsed_toml: WebAPI = toml::from_str(toml_str).unwrap();
        parsed_toml
    }
}

/// Holds the data for grouped endpoints working with the same logic.
///
/// i.e. all of the calls that handle user data should be placed in a single
/// `EndpointGroup` struct (`groups` in TOML).
///
/// Most of the useful data is in the `endpoints` field, as those are the most
/// granular and mapable struct types.
///
/// Mainly exists to conform easily to TOML structure.
#[derive(Deserialize, Debug)]
struct EndpointGroup {
    name: String,
    endpoints: Vec<Endpoint>,
}

/// Single-endpoint struct. Holds info such as type, route, input data, etc.
///
/// A single WebAPI is composed of many of these `Endpoint` instances nested within
/// a `EndpointGroup` struct.
///
/// This is the single struct that should have the most meaningful data specific to a single
/// transaction in the API as a whole.
#[derive(Deserialize, Debug)]
struct Endpoint {
    route: String,
    http_verb: HTTPVerbs,
    query_param: Option<QueryParam>,
    // TODO: eventually this success code needs to be validated and wrapped in a StatusCode enum.
    success_code: u16,
    return_model: String,
}

/// Made for requests (`GET`s) that need to take in a URL query string parameter.
///
/// The `field_type` is a non-exhaustive enum of valid data types which are to be
/// used later in conjunction with the `writer` mod.
#[derive(Deserialize, Debug)]
struct QueryParam {
    name: String,
    field_type: UnitTypes,
}

/// Very small (and frankly hacky) list of accepted data types (think primitives but worse).
#[derive(Deserialize, Debug)]
enum UnitTypes {
    String,
    U32,
    I32,
    U16,
    I16,
}

// TODO: replace this with a `std` lib enum of http verbs.
/// Very basic (and probably bad) implementation of the few accepted HTTP Verbs.
///
/// Eventually these will require their own valid implementation for use with
/// the `writer` mod.
#[derive(Deserialize, Debug)]
enum HTTPVerbs {
    Get,
    Post,
    Delete,
    Update,
}
