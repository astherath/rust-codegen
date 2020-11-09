//! StringBuilder module that writes most of the generated code in the form of
//! the actual actix endpoint methods.
//!
//! Relies on data from `Endpoint` structs.
//!
//! Should ONLY ever be written to a file AFTER the header string has been
//! written to it (else compile will fail).
//!
//! FIXME: The name is a bit misleading, in reality it just composes the
//! output file strings without writing them. Maybe a refactor is in the works?
//! Or maybe the module should actually write to IO.
//! Worst case we can name swap between this mod and `HTTPGetEndpointBuilder`.

use crate::readers::assembler::Endpoint;


pub struct HttpGet{endpoint: Endpoint}

impl BodyBuilder for HttpGet {
    fn new(endpoint: &Endpoint) -> HttpGet {
        HttpGet{endpoint: endpoint.clone()}
    }

    fn get_body_string_from_endpoint(&self) -> String {
        let mut full_output_string = String::new();

        let macro_header_string = self.macro_string();
        full_output_string.push_str(&macro_header_string);

        full_output_string
    }

    fn macro_string(&self) -> String {
        let route = &self.endpoint.route;
        let output_string = format!("#[get(\"{}\")]\n", route);
        output_string
    }

    fn method_signature_string(&self) -> String {String::new()}

    fn method_body_string(&self) -> String {String::new()}
}


/// This trait is to be shared amongst all of the HTTP<verb>BodyStringBuilders, and has
/// common util functions for them all so that unpacking calls work polymorphically
pub trait BodyBuilder {

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
