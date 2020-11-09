//! Single source generator for the header string that will eventually go to
//! the generated file.
//!
//! Most info comes from the top level `WebAPI` struct data, but also relies on
//! `Endpoint` data for some things.
//!
//! Header string includes module declarations, docstrings, and imports.

use crate::readers::assembler::Endpoint;

/// Acts just as a placeholder facade for the header string operations.
///
/// Since no real data needs to be stored long term, it's easier to just have
/// the functions handle the pass-ins.
pub struct HeaderBuilder {}

impl HeaderBuilder {
    /// Top-level function for the struct that returns the final,
    /// assembled header string from endpoint data.
    pub fn get_header_string_from_endpoint(endpoint: &Endpoint) -> String {
        HeaderBuilder::get_import_string()
    }

    /// Handles the string literals that have the imports.
    ///
    /// If more imports need to be added/modified this should be the ONLY
    /// method in which they are handled.
    fn get_import_string() -> String {
        // TODO: this method might be better if it read data in from another source
        let parent_actix_import = "actix_web";
        let actix_imports = ["middleware", "web", "App", "HttpRequest", "HttpServer"];

        // assemble the final import string

        // this creates the "use foo::{" string, with space for the child imports to come
        let mut full_import_string = format!("use {}::{{", parent_actix_import);
        let child_imports = actix_imports.join(",");
        // asemble the full import string
        full_import_string.push_str(&format!("{}}};", child_imports));

        full_import_string
    }
}
