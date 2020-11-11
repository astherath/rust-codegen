//! Facade for the actual util builder, only handling a single endpoint at a time.

use crate::readers::assembler::Endpoint;

/// Main struct for the `mod` that handles all of the string building.
///
/// This allows us to build many endpoint util strings while only
/// making a single UtilBuilder object, which greatly enhances the
/// overall level of abstraction that callers of this module face.
pub struct UtilEndpointBuilder {
    endpoint: Endpoint,
}

impl UtilEndpointBuilder {
    /// Top level string builder method that returns finished string
    /// through a single call.
    ///
    /// Under the hood just assembling subcalls.
    ///
    /// If changes in order of the final output string are needed, or if
    /// a new component needs to be added, this is the method to edit first.
    pub fn get_util_string_from_endpoint(endpoint_ref: &Endpoint) -> String {
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
            param_string.push_str(&format!("{}: {}, ", query.name, query.field_type));
        }

        // final output string
        format!(
            "async fn {}({}collection: Collection) -> {} {{\n",
            &self.endpoint.name, param_string, &self.endpoint.return_model_name
        )
    }

    /// Creates the method response model as per defined in the input TOML file.
    ///
    /// Assumes all types are valid and syntax checks out. If not, relies on post-writing
    /// compilation/autofmt check to catch the errors (weight on user not system).
    fn method_return_struct_string(&self) -> String {
        format!(
            "\
            #[derive(Deserialize, Debug)]
            struct {} {}\n",
            &self.endpoint.return_model_name, &self.endpoint.return_model
        )
    }

    /// Actual method body implementation generator. Most of the work on the module is
    /// done here. Add features with caution.
    fn method_body_string(&self) -> String {
        // not all endpoints will have queries, so depending on that,
        // the actual generated mongo code will differ.

        let query_string = {
            if let Some(query) = &self.endpoint.query_param {
                format!("{}", &query.name)
            } else {
                format!("")
            }
        };

        format!(
            "\
            let query = doc! {{{}}};
            let document = collection.find_one(query, None).await.unwrap().unwrap();
            let response: {} = from_bson(Bson::Document(document)).unwrap();
            response
        }}",
            query_string, &self.endpoint.return_model_name
        )
    }
}
