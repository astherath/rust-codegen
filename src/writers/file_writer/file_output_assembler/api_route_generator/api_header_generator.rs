//! Single source generator for the header string that will eventually go to
//! the generated file.
//!
//! Most info comes from the top level `WebAPI` struct data, but also relies on
//! `Endpoint` data for some things.
//!
//! Header string includes module declarations, docstrings, and imports.

/// Acts just as a placeholder facade for the header string operations.
///
/// Since no real data needs to be stored long term, it's easier to just have
/// the functions handle the pass-ins.
pub struct HeaderBuilder {}

impl HeaderBuilder {
    /// Top-level function for the struct that returns the final,
    /// assembled header string from endpoint data.
    pub fn get_header_string() -> String {
        // [Self::get_import_string(), Self::get_misc_header_string()].connect("\n")
        [Self::get_misc_header_string()].join("\n")
    }

    /// Handles the string literals that have the imports.
    ///
    /// If more imports need to be added/modified this should be the ONLY
    /// method in which they are handled.
    fn get_import_string() -> String {
        // TODO: this method might be better if it read data in from another source
        let parent_import = "rocket";
        let child_imports_list = ["web", "get", "HttpResponse", "Responder"];

        // assemble the final import string
        // this creates the "use foo::{" string, with space for the child imports to come
        let mut full_import_string = format!("use {}::{{", parent_import);
        let child_imports = child_imports_list.join(",");
        // asemble the full import string
        full_import_string.push_str(&format!("{}}};\n", child_imports));

        // add remaining misc (i.e. non-actix) imports
        full_import_string.push_str(&String::from("use super::util;"));

        full_import_string
    }

    /// Basically anything that should be in the first few lines of
    /// the file, that isn't an import, is returned here.
    ///
    /// Includes: macro declarations, extern crates, etc.
    fn get_misc_header_string() -> String {
        [
            "#![feature(proc_macro_hygiene, decl_macro)]",
            "use super::util;",
        ]
        .join("\n")
    }
}
