//! UtilBuilder creates the output string that actually implements the util methods.
//!
//! This includes but is not limited to:
//! - database queries
//! - input/output sanitization
//! - packing/unpacking data into/from (user defined) structs
//! - etc.

use crate::readers::assembler::Endpoint;

/// Facade for the actual util builder.
///
/// This allows us to build many endpoint util strings while only
/// making a single UtilBuilder object, which greatly enhances the
/// overall level of abstraction that callers of this module face.
struct UtilEndpointBuilder {
    endpoint: Endpoint,
}

impl UtilEndpointBuilder {
    fn get_util_string_from_endpoint(endpoint_ref: &Endpoint) -> String {
        // setting up the util builder
        let endpoint = endpoint_ref.clone();
        let util_builder = UtilEndpointBuilder { endpoint };

        // output string-to-be
        let mut full_output_string = String::new();

        // response model struct handling
        full_output_string.push_str(&util_builder.method_return_struct_string());

        // method signature handling
        full_output_string.push_str(&util_builder.method_signature_string());

        // method body handling
        full_output_string.push_str(&util_builder.method_body_string());

        full_output_string
    }

    /// Returns a string with the method signature that matches the one used
    /// by the `body_writer` mod.
    fn method_signature_string(&self) -> String {
        // create the string of params (if none given, 0 len string)
        let mut param_string = String::new();
        if let Some(query) = &self.endpoint.query_param {
            param_string.push_str(&format!("{}: {}", query.name, query.field_type));
        }

        // final output string
        format!(
            "async fn {}({}) -> impl Responder {{\n",
            &self.endpoint.name, param_string
        )
    }

    /// Creates the method response model as per defined in the input TOML file.
    ///
    /// Assumes all types are valid and syntax checks out. If not, relies on post-writing
    /// compilation/autofmt check to catch the errors (weight on user not system).
    fn method_return_struct_string(&self) -> String {
        format!(
            "struct {} {}\n",
            &self.endpoint.return_model_name, &self.endpoint.return_model
        )
    }

    /// Actual method body implementation generator. Most of the work on the module is
    /// done here. Add features with caution.
    fn method_body_string(&self) -> String {
        String::new()
    }
}

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
    ///
    /// Under the hood, this method actually makes calls to the `UtilEndpointBuilder` struct,
    /// Allowing us to make a single util file for a vec of endpoints.
    pub fn get_util_method_string(&self, endpoints: &Vec<&Endpoint>) -> String {
        let mut final_output_string = String::new();
        for endpoint in endpoints {
            let endpoint_util_string = UtilEndpointBuilder::get_util_string_from_endpoint(endpoint);
            final_output_string.push_str(&endpoint_util_string)
        }
        final_output_string
    }

    /// Returns a string with the imports needed for the util file
    fn util_import_string(&self) -> String {
        let mut imports = String::new();
        // placeholder for now obviously
        imports
    }
}
