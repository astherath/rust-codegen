//! Writes a single functions - `main`. Has the code to run the async rocket function.

/// Only public interface for the `mod` that returns the
/// finished string output.
///
/// Behind the scenes instanciates and uses `MainMethodBuilder`
/// struct calls.
pub fn get_main_method_string() -> String {
    MainMethodBuilder::get_main_method_string()
}

/// Interface for the main method string builder.
struct MainMethodBuilder {}

impl MainMethodBuilder {
    fn get_main_method_string() -> String {
        // output string-to-be
        let mut full_output_string = String::new();

        // import header string handling
        full_output_string.push_str(&Self::get_main_method_import_and_header_string());

        // method signature handling
        full_output_string.push_str(&Self::method_signature_string());

        // method body handling
        full_output_string.push_str(&Self::method_body_string());

        full_output_string
    }

    /// Get all of the imports and other misc. headers
    fn get_main_method_import_and_header_string() -> String {
        "\
        #![feature(proc_macro_hygiene, decl_macro)]
        mod users;

        #[macro_use]
        extern crate rocket;
        "
        .to_string()
    }

    /// Returns a string with the method signature of the main method
    fn method_signature_string() -> String {
        "
        fn main() {
        "
        .to_string()
    }

    /// Main method body string
    fn method_body_string() -> String {
        "
        rocket::ignite().mount(\"/\", routes![users::routes::find_user_by_id]).launch();
        }
        "
        .to_string()
    }
}
