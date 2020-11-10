//! This module has all of the actual (large) database string generator code,
//! if DB related code needs to be added/edited this is the first point of contact.

/// Generates a fully contained/functional database system using mongodb.
///
/// Main interface to be used in other generated methods is:
/// `DB::get_collection(db_name, collection)`.
pub fn get_database_setup_string(db_uri: &String) -> String {
    format!(
        "
            pub struct DB {{
                pub client: Client,
                }}

            impl DB {{
                pub async fn init() -> Result<Self> {{
                    let mut client_options = ClientOptions::parse(\"{}\").await?;
                    client_options.app_name = Some(\"TEST\".to_string());
                    Ok(Self {{
                        client: Client::with_options(client_options)?,
                    }})
                }}

                pub fn get_collection(&self, db_name: String, collection: String) -> Collection {{
                    self.client.database(db_name).collection(collection)
                }}

            }}
            ",
        db_uri
    )
}

/// Just returns the import header needed to cover all of the mongodb imports.
pub fn get_database_import_string() -> String {
    String::from(
        "\
        use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
        use mongodb::{options::ClientOptions, Client, Collection};
        ",
    )
}
