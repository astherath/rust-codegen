//! Writes a single functions - `main`. Has the code to run the async actix function.

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
        full_output_string.push_str(&Self::get_main_method_import_string());

        // method signature handling
        full_output_string.push_str(&Self::method_signature_string());

        // method body handling
        full_output_string.push_str(&Self::method_body_string());

        full_output_string
    }

    /// Get static method string for `main`
    fn get_main_method_import_string() -> String {
        "
            use tokio;
            mod users;
            "
        .to_string()
    }

    /// Returns a string with the method signature that has the
    /// `tokio async` macro header.
    fn method_signature_string() -> String {
        "
            #[tokio::main]
            async fn main() {{
            "
        .to_string()
    }

    /// Main method body string
    fn method_body_string() -> String {
        "
            let db = users::util::DB::init().await.unwrap();
            let col = db.get_collection();
            let user_id = String::from(\"123\");
            let user = users::util::find_user_by_id_util(user_id, col).await;
            println!(\"{:#?}\", user);
            }}
            "
        .to_string()
    }
}
