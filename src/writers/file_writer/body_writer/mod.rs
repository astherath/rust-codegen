//! Writer module that writes most of the generated code in the form of
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

/// Base struct for the implementations of a single
/// actix `GET` endpoint.
struct HTTPGetBodyWriter {
}


/// Note: none of these methods actually require an instanciation mainly because
/// the writers only really expose one method to the public: `write`.
impl HTTPGetBodyWriter {
    /// Top-level function for the struct, returns final output string.
    ///
    /// Based entirely off of an `Endpoint` reference, decomposing it
    /// into the final output string headed to file.
    pub fn write(endpoint: &Endpoint) -> String {
        String::from("lol")
    }

}


/// This trait is to be shared amongst all of the HTTP<verb>BodyWriters, and has
/// common util functions for them all so that unpacking calls work polymorphically
trait HTTPBodyWriter {
    /// Top-level function for the struct, returns final output string.
    ///
    /// Based entirely off of an `Endpoint` reference, decomposing it
    /// into the final output string headed to file.
    pub fn new(endpoint: &Endpoint) -> String;

    /// Returns the actix macro string for the specific HTTP<Verb> builder
    fn macro_string() -> String;

    /// Returns the actual method signature string for the actix method
    fn method_signature_string() -> String;

    /// Returns the body string of the actix method.
    ///
    /// This method can call many helper methods as long as the result
    /// returned is final and output-ready.
    fn method_body_string() -> String;
}
