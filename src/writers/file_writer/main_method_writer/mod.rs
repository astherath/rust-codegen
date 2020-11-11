//! Writes a single functions - `main`. Has the code to run the async actix function.

use crate::readers::assembler::Endpoint;

/// Interface for the main method string builder.
pub struct MainMethodBuilder {}

impl MainMethodBuilder {
    pub fn get_main_method_string() -> String {
        // output string-to-be
        let mut full_output_string = String::new();

        // import header string handling
        full_output_string.push_str(&Self::get_main_method_import_string());

        // method signature handling
        full_output_string.push_str(&Self::method_signature_string());

        // method body handling
        full_output_string.push_str(&Self::method_body_string());

        full_output_string
    }

    /// Get static method string for `main`
    fn get_main_method_import_string() -> String {
        format!(
            "
            use tokio;
            mod users;
            "
        )
    }

    /// Returns a string with the method signature that has the
    /// `tokio async` macro header.
    fn method_signature_string() -> String {
        format!(
            "
            #[tokio::main]
            async fn main() {{
            "
        )
    }

    /// Main method body string
    fn method_body_string() -> String {
        format!(
            "
                let db = users::utils::DB::init().await.unwrap();
                let col = db.get_collection();
                let user_id = String::from(\"123\");
                let user = users::utils::find_user_by_id_util(user_id, col).await;
                println!(\"{{:#?}}\", user);
            }}
            "
        )
    }
}
