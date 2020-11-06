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
    pub fn parse_toml(toml_str: &String) -> WebAPI {
        let parsed_toml: WebAPI = toml::from_str(toml_str).unwrap();
        parsed_toml
    }
}
