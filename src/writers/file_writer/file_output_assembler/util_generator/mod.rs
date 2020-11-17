//! UtilBuilder creates the output string that actually implements the util methods.
//!
//! This includes but is not limited to:
//! - database queries
//! - input/output sanitization
//! - packing/unpacking data into/from (user defined) structs
//! - etc.

use crate::readers::assembler::Endpoint;
pub mod database_generator;
mod single_endpoint_generator;

/// Holds all of the ops (altough interface will only call one) for the util builder.
///
/// Currently only holds a single piece of data that will be share by all util
/// methods for the entire API, but obviously is made to be easily expanded upon.
pub struct UtilBuilder {
    db_info: database_generator::DatabaseInfo,
}

impl UtilBuilder {
    /// Constructor takes in only the info that will be required for basically every
    /// single endpoint regardless of HTTP Verb/other info.
    ///
    /// Assumes that database is MongoDB (to be extended at some point)
    pub fn new(db_info: database_generator::DatabaseInfo) -> UtilBuilder {
        UtilBuilder { db_info }
    }

    /// Top level function for the builder; assembles and returns a single output-ready
    /// string with all of the imports, code, etc. that are needed for the util method.
    ///
    /// Under the hood, this method actually makes calls to the `UtilEndpointBuilder` struct,
    /// Allowing us to make a single util file for a vec of endpoints.
    pub fn get_util_method_string(&self, endpoints: &[&Endpoint]) -> String {
        let mut final_output_string = String::new();

        // add the file-wide import header string
        final_output_string.push_str(&self.util_import_string());

        // add the code for all of the endpoints (structs, methods, etc.)
        for endpoint in endpoints {
            let endpoint_util_string =
                single_endpoint_generator::UtilEndpointBuilder::get_util_string_from_endpoint(
                    endpoint,
                );
            final_output_string.push_str(&endpoint_util_string);
        }

        // string gen. for the database related code
        final_output_string.push_str(&self.mongodb_client_string());

        final_output_string
    }

    /// Returns a string with the imports needed for the util file.
    ///
    /// This also includes DB imports
    fn util_import_string(&self) -> String {
        let mut imports = String::new();

        imports.push_str(&"\nuse serde_derive::{{Deserialize, Serialize}};\n".to_string());

        // get database imports from the database_generator under the hood
        imports.push_str(&database_generator::get_database_import_string());

        imports
    }

    /// Returns string that holds the mongodb client connection and
    /// all of the other relevant DB related structs/implementations
    /// (only one instance per util file at most)
    fn mongodb_client_string(&self) -> String {
        // This actually just calls an internal module so as to not have to
        // deal/edit the DB string here (too messy/large)
        database_generator::get_database_setup_string(&self.db_info)
    }
}
