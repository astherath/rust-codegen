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

impl ActixRouteBuilder for HttpGet {
    fn new(endpoint: &Endpoint) -> HttpGet {
        HttpGet {
            endpoint: endpoint.clone(),
        }
    }

    fn get_body_string_from_endpoint(&self) -> String {
        let mut full_output_string = String::new();

        // macro header handling
        full_output_string.push_str(&self.macro_string());

        // method signature handling
        full_output_string.push_str(&self.method_signature_string());

        // method body handling
        full_output_string.push_str(&self.method_body_string());

        full_output_string
    }

    fn macro_string(&self) -> String {
        let route = &self.endpoint.route;
        format!("#[get(\"{}\")]\n", route)
    }

    fn method_signature_string(&self) -> String {
        let mut output_string = String::new();

        let mut param_string = String::new();

        if let Some(query) = &self.endpoint.query_param {
            param_string.push_str(&format!("{}: {}, ", query.name, query.field_type));
        }

        let fn_name = &self.endpoint.name;
        format!(
            "async fn {}({}data: web::Data<util::DB>) -> impl Responder {{\n",
            fn_name, param_string
        )
    }

    fn method_body_string(&self) -> String {
        // setup and create the util method handler
        let mut param_string = String::new();
        if let Some(query) = &self.endpoint.query_param {
            param_string.push_str(&format!("{}, ", query.name));
        }
        let util_method = format!(
            "
            let collection = data.get_collection();
            let response = util::{}_util({}collection).await;",
            &self.endpoint.name, param_string
        );

        let http_response = String::from("HttpResponse::Ok().json(response)}");

        format!("{}\n{}", util_method, http_response)
    }
}

/// This trait is to be shared amongst all of the HTTP<verb>BodyStringBuilders, and has
/// common util functions for them all so that unpacking calls work polymorphically
pub trait ActixRouteBuilder {
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
