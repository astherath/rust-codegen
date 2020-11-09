//! UtilBuilder creates the output string that actually implements the util methods.
//!
//! This includes but is not limited to:
//! - database queries
//! - input/output sanitization
//! - packing/unpacking data into/from (user defined) structs
//! - etc.

use crate::readers::assembler::Endpoint;

/// Holds all of the ops (altough interface will only call one) for the util builder.
///
/// Currently only holds a single piece of data that will be share by all util
/// methods for the entire API, but obviously is made to be easily expanded upon.
pub struct UtilBuilder {
    database_uri: String,
}

impl UtilBuilder {
    /// Constructor takes in only the info that will be required for basically every
    /// single endpoint regardless of HTTP Verb/other info.
    ///
    /// Assumes that database is MongoDB (to be extended at some point)
    pub fn new(database_uri: String) -> UtilBuilder {
        UtilBuilder { database_uri }
    }

    /// Top level function for the builder; assembles and returns a single output-ready
    /// string with all of the imports, code, etc. that are needed for the util method.
    pub fn get_util_method_string(&self, endpoint: &Endpoint) -> String {
        String::from("util file io")
    }
}
