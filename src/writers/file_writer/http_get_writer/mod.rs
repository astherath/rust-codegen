//! Serves as the abstract string-building interface for
//! writing `arctix` GET endpoint code to file.
//!
//! Due to the nature of raw string manipulation/output building,
//! this code tries to hide as much of the actual interface it works with
//! in order to simplify the top-level calls that the `file_writer` mod makes.

use super::header_writer::HeaderBuilder;
use crate::readers::assembler::Endpoint;

/// Main output builder interface (HTTP added in front to avoid naming confusion)
///
/// Uses the exact same template for all HTTP Get endpoints and swaps in the
/// user-specific input vars. with `format!`.
pub struct HTTPGetEndpointBuilder {}

impl HTTPGetEndpointBuilder {
    pub fn new() -> HTTPGetEndpointBuilder {
        HTTPGetEndpointBuilder {}
    }

    /// Main assembling method that makes all of the calls required to
    /// fully assemble the output string using the user-passed data.
    ///
    /// Takes in an `Enpoint` struct reference, as it has all of the
    /// necessary info to split into the sub tasks.
    pub fn create_endpoint(&self, endpoint: &Endpoint) -> String {
        let mut full_endpoint_string = String::new();

        // start assembling the strings and pushing onto the final one
        let header_string = HeaderBuilder::get_header_string_from_endpoint(endpoint);
        full_endpoint_string.push_str(&header_string);

        full_endpoint_string
    }
}
