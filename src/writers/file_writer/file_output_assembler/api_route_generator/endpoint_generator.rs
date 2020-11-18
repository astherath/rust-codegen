//! StringBuilder module that writes most of the generated code in the form of
//! the actual actix endpoint methods.
//!
//! Relies on data from `Endpoint` structs.
//!
//! Should ONLY ever be written to a file AFTER the header string has been
//! written to it (else compile will fail).

use crate::readers::assembler::Endpoint;

pub struct HttpGet {
    endpoint: Endpoint,
}

impl RocketRouteBuilder for HttpGet {
    fn new(endpoint: &Endpoint) -> HttpGet {
        HttpGet {
            endpoint: endpoint.clone(),
        }
    }

    fn get_body_string_from_endpoint(&self) -> String {
        [
            self.macro_string(),
            self.method_signature_string(),
            self.method_body_string(),
        ]
        .join("\n")
    }

    fn macro_string(&self) -> String {
        let route = &self.endpoint.route;
        let query_string = {
            if let Some(query) = &self.endpoint.query_param {
                format!("?<{}>", query.name)
            } else {
                "".to_string()
            }
        };
        format!("#[get(\"{}{}\")]", route, &query_string)
    }

    fn method_signature_string(&self) -> String {
        // FIXME: this exact if let gets re-written like 6 times with
        //        slightly different return variations. Maybe employ a
        //        class that handles query manips. independently?
        //        in any case, this needs to be refactored because it smells.
        let query_string = {
            if let Some(query) = &self.endpoint.query_param {
                format!("{}: {}", query.name, "&RawStr")
            } else {
                "".to_string()
            }
        };
        format!(
            "pub fn {}({}) -> String {{",
            &self.endpoint.name,
            query_string // &self.endpoint.name, &self.endpoint.return_model_name
        )
    }

    fn method_body_string(&self) -> String {
        let param_string = {
            if let Some(query) = &self.endpoint.query_param {
                format!("{}.to_string()", query.name)
            } else {
                "".to_string()
            }
        };

        let util_method_call = format!(
            "let response = util::{}_util({});",
            &self.endpoint.name, param_string
        );
        let return_statement = "response}".to_string();

        [util_method_call, return_statement].join("\n")
    }
}

/// This trait is to be shared amongst all of the HTTP<verb>s, and has
/// common util functions for them all so that unpacking calls work polymorphically
pub trait RocketRouteBuilder {
    /// Dummy constructor for allowing trait usage
    fn new(endpoint_ref: &Endpoint) -> Self;

    /// Top-level function for the struct, returns final output string.
    ///
    /// Based entirely off of an `Endpoint` reference, decomposing it
    /// into the final output string headed to file.
    fn get_body_string_from_endpoint(&self) -> String;

    /// Returns the actix macro string for the specific HTTP<Verb> builder
    fn macro_string(&self) -> String;

    /// Returns the actual method signature string for the actix method
    fn method_signature_string(&self) -> String;

    /// Returns the body string of the actix method.
    ///
    /// This method can call many helper methods as long as the result
    /// returned is final and output-ready.
    fn method_body_string(&self) -> String;
}
