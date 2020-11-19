//! Writes a single functions - `main`. Has the code to run the async rocket function.

use crate::readers::assembler::EndpointGroup;

/// Interface for the main method string builder.
pub struct MainMethodBuilder {
    mounted_routes: Vec<String>,
}

impl MainMethodBuilder {
    pub fn new() -> Self {
        let mounted_routes = Vec::new();
        Self { mounted_routes }
    }

    /// Creates a mountable string with the group data and returns it
    pub fn mount_group(&mut self, group: &EndpointGroup) {
        let util_method_strings = group.get_util_method_names();
        let to_be_mounted_string = util_method_strings.join(", ");

        let mounted_route_string = format!(".mount(\"/\", routes![{}])", to_be_mounted_string);

        self.mounted_routes.push(mounted_route_string);
    }

    pub fn get_main_method_string(&self) -> String {
        [
            Self::get_main_method_import_and_header_string(),
            Self::method_signature_string(),
            self.method_body_string(),
        ]
        .join("\n")
    }

    /// Get all of the imports and other misc. headers
    fn get_main_method_import_and_header_string() -> String {
        [
            "#![feature(proc_macro_hygiene, decl_macro)]",
            "mod users;",
            "#[macro_use]",
            "extern crate rocket;",
        ]
        .join("\n")
    }

    /// Returns a string with the method signature of the main method
    fn method_signature_string() -> String {
        "fn main() {".to_string()
    }

    /// Main method body string
    fn method_body_string(&self) -> String {
        if self.mounted_routes.is_empty() {
            return "
                rocket::ignite()
                    .mount(\"/\", routes![])
                    .launch();
                }"
            .to_string();
        }

        let routes_to_mount_string = self.mounted_routes.join("\n");
        format!(
            "
            rocket::ignite(){}.launch();
            }}",
            routes_to_mount_string
        )
    }
}
